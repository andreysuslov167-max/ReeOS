use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::port::Port;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
pub fn set_color(fg: Color, bg: Color) {
    WRITER.lock().color_code = ColorCode::new(fg, bg);
}
pub fn get_color() -> (Color, Color) {
    let writer = WRITER.lock();
    let fg = match writer.color_code.0 & 0x0F {
        0 => Color::Black,
        1 => Color::Blue,
        2 => Color::Green,
        3 => Color::Cyan,
        4 => Color::Red,
        5 => Color::Magenta,
        6 => Color::Brown,
        7 => Color::LightGray,
        8 => Color::DarkGray,
        9 => Color::LightBlue,
        10 => Color::LightGreen,
        11 => Color::LightCyan,
        12 => Color::LightRed,
        13 => Color::Pink,
        14 => Color::Yellow,
        15 => Color::White,
        _ => Color::White,
    };
    let bg = match (writer.color_code.0 >> 4) & 0x0F {
        0 => Color::Black,
        1 => Color::Blue,
        2 => Color::Green,
        3 => Color::Cyan,
        4 => Color::Red,
        5 => Color::Magenta,
        6 => Color::Brown,
        7 => Color::LightGray,
        _ => Color::Black,
    };
    (fg, bg)
}

pub fn reset_color() {
    WRITER.lock().color_code = ColorCode::new(Color::White, Color::Black);
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

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}
pub fn wait_for_enter() {
    let mut port = Port::new(0x60);
    loop {
        let scancode: u8 = unsafe { port.read() };
        if scancode == 0x1C {  
            break;
        }
    }
}

fn scancode_to_ascii(scancode: u8) -> Option<char> {
    match scancode {
        0x02 => Some('1'), 0x03 => Some('2'), 0x04 => Some('3'),
        0x05 => Some('4'), 0x06 => Some('5'), 0x07 => Some('6'),
        0x08 => Some('7'), 0x09 => Some('8'), 0x0A => Some('9'),
        0x0B => Some('0'), 0x10 => Some('q'), 0x11 => Some('w'),
        0x12 => Some('e'), 0x13 => Some('r'), 0x14 => Some('t'),
        0x15 => Some('y'), 0x16 => Some('u'), 0x17 => Some('i'),
        0x18 => Some('o'), 0x19 => Some('p'), 0x1E => Some('a'),
        0x1F => Some('s'), 0x20 => Some('d'), 0x21 => Some('f'),
        0x22 => Some('g'), 0x23 => Some('h'), 0x24 => Some('j'),
        0x25 => Some('k'), 0x26 => Some('l'), 0x2C => Some('z'),
        0x2D => Some('x'), 0x2E => Some('c'), 0x2F => Some('v'),
        0x30 => Some('b'), 0x31 => Some('n'), 0x32 => Some('m'),
        0x0C => Some('-'), 0x0D => Some('='), 0x35 => Some('/'),
        0x34 => Some('.'), 0x28 => Some('\''),
                

        _ => None,
    }
}
pub fn read_line() -> [u8; 256] {
    let mut port = Port::new(0x60);
    let mut buffer = [0u8; 256];
    let mut pos = 0;
    
    loop {
        let scancode: u8 = unsafe { port.read() };
        
        if scancode & 0x80 != 0 {
            continue;
        }
        
        match scancode {
            0x1C => break, 
            0x0E => { 
                if pos > 0 {
                    pos -= 1;
                }
            }
            _ => {
                if let Some(c) = scancode_to_ascii(scancode) {
                    if pos < 255 {
                        buffer[pos] = c as u8;
                        pos += 1;
                    }
                }
            }
        }
    }
    
   
    for i in 0..pos {
        WRITER.lock().write_byte(buffer[i]);
    }
    WRITER.lock().write_byte(b'\n');
    
    buffer
}
pub fn clear_keyboard_buffer() {
    use x86_64::instructions::port::Port;
    let mut port: Port<u8> = Port::new(0x60);
    let mut status_port: Port<u8> = Port::new(0x64);
    
  
    for _ in 0..100 {
        let status: u8 = unsafe { status_port.read() };
        if status & 1 != 0 {
            unsafe { port.read(); }
        } else {
            break;
        }
        for _ in 0..1000 {
            x86_64::instructions::nop();
        }
    }
}

