use crate::device::keyboard::{inb, outb};

pub struct Cursor{
    x: u16,
    y: u16,
    pos: u16,
}

impl Default for  Cursor{
    fn default() -> Self {
        Self::new()
    }
}


impl Cursor{

    pub const fn new() -> Self {
        Cursor{
            x: 0,
            y: 0,
            pos: 0,
        }
    }


    pub fn enable_cursor(cursor_start: u8, cursor_end: u8){
        outb(0x3D4, 0x0A);
        outb(0x3D5, (inb(0x3D5) & 0xC0) | cursor_start);

        outb(0x3D4, 0x0B);
        outb(0x3D5, (inb(0x3D5) & 0xE0) | cursor_end);
    }
    pub fn disable_cursor(){
        outb(0x3D4, 0x0A);
        outb(0x3D5, 0x20);
    }

    pub fn update_cursor(&mut self, x: u16, y: u16) {
        self.pos = y * crate::vga_buffer::vga_buffer::BUFFER_WIDTH as u16 + x;

        outb(0x3D4, 0x0F);
        outb(0x3D5, (self.pos & 0xFF) as u8);
        outb(0x3D4, 0x0E);
        outb(0x3D5, ((self.pos >> 8) & 0xFF) as u8);
    }

    pub fn get_cursor_position(&mut self) -> (u16, u16){

        outb(0x3D4, 0x0F);
        self.pos |= inb(0x3D5) as u16;
        outb(0x3D4, 0x0E);
        self.pos |= (inb(0x3D5) as u16) << 8;
        self.x = self.pos % 80;
        self.y = self.pos / 80;
        (self.x, self.y)
    }
}

use spin::Mutex;
pub static CURSOR: Mutex<Cursor> = Mutex::new(Cursor::new());
