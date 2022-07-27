#![feature(lang_items)]
#![no_std]

mod vga_buffer;


use core::panic::PanicInfo;
use core::fmt::Write;

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kmain() -> ! {

    // If all the assembly are correctly set up, this will print OKAY
    unsafe {
        let vga = 0xb8000 as *mut u64;

        *vga = 0x2f592f412f4b2f4f;
    };

    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 13, 1.337).unwrap();

    loop {}
}

