/// Machine-mode cycle counter
/// Tracks the current timer value, incrementing at a constant rate determined by the hardware.
/// Contains the current cycle count or clock ticks since system start, incremented by the hardware clock.
/// The time register is read-only.
pub unsafe fn read() -> usize {
    let x: usize;
    core::arch::asm!("csrr {}, time", out(reg) x);
    x
}
