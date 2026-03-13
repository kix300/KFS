// use crate::vga_buffer::WRITER;
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
        }
    }

    pub fn remove_buffer(&mut self) {
        if self.input_len > 0 {
            self.input_len -= 1;
            self.input_buf[self.input_len] = 0;
            crate::vga_buffer::WRITER.lock().delete_last_char();
        }
    }

    pub fn current_input(&self) -> &[u8] {
        &self.input_buf[..self.input_len]
    }
    pub fn execute(&mut self, cmd: &[u8]){
        match cmd {
            b"help" => println!("command: help"),
            // b"clear" => WRITER.lock().clear(),
            _ => println!("command unknow: {}", core::str::from_utf8(cmd).unwrap_or("?"))
        }
    } 
}
use spin::Mutex;
pub static TTY: Mutex<Tty> = Mutex::new(Tty::new());
