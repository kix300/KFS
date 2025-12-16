#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        // Utilisation directe de l'instruction outl (out long/dword)
        // Port 0xf4 est le port isa-debug-exit de QEMU
        core::arch::asm!(
            "out dx, eax",
            in("dx") 0xf4u16,
            in("eax") exit_code as u32,
            options(nomem, nostack)
        );
    }
}
