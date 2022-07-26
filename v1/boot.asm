	global start

	section .text
	bits 32
start:
	;;  Point the first entry of the level 4 page table to the first entry in the p3 table
	mov eax, p3_table
	or eax, 0b11
	mov dword [p4_table + 0], eax

	;; Point the first entry of the level 3 page table to the first entry in the p2 table
	mov eax, p2_table
	or eax, 0b11
	mov dword [p3_table + 0], eax

	;; Point each page table level two entry to a page
	mov ecx, 0 										; counter variable
.map_p2_table:
	mov eax, 0x20000 							; 2 MB
	mul ecx
	or eax, 0b10000011
	mov [p2_table + ecx * 8], eax
	inc ecx
	cmp ecx, 512
	jne .map_p2_table

	;; Move page table to cr3
	mov eax, p4_table
	mov cr3, eax

	;; Enable Physical Address Extension
	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax

	;; Set the long mode bit
	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr

	;; Enable paging
	mov eax, cr0
	or eax, 1 << 31
	or eax, 1 << 16
	mov cr0, eax

	;; Start Up Text
	mov word [0xb8000], 0x0248  	; H
	mov word [0xb8002], 0x0265		; e
	mov word [0xb8004], 0x026c		; l
	mov word [0xb8006], 0x026c		; l
	mov word [0xb8008], 0x026f		; o
	mov word [0xb800a], 0x0221		; !

	section .bss
	align 4096
p4_table:
	resb 4096
p3_table:
	resb 4096
p2_table:
	resb 4096

	;; Add Minimum Global Descriptor Table to enter real 64 bit mode
	section .rodata
gdt64:
	dq 0 													; Zero entry
.code: equ $ - gdt64
	dq (1 << 44) | (1 << 47) | (1 << 41) | (1 << 43) | (1 << 53) ; Code Segment
.data: equ $ - gdt64
	dq (1 << 44) | (1 << 47) | (1 << 41) ; Data Segment
.pointer:
	dw .pointer - gdt64 - 1
	dq gdt64

	lgdt [gdt64.pointer]

	;; Updating selectors
	mov ax, gdt64.data
	mov ss, ax
	mov ds, ax
	mov es, ax

	;; Jump to long mode
	jmp gdt64.code:long_mode_start

	section .text
	bits 64
long_mode_start:
	mov rax, 0x2f592f412f4b2f4f
	mov qword [0xb8000], rax
	hlt
