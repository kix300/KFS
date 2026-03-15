use crate::println;

const CMD_MAX_LEN: usize = 256;
const CMD_HISTORY_SIZE: usize = 16;

pub struct Tty {
    input_buf:    [u8; CMD_MAX_LEN],
    input_len:    usize,

    history:      [[u8; CMD_MAX_LEN]; CMD_HISTORY_SIZE],
    history_lens: [usize; CMD_HISTORY_SIZE],
    history_len:  usize,
    history_idx:  Option<usize>,

    prompt_col:   usize,
}
impl Default for Tty {
    fn default() -> Self {
        Self::new()
    }
}

impl Tty {

    pub const fn new() -> Self {
        Tty {
            input_buf:    [0u8; CMD_MAX_LEN],
            input_len:    0,
            history:      [[0u8; CMD_MAX_LEN]; CMD_HISTORY_SIZE],
            history_lens: [0usize; CMD_HISTORY_SIZE],
            history_len:  0,
            history_idx:  None,
            prompt_col:   0,
        }
    }

    pub fn add_buffer(&mut self, c: u8) {
        if self.input_len < CMD_MAX_LEN {
            self.input_buf[self.input_len] = c;
            self.input_len += 1;
            crate::vga_buffer::WRITER.lock().write_byte(c);
            let (cx, cy) = crate::device::cursor::CURSOR.lock().get_cursor_position();
            crate::device::cursor::CURSOR.lock().update_cursor(cx+1, cy);
        }
    }
    pub fn get_len(&mut self) -> usize {
        self.input_len
    }

    pub fn remove_buffer(&mut self) {
        if self.input_len > 0 {
            self.input_len -= 1;
            self.input_buf[self.input_len] = 0;
            crate::vga_buffer::WRITER.lock().delete_last_char();
            let (cx, cy) = crate::device::cursor::CURSOR.lock().get_cursor_position();
            if cx > 0 {
                crate::device::cursor::CURSOR.lock().update_cursor(cx-1, cy);
            }
        }
    }

    pub fn clear(&mut self){
        self.input_buf = [0u8; CMD_MAX_LEN];
        self.input_len = 0;
    }
    pub fn execute(&mut self, cmd: &[u8]){
        match cmd {
            b"help" => crate::builtin::common_builtin::help(),
            b"miguel" => crate::builtin::common_builtin::miguel(),
            b"reboot" => crate::builtin::common_builtin::reboot(),
            b"clear" => crate::builtin::common_builtin::clear(),
            _ => println!("command unknow: {}", core::str::from_utf8(cmd).unwrap_or("?"))
        }
    } 
    pub fn tty(&mut self){
        let scancode = crate::device::keyboard::inb(0x60);
        // backspace
        if scancode == 0x0E{
            self.remove_buffer();
        }
        //right arrow key
        else if scancode == 0xcd {
            let (cx, cy) = crate::device::cursor::CURSOR.lock().get_cursor_position();
            if cx < self.get_len() as u16{
                crate::device::cursor::CURSOR.lock().update_cursor(cx+1, cy);
            }
        }
        //left arrow key
        else if scancode == 0xcb {
            let (cx, cy) = crate::device::cursor::CURSOR.lock().get_cursor_position();
            if cx > 0 {
                crate::device::cursor::CURSOR.lock().update_cursor(cx-1, cy);
            }
        }
        else if let Some(c) = crate::device::keyboard::KEYBOARD.lock().process(scancode) {
            if c != '\0' {
                // enter
                if scancode == 0x1c {
                    println!();
                    // println!("len buff tty: {}", self.get_len());
                    let cy = crate::device::cursor::CURSOR.lock().get_cursor_position().1;
                    crate::device::cursor::CURSOR.lock().update_cursor(0, cy);
                    let cmd_len = self.input_len;
                    let cmd_buf = self.input_buf;
                    self.execute(&cmd_buf[..cmd_len]);
                    self.clear();
                }else{
                    self.add_buffer(c as u8);
                }

            }
        }

    }
}
use spin::Mutex;
pub static TTY: Mutex<Tty> = Mutex::new(Tty::new());

