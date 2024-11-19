/// Machine Interrupt Delegation Register
/// mideleg register holds the machine interrupt delegation register
/// it is used to delegate the handling of certain interrupts to supervisor mode
/// If a bit is set (1), that exception is delegated to supervisor mode.
/// If the bit is cleared (0), the exception remains in machine mode.
pub unsafe fn write(x: usize) {
    core::arch::asm!("csrw mideleg, {}", in(reg) x);
}
