/// Machine Interrupt Enable Register
pub const STIE: usize = 1 << 5;

pub unsafe fn read() -> usize {
    let x: usize;
    core::arch::asm!("csrr {}, mie", out(reg) x);
    x
}

pub unsafe fn write(x: usize) {
    core::arch::asm!("csrw mie, {}", in(reg) x);
}
