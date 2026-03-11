//! MOUSE MOD - PS/2 Mouse Driver
//! NEED GDT & IDT

const PS2_DATA_PORT: u16 = 0x60;
const PS2_STATUS_PORT: u16 = 0x64;
const PS2_COMMAND_PORT: u16 = 0x64;

const PS2_CMD_ENABLE_SECOND_PORT: u8 = 0xA8;
const PS2_CMD_WRITE_TO_MOUSE: u8 = 0xD4;
const PS2_CMD_READ_CONFIG: u8 = 0x20;
const PS2_CMD_WRITE_CONFIG: u8 = 0x60;

const MOUSE_CMD_SET_DEFAULTS: u8 = 0xF6;
const MOUSE_CMD_ENABLE_PACKET_STREAMING: u8 = 0xF4;
const MOUSE_CMD_GET_DEVICE_ID: u8 = 0xF2;
const MOUSE_CMD_SET_SAMPLE_RATE: u8 = 0xF3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, Copy)]
pub enum MouseEvent {
    Move { delta_x: i16, delta_y: i16 },
    ButtonPressed(MouseButton),
    ButtonReleased(MouseButton),
    WheelUp,
    WheelDown,
}
pub struct Mouse {
    left_button:   bool,
    right_button:  bool,
    middle_button: bool,
    x_position:    i16,
    y_position:    i16,
    has_wheel:     bool,
    packet_state:  u8,
    packet_buf:    [u8; 4],
}
impl Default for Mouse {
    fn default() -> Self {
        Self::new()
    }
}

impl Mouse {
    pub const fn new() -> Self {
        Mouse {
            left_button:   false,
            right_button:  false,
            middle_button: false,
            x_position:    0,
            y_position:    0,
            has_wheel:     false,
            packet_state:  0,
            packet_buf:    [0; 4],
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        Self::wait_write();
        outb(PS2_COMMAND_PORT, PS2_CMD_ENABLE_SECOND_PORT);

        Self::wait_write();
        outb(PS2_COMMAND_PORT, PS2_CMD_READ_CONFIG);
        let config = Self::read_data()?;
        Self::wait_write();
        outb(PS2_COMMAND_PORT, PS2_CMD_WRITE_CONFIG);
        Self::wait_write();
        outb(PS2_DATA_PORT, config | 0x02);

        Self::write_mouse(MOUSE_CMD_SET_DEFAULTS)?;
        Self::read_data()?;

        if Self::enable_wheel().is_ok() {
            self.has_wheel = true;
        }

        Self::write_mouse(MOUSE_CMD_ENABLE_PACKET_STREAMING)?;
        Self::read_data()?;

        Self::flush();

        Ok(())
    }

    fn enable_wheel() -> Result<(), &'static str> {
        Self::write_mouse(MOUSE_CMD_SET_SAMPLE_RATE)?; Self::read_data()?;
        Self::write_mouse(200)?;                       Self::read_data()?;
        Self::write_mouse(MOUSE_CMD_SET_SAMPLE_RATE)?; Self::read_data()?;
        Self::write_mouse(100)?;                       Self::read_data()?;
        Self::write_mouse(MOUSE_CMD_SET_SAMPLE_RATE)?; Self::read_data()?;
        Self::write_mouse(80)?;                        Self::read_data()?;

        Self::write_mouse(MOUSE_CMD_GET_DEVICE_ID)?;
        Self::read_data()?;
        let id = Self::read_data()?;
        if id == 3 { Ok(()) } else { Err("no wheel") }
    }

    fn write_mouse(value: u8) -> Result<(), &'static str> {
        Self::wait_write();
        outb(PS2_COMMAND_PORT, PS2_CMD_WRITE_TO_MOUSE);
        Self::wait_write();
        outb(PS2_DATA_PORT, value);
        Ok(())
    }

    fn read_data() -> Result<u8, &'static str> {
        for _ in 0..100_000 {
            if inb(PS2_STATUS_PORT) & 0x01 != 0 {
                return Ok(inb(PS2_DATA_PORT));
            }
        }
        Err("mouse timeout")
    }

    fn wait_write() {
        for _ in 0..100_000 {
            if inb(PS2_STATUS_PORT) & 0x02 == 0 {
                return;
            }
        }
    }

    fn flush() {
        for _ in 0..32 {
            if inb(PS2_STATUS_PORT) & 0x01 == 0 { break; }
            inb(PS2_DATA_PORT);
        }
    }

    pub fn process(&mut self, byte: u8) -> Option<MouseEvent> {
        let state = self.packet_state;

        if state == 0 && (byte & 0x08) == 0 {
            return None;
        }

        self.packet_buf[state as usize] = byte;
        let packet_size: u8 = if self.has_wheel { 4 } else { 3 };

        if state + 1 < packet_size {
            self.packet_state += 1;
            return None;
        }

        self.packet_state = 0;
        self.decode_packet()
    }

    fn decode_packet(&mut self) -> Option<MouseEvent> {
        let flags = self.packet_buf[0];
        let raw_x = self.packet_buf[1] as i16;
        let raw_y = self.packet_buf[2] as i16;

        // Overflow → paquet corrompu
        if (flags & 0xC0) != 0 {
            return None;
        }

        let delta_x = if flags & 0x10 != 0 { raw_x | !0xFF } else { raw_x };
        let delta_y = if flags & 0x20 != 0 { raw_y | !0xFF } else { raw_y };
        let delta_y = -delta_y;

        self.x_position = self.x_position.saturating_add(delta_x);
        self.y_position = self.y_position.saturating_add(delta_y);

        let left   = flags & 0x01 != 0;
        let right  = flags & 0x02 != 0;
        let middle = flags & 0x04 != 0;

        // Wheel en PREMIER (événement le plus "perdu" avant)
        if self.has_wheel {
            let raw = self.packet_buf[3];
            let wheel = if raw & 0x08 != 0 {
                (raw | 0xF0) as i8
            } else {
                (raw & 0x0F) as i8
            };
            if wheel > 0 { return Some(MouseEvent::WheelDown); }
            if wheel < 0 { return Some(MouseEvent::WheelUp); }
        }

        // Boutons
        if left != self.left_button {
            self.left_button = left;
            return Some(if left {
                MouseEvent::ButtonPressed(MouseButton::Left)
            } else {
                    MouseEvent::ButtonReleased(MouseButton::Left)
                });
        }
        if right != self.right_button {
            self.right_button = right;
            return Some(if right {
                MouseEvent::ButtonPressed(MouseButton::Right)
            } else {
                    MouseEvent::ButtonReleased(MouseButton::Right)
                });
        }
        if middle != self.middle_button {
            self.middle_button = middle;
            return Some(if middle {
                MouseEvent::ButtonPressed(MouseButton::Middle)
            } else {
                    MouseEvent::ButtonReleased(MouseButton::Middle)
                });
        }

        if delta_x != 0 || delta_y != 0 {
            return Some(MouseEvent::Move { delta_x, delta_y });
        }

        None
    }

    pub fn position(&self) -> (i16, i16) {
        (self.x_position, self.y_position)
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left   => self.left_button,
            MouseButton::Right  => self.right_button,
            MouseButton::Middle => self.middle_button,
        }
    }
}

pub fn inb(port: u16) -> u8 {
    unsafe {
        let result: u8;
        core::arch::asm!(
            "in al, dx",
            in("dx") port,
            out("al") result,
            options(nomem, nostack, preserves_flags)
        );
        result
    }
}

pub fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

use spin::Mutex;

pub static MOUSE: Mutex<Mouse> = Mutex::new(Mouse::new());
