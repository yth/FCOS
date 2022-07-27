// Prevent compiler optimization from eliminating volatile memory access to the wrapped item
use volatile::Volatile;

// Enable statics that require code to be executed at runtime in order to be initialized
use lazy_static::lazy_static;

// Runtime free mutex
use spin::Mutex;

// String formatting
use core::fmt;


lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::GREEN, Color::BLACK),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}


const VGA_BUFFER_HEIGHT: usize = 25;
const VGA_BUFFER_WIDTH: usize = 80;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    BLACK      = 0b0000,
    BLUE       = 0b0001,
    GREEN      = 0b0010,
    CYAN       = 0b0011,
    RED        = 0b0100,
    MAGENTA    = 0b0101,
    BROWN      = 0b0110,
    LIGHTGRAY  = 0b0111,
    DARKGRAY   = 0b1000,
    LIGHTBLUE  = 0b1001,
    LIGHTGREEN = 0b1010,
    LIGHTCYAN  = 0b1011,
    LIGHTRED   = 0b1100,
    PINK       = 0b1101,
    YELLOW     = 0b1110,
    WHITE      = 0b1111,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}


#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT]
}


pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= VGA_BUFFER_WIDTH {
                    self.new_line();
                }

                let row = VGA_BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code
                });

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..VGA_BUFFER_HEIGHT {
            for col in 0..VGA_BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }

        self.clear_row(VGA_BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code
        };

        for col in 0..VGA_BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/* // Examples
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::GREEN, Color::BLACK),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    for i in 0..VGA_BUFFER_HEIGHT {
        writer.clear_row(i);
    }

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!\n");
    write!(writer, "The numbers are {}, {}, and {}", 42, 1.0/3.0, 3.14159142813);
}
*/

