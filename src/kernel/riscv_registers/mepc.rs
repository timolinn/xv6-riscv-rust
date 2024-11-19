use core::arch::asm;

/// RISC-V Machine Exception Program Counter register
/// write value to mepc register.
/// mepc register holds the machine exception program counter.
/// which is the return address when mret instruction is executed.
pub unsafe fn write(x: usize) {
    asm!("csrw mepc, {}", in(reg) x);
}
