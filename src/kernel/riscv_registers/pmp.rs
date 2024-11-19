/// Physical Memory Protection (PMP) Configuration
/// this writes to PMP. Which configures the Physical Memory Protection
/// access permissions for the processor.
pub unsafe fn write_pmpaddr0(x: usize) {
    core::arch::asm!("csrw pmpaddr0, {}", in(reg) x);
}

/// configurss the Physical Memory Protection (PMP)
/// region-zero permissions
pub unsafe fn write_pmpcfg0(x: usize) {
    core::arch::asm!("csrw pmpcfg0, {}", in(reg) x);
}
