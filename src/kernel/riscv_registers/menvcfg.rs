pub const SSTC: usize = 1usize << 63;

/// Machine Environment Configuration
/// controls settings that affect the behavior of the RISC-V environment. For instance:
///     Enable or configure specific architectural features.
///     Fine-tune security or operational aspects of the environment.
pub unsafe fn write(x: usize) {
    core::arch::asm!("csrw menvcfg, {}", in(reg) x);
}

pub unsafe fn read() -> usize {
    let x: usize;
    core::arch::asm!("csrr {}, menvcfg", out(reg) x);
    x
}
