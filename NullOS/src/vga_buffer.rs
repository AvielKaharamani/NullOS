use core::fmt;
use volatile::Volatile;

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    column_boundery: usize,
    row_boundery: usize,
}

impl Writer {
    pub fn set_boundery(&mut self) {
        self.column_boundery = self.column_position;
        self.row_boundery = self.row_position;
    }

    pub fn write_byte(&mut self, byte: u8) {
        use core::fmt::Write;
        match byte {
            b'\t' => {
                self.write_str("    ").unwrap();
            },
            b'\n' => {
                self.new_line();   
                
            },
            // Backspace keycode
            b'\x08' => {
                self.delete_char();
            }
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();   
                }
                let row = self.row_position;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer().chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
        self.update_cursor();
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { &mut *(0xb8000 as *mut Buffer) }
    }

    fn new_line(&mut self) {
        if self.row_position == BUFFER_HEIGHT-1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let buffer = self.buffer();
                    let character = buffer.chars[row][col].read();
                    buffer.chars[row-1][col].write(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT-1);
        }

        use core::cmp::min;
        self.row_position = min(self.row_position+1, BUFFER_HEIGHT-1);
        self.column_position = 0;

        self.update_cursor();
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b'\0',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[row][col].write(blank);
        }
    }

    fn update_cursor(&self) {
        use x86_64::instructions::port::Port;
        let pos = self.row_position * BUFFER_WIDTH + self.column_position;

        let mut controller_port = Port::new(0x3D4);
        let mut pos_port = Port::new(0x3D5);

        unsafe {
            controller_port.write(0x0F as u8);
            pos_port.write((pos & 0xFF) as u8);
            controller_port.write(0x0E as u8);
            pos_port.write(((pos >> 8) & 0xFF) as u8);
        }
    }

    pub fn delete_char(&mut self) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        let mut col = self.column_position;
        let mut row = self.row_position;
        if row == self.row_boundery && col == self.column_boundery {
            return;
        }
        if col == 0 {
            row -= 1;
            let mut end_col = 0;
            let space_ascii_value = 0x20;

            for i in 0..BUFFER_WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][BUFFER_WIDTH-i-1].read();
                if character.ascii_character != space_ascii_value {
                    end_col = BUFFER_WIDTH-i;
                    break;
                }
            }

            col = end_col;
        } else {
            col -= 1;
        }
        self.column_position = col;
        self.row_position = row;
        self.buffer().chars[row][col].write(blank);
        self.update_cursor();
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

pub static mut WRITER: Writer = Writer {
    column_position: 0,
    row_position: 0,
    color_code: ColorCode::new(Color::LightGray, Color::Black),
    column_boundery: 0,
    row_boundery: 0
};

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        #[allow(unused_unsafe)]
        let writer = unsafe { &mut $crate::vga_buffer::WRITER };
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

#[allow(unused_macros)]
macro_rules! set_boundery {
    ($($arg:tt)*) => ({
        #[allow(unused_unsafe)]
        let writer = unsafe { &mut $crate::vga_buffer::WRITER };
        writer.set_boundery();
    });
}

#[allow(unused_macros)]
macro_rules! hex_dump {
    ($value: expr) => {
       for (i, n) in $value.iter().enumerate() {
            print!("{:02x} ", *n);
            if (i+1) % 16 == 0 {
                print!("\n");
            } else if (i+1) % 8 == 0 {
                print!("  ");
            }
        }
        println!("");
    };
}

pub fn clear_row(row: usize) {
    unsafe {WRITER.clear_row(row);}
}

pub fn clear_screen() {
    unsafe {
        for row in 0..BUFFER_HEIGHT {
            WRITER.clear_row(row);
        }
        WRITER.column_position = 0;
        WRITER.row_position = 0;
    }
}