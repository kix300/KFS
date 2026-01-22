//! MOUSE MOD
//! GET input
//! NEED GDT & IDT

use core::sync::atomic::{AtomicU8, Ordering};

const PS2_DATA_PORT: u16 = 0x60;
const PS2_STATUS_PORT: u16 = 0x64;
const PS2_COMMAND_PORT: u16 = 0x64;

const PS2_CMD_ENABLE_SECOND_PORT: u8 = 0xA8;
const PS2_CMD_WRITE_TO_MOUSE: u8 = 0xD4;

const MOUSE_CMD_SET_DEFAULTS: u8 = 0xF6;
const MOUSE_CMD_ENABLE_PACKET_STREAMING: u8 = 0xF4;
const MOUSE_CMD_GET_DEVICE_ID: u8 = 0xF2;
const MOUSE_CMD_SET_SAMPLE_RATE: u8 = 0xF3;

static PACKET_STATE: AtomicU8 = AtomicU8::new(0);
static mut PACKET_BUFFER: [u8; 4] = [0; 4];

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
    left_button: bool,
    right_button: bool,
    middle_button: bool,
    x_position: i16,
    y_position: i16,
    has_wheel: bool,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            left_button: false,
            right_button: false,
            middle_button: false,
            x_position: 0,
            y_position: 0,
            has_wheel: false,
        }
    }
}

impl Mouse {
    pub fn init(&mut self) -> Result<(), &'static str> {
        Self::wait_write();
        outb(PS2_COMMAND_PORT, PS2_CMD_ENABLE_SECOND_PORT);

        Self::write_mouse(MOUSE_CMD_SET_DEFAULTS)?;
        Self::read_response()?;

        if Self::enable_wheel().is_ok() {
            self.has_wheel = true;
        }

        Self::write_mouse(MOUSE_CMD_ENABLE_PACKET_STREAMING)?;
        Self::read_response()?;

        Ok(())
    }

    fn enable_wheel() -> Result<(), &'static str> {
        Self::write_mouse(MOUSE_CMD_SET_SAMPLE_RATE)?;
        Self::read_response()?;
        Self::write_mouse(200)?;
        Self::read_response()?;

        Self::write_mouse(MOUSE_CMD_SET_SAMPLE_RATE)?;
        Self::read_response()?;
        Self::write_mouse(100)?;
        Self::read_response()?;

        Self::write_mouse(MOUSE_CMD_SET_SAMPLE_RATE)?;
        Self::read_response()?;
        Self::write_mouse(80)?;
        Self::read_response()?;

        Self::write_mouse(MOUSE_CMD_GET_DEVICE_ID)?;
        let device_id = Self::read_response()?;

        if device_id == 3 {
            Ok(())
        } else {
            Err("Wheel not supported")
        }
    }

    fn write_mouse(value: u8) -> Result<(), &'static str> {
        Self::wait_write();
        outb(PS2_COMMAND_PORT, PS2_CMD_WRITE_TO_MOUSE);
        Self::wait_write();
        outb(PS2_DATA_PORT, value);
        Ok(())
    }

    fn read_response() -> Result<u8, &'static str> {
        for _ in 0..1000 {
            if Self::can_read() {
                return Ok(inb(PS2_DATA_PORT));
            }
            Self::tiny_delay();
        }
        Err("Mouse read timeout")
    }

    fn wait_write() {
        for _ in 0..1000 {
            if (inb(PS2_STATUS_PORT) & 0x02) == 0 {
                return;
            }
            Self::tiny_delay();
        }
    }

    fn can_read() -> bool {
        let status = inb(PS2_STATUS_PORT);
        (status & 0x01) != 0 && (status & 0x20) != 0
    }

    fn tiny_delay() {
        for _ in 0..100 {
            unsafe { core::arch::asm!("nop") };
        }
    }

    pub fn handle_interrupt(&mut self) -> Option<MouseEvent> {
        if !Self::can_read() {
            return None;
        }

        let data = inb(PS2_DATA_PORT);
        let state = PACKET_STATE.load(Ordering::Relaxed);

        unsafe {
            PACKET_BUFFER[state as usize] = data;
        }

        let packet_size = if self.has_wheel { 4 } else { 3 };

        if state + 1 < packet_size {
            PACKET_STATE.store(state + 1, Ordering::Relaxed);
            return None;
        }

        PACKET_STATE.store(0, Ordering::Relaxed);

        unsafe {
            let flags = PACKET_BUFFER[0];
            let delta_x = PACKET_BUFFER[1] as i16;
            let delta_y = PACKET_BUFFER[2] as i16;

            if (flags & 0x08) == 0 {
                return None;
            }

            let delta_x = if (flags & 0x10) != 0 {
                delta_x | 0xFF00u16 as i16
            } else {
                delta_x
            };

            let delta_y = if (flags & 0x20) != 0 {
                delta_y | 0xFF00u16 as i16
            } else {
                delta_y
            };

            let delta_y = -delta_y;

            self.x_position = self.x_position.saturating_add(delta_x);
            self.y_position = self.y_position.saturating_add(delta_y);

            let left = (flags & 0x01) != 0;
            let right = (flags & 0x02) != 0;
            let middle = (flags & 0x04) != 0;

            if left && !self.left_button {
                self.left_button = true;
                return Some(MouseEvent::ButtonPressed(MouseButton::Left));
            } else if !left && self.left_button {
                self.left_button = false;
                return Some(MouseEvent::ButtonReleased(MouseButton::Left));
            }

            if right && !self.right_button {
                self.right_button = true;
                return Some(MouseEvent::ButtonPressed(MouseButton::Right));
            } else if !right && self.right_button {
                self.right_button = false;
                return Some(MouseEvent::ButtonReleased(MouseButton::Right));
            }

            if middle && !self.middle_button {
                self.middle_button = true;
                return Some(MouseEvent::ButtonPressed(MouseButton::Middle));
            } else if !middle && self.middle_button {
                self.middle_button = false;
                return Some(MouseEvent::ButtonReleased(MouseButton::Middle));
            }

            if self.has_wheel {
                let wheel_delta = PACKET_BUFFER[3] as i8;
                if wheel_delta > 0 {
                    return Some(MouseEvent::WheelUp);
                } else if wheel_delta < 0 {
                    return Some(MouseEvent::WheelDown);
                }
            }

            if delta_x != 0 || delta_y != 0 {
                return Some(MouseEvent::Move { delta_x, delta_y });
            }

            None
        }
    }

    pub fn position(&self) -> (i16, i16) {
        (self.x_position, self.y_position)
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.left_button,
            MouseButton::Right => self.right_button,
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
