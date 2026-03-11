pub struct Port {
    port: u16,
}

impl Port {
    pub const fn new(port: u16) -> Self {
        Port { port }
    }

    pub unsafe fn read(&self) -> u8 {
        let value: u8;
        core::arch::asm!(
            "in al, dx",
            in("dx") self.port,
            out("al") value,
            options(nostack)
        );
        value
    }

    pub unsafe fn write(&mut self, value: u8) {
        core::arch::asm!(
            "out dx, al",
            in("dx") self.port,
            in("al") value,
            options(nostack)
        );
    }
}
