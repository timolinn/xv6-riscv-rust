/// Thread pointer register
/// A general-purpose register reserved for thread-specific data,
/// vital for implementing thread-local storage in multithreaded environments
pub unsafe fn read() -> usize {
    let x: usize;
    core::arch::asm!("mv {}, tp", out(reg) x);
    x
}

pub unsafe fn write(x: usize) {
    core::arch::asm!("mv tp, {}", in(reg) x);
}
