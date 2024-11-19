use core::arch::asm;

/// previous privilege
const MSTATUS_MPP_MASK: u64 = 3 << 11;
/// supervisor mode
const MSTATUS_MPP_S: u64 = 1 << 11;
/// machine-mode interrupt enable
const MSTATUS_MIE: u64 = 1 << 3;

// RISC-V Machine Status Register
/// reads value from mstatus register
/// mstatus register holds the machine status register
/// which contains the machine-level operating mode and status flags
unsafe fn read() -> u64 {
    let r: u64;
    asm!("csrr {}, mstatus", out(reg) r);
    r
}

// RISC-V Machine Status Register
/// writes value to mstatus register
/// mstatus register holds the machine status register
/// which contains the machine-level operating mode and status flags
unsafe fn write(x: u64) {
    asm!("csrw mstatus, {}", in(reg) x);
}

// RISC-V Machine Status Register
/// set Machine Previous Privilege mode to Supervisor, for mret (Machine Return).
pub unsafe fn set_mpp_supervisor() {
    let mut x = read();
    x &= !MSTATUS_MPP_MASK;
    x |= MSTATUS_MPP_S;
    write(x);
}
