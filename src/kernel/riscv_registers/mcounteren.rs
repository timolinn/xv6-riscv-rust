/// Machine mode counter enable register
pub unsafe fn write(x: usize) {
    core::arch::asm!("csrw mcounteren, {}", in(reg) x);
}

/// Read the value of the mcounteren register
pub unsafe fn read() -> usize {
    let x: usize;
    core::arch::asm!("csrr {}, mcounteren", out(reg) x);
    x
}
