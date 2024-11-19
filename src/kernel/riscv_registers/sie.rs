/// RISC-V Supervisor Interrupt Enable Register (sie)
/// sie register holds the supervisor interrupt enable register
/// it is used to enable and disable certain interrupts in supervisor mode
/// If a bit is set (1), that interrupt is enabled.
/// If the bit is cleared (0), the interrupt is disabled.
pub enum SIE {
    SEIE = 1 << 9, // external
    STIE = 1 << 5, // timer
    SSIE = 1 << 1, // software
}

#[inline]
unsafe fn write(x: usize) {
    core::arch::asm!("csrw sie, {}", in(reg) x);
}

/// enable all interrupts
#[inline]
pub unsafe fn intr_enable() {
    let mut sie = 0;
    sie |= SIE::SSIE as usize;
    sie |= SIE::STIE as usize;
    sie |= SIE::SEIE as usize;

    write(sie);
}
