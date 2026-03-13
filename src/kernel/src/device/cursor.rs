use crate::device::keyboard::{inb, outb};

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

pub fn update_cursor(x: usize, y: usize) {
	let pos: u16  = (y * crate::vga_buffer::vga_buffer::BUFFER_WIDTH + x) as u16;

	outb(0x3D4, 0x0F);
	outb(0x3D5, (pos & 0xFF) as u8);
	outb(0x3D4, 0x0E);
	outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
}

pub fn get_cursor_position() -> u16{

    let mut pos: u16 = 0;
    outb(0x3D4, 0x0F);
    pos |= inb(0x3D5) as u16;
    outb(0x3D4, 0x0E);
    pos |= (inb(0x3D5) as u16) << 8;
    pos
}
