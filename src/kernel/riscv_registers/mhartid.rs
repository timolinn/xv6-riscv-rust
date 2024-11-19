pub unsafe fn read() -> usize {
    let x: usize;
    core::arch::asm!("csrr {}, mhartid", out(reg) x);
    x
}
