use volatile::Volatile;
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
    LightMagenta = 13,
    Yellow = 14,
    White = 15
}

#[derive(Debug,Clone,Copy,PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode{
        ColorCode((foreground as u8 ) << 4 | (background as u8))
        // Shifts the foreground color 4 bits to the left, then performs an 'or' operation between the
        // foreground and the background.
        // 0000xxxx = foreground
        // 0000yyyy = background
        // foreground << 4 = xxxx0000
        // foreground | background 
        // xxxx0000
        // 0000yyyy
        // xxxxyyyy
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar{
    ascii_code: u8, //Char Ascii code.
    color_code: ColorCode, // background/foreground color.
}

//VGA table consts
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer{
    chars:[[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer{
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    fn write_byte(&mut self, byte: u8){
        match byte {
            b'\n' => {
                self.new_line();
            }
            byte => {
                if self.column_position >= BUFFER_WIDTH{
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar{ascii_code: byte,
                color_code});
                self.column_position+=1;
            }
        }
    }
    fn write_string(&mut self, str: &str){
        for byte in str.bytes(){
            match byte {
                0x20..=0x7e => {
                    self.write_byte(byte)
                }
                _ => {
                    self.write_byte(0xfe)
                }
            }
        }
    }
    fn new_line(&mut self){
        
    }
}

pub fn print_test(){
    let mut writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
    };

    //writer.write_byte(b'a');
   writer.write_string("ello");
   writer.write_string("World!");
}

