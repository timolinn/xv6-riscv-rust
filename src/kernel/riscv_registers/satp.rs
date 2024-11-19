/// write the value of x to the satp register
/// satp register holds the supervisor address translation and protection
/// which holds the physical page number of the root page table
/// and the mode of address translation and protection.
pub unsafe fn write(x: usize) {
    core::arch::asm!("csrw satp, {}", in(reg) x);
}
