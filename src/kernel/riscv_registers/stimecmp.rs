/// Supervisor Timer Comparison Register
/// Holds a compare value for the Supervisor timer.
/// When the stimecmp value matches or exceeds the stime (or time) value, a timer interrupt is triggered for Supervisor mode.
pub unsafe fn read() -> usize {
    let x: usize;
    core::arch::asm!("csrr {}, stimecmp", out(reg) x);
    x
}

/// Supervisor Timer Comparison Register
pub unsafe fn write(x: usize) {
    core::arch::asm!("csrw stimecmp, {}", in(reg) x);
}