pub fn read_line_with_echo() -> [u8; 256] {
    use x86_64::instructions::port::Port;
    let mut port: Port<u8> = Port::new(0x60);
    let mut buffer = [0u8; 256];
    let mut pos = 0;
    
   
    clear_keyboard_buffer();
    
    loop {
      
        let scancode: u8 = loop {
            let status: u8 = unsafe { Port::new(0x64).read() };
            if status & 1 != 0 {
                let code = unsafe { port.read() };
                
                if code & 0x80 == 0 {
                    break code;
                }
            }
           
            for _ in 0..1000 {
                x86_64::instructions::nop();
            }
        };
        
        match scancode {
            0x1C => {
                WRITER.lock().write_byte(b'\n');
                break;
            }
            0x0E => { 
                if pos > 0 {
                    pos -= 1;
                    let mut writer = WRITER.lock();
                    writer.write_byte(0x08);
                    writer.write_byte(b' ');
                    writer.write_byte(0x08);
                }
            }
            0x39 => { // Space
                if pos < 255 {
                    buffer[pos] = b' ';
                    WRITER.lock().write_byte(b' ');
                    pos += 1;
                }
            }
         
            0x02 => if pos < 255 { buffer[pos] = b'1'; WRITER.lock().write_byte(b'1'); pos += 1; }
            0x03 => if pos < 255 { buffer[pos] = b'2'; WRITER.lock().write_byte(b'2'); pos += 1; }
            0x04 => if pos < 255 { buffer[pos] = b'3'; WRITER.lock().write_byte(b'3'); pos += 1; }
            0x05 => if pos < 255 { buffer[pos] = b'4'; WRITER.lock().write_byte(b'4'); pos += 1; }
            0x06 => if pos < 255 { buffer[pos] = b'5'; WRITER.lock().write_byte(b'5'); pos += 1; }
            0x07 => if pos < 255 { buffer[pos] = b'6'; WRITER.lock().write_byte(b'6'); pos += 1; }
            0x08 => if pos < 255 { buffer[pos] = b'7'; WRITER.lock().write_byte(b'7'); pos += 1; }
            0x09 => if pos < 255 { buffer[pos] = b'8'; WRITER.lock().write_byte(b'8'); pos += 1; }
            0x0A => if pos < 255 { buffer[pos] = b'9'; WRITER.lock().write_byte(b'9'); pos += 1; }
            0x0B => if pos < 255 { buffer[pos] = b'0'; WRITER.lock().write_byte(b'0'); pos += 1; }            
            0x10 => if pos < 255 { buffer[pos] = b'q'; WRITER.lock().write_byte(b'q'); pos += 1; }
            0x11 => if pos < 255 { buffer[pos] = b'w'; WRITER.lock().write_byte(b'w'); pos += 1; }
            0x12 => if pos < 255 { buffer[pos] = b'e'; WRITER.lock().write_byte(b'e'); pos += 1; }
            0x13 => if pos < 255 { buffer[pos] = b'r'; WRITER.lock().write_byte(b'r'); pos += 1; }
            0x14 => if pos < 255 { buffer[pos] = b't'; WRITER.lock().write_byte(b't'); pos += 1; }
            0x15 => if pos < 255 { buffer[pos] = b'y'; WRITER.lock().write_byte(b'y'); pos += 1; }
            0x16 => if pos < 255 { buffer[pos] = b'u'; WRITER.lock().write_byte(b'u'); pos += 1; }
            0x17 => if pos < 255 { buffer[pos] = b'i'; WRITER.lock().write_byte(b'i'); pos += 1; }
            0x18 => if pos < 255 { buffer[pos] = b'o'; WRITER.lock().write_byte(b'o'); pos += 1; }
            0x19 => if pos < 255 { buffer[pos] = b'p'; WRITER.lock().write_byte(b'p'); pos += 1; }
            0x1E => if pos < 255 { buffer[pos] = b'a'; WRITER.lock().write_byte(b'a'); pos += 1; }
            0x1F => if pos < 255 { buffer[pos] = b's'; WRITER.lock().write_byte(b's'); pos += 1; }
            0x20 => if pos < 255 { buffer[pos] = b'd'; WRITER.lock().write_byte(b'd'); pos += 1; }
            0x21 => if pos < 255 { buffer[pos] = b'f'; WRITER.lock().write_byte(b'f'); pos += 1; }
            0x22 => if pos < 255 { buffer[pos] = b'g'; WRITER.lock().write_byte(b'g'); pos += 1; }
            0x23 => if pos < 255 { buffer[pos] = b'h'; WRITER.lock().write_byte(b'h'); pos += 1; }
            0x24 => if pos < 255 { buffer[pos] = b'j'; WRITER.lock().write_byte(b'j'); pos += 1; }
            0x25 => if pos < 255 { buffer[pos] = b'k'; WRITER.lock().write_byte(b'k'); pos += 1; }
            0x26 => if pos < 255 { buffer[pos] = b'l'; WRITER.lock().write_byte(b'l'); pos += 1; }
            0x2C => if pos < 255 { buffer[pos] = b'z'; WRITER.lock().write_byte(b'z'); pos += 1; }
            0x2D => if pos < 255 { buffer[pos] = b'x'; WRITER.lock().write_byte(b'x'); pos += 1; }
            0x2E => if pos < 255 { buffer[pos] = b'c'; WRITER.lock().write_byte(b'c'); pos += 1; }
            0x2F => if pos < 255 { buffer[pos] = b'v'; WRITER.lock().write_byte(b'v'); pos += 1; }
            0x30 => if pos < 255 { buffer[pos] = b'b'; WRITER.lock().write_byte(b'b'); pos += 1; }
            0x31 => if pos < 255 { buffer[pos] = b'n'; WRITER.lock().write_byte(b'n'); pos += 1; }
            0x32 => if pos < 255 { buffer[pos] = b'm'; WRITER.lock().write_byte(b'm'); pos += 1; }
            0x0C => if pos < 255 { buffer[pos] = b'-'; WRITER.lock().write_byte(b'-'); pos += 1; }
            0x0D => if pos < 255 { buffer[pos] = b'='; WRITER.lock().write_byte(b'='); pos += 1; }
            0x35 => { if pos < 255 { buffer[pos] = b'/'; WRITER.lock().write_byte(b'/'); pos += 1; } }
            0x34 => { if pos < 255 { buffer[pos] = b'.'; WRITER.lock().write_byte(b'.'); pos += 1; } }
            0x28 => { if pos < 255 { buffer[pos] = b'\''; WRITER.lock().write_byte(b'\''); pos += 1; } }
            _ => {}
        }
        
      
        for _ in 0..50000 {
            x86_64::instructions::nop();
        }
    }
    
    buffer
}
pub fn test_write() {
    WRITER.lock().write_byte(b'a');
    WRITER.lock().write_byte(b'b'); 
    WRITER.lock().write_byte(b'c');
    WRITER.lock().write_byte(b'\n');
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}