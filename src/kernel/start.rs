use crate::main;

use super::riscv_registers::{
    mcounteren, medeleg, menvcfg, mepc, mhartid, mideleg, mie, mret, mstatus, pmp, satp, sie,
    stimecmp, time, tp,
};

pub const NUM_CPU: usize = 4;
pub const NUM_STACK_PAGE: usize = 8;

#[repr(C, align(16))]
#[derive(Debug)]
pub struct Stack([u8; 4096 * NUM_STACK_PAGE * NUM_CPU]);

#[no_mangle]
pub static mut STACK0: Stack = Stack([0; 4096 * NUM_STACK_PAGE * NUM_CPU]);

pub unsafe fn start() -> ! {
    // set Machine Previous Privilege mode to Supervisor, for mret (Machine Return)
    mstatus::set_mpp_supervisor();

    // set MEPC to main function, for mret
    mepc::write(main as usize);

    // disable paging for now.
    // disables virtual address translation in supervisor mode
    // by writing 0 into the page-table register satp
    satp::write(0);

    // delegate all interrupts and exceptions to supervisor mode.
    medeleg::write(0xffff); // 0xffff is the value that enables first 16 bits of medeleg and mideleg
    mideleg::write(0xffff);
    sie::intr_enable();

    // configure Physical Memory Protection to give supervisor mode
    // access to all of physical memory.
    pmp::write_pmpaddr0(0x3fffffffffffff);
    pmp::write_pmpcfg0(0xf); // sets read, write, execute and mode on pmp region 0 configuration

    // setup and ask for clock interrupts.
    timer_init();

    // keep each CPU's hartid in its tp register, for cpuid().
    let id = mhartid::read();
    tp::write(id);

    // switch to supervisor mode and jump to main().
    mret::exec();

    loop {}
}

unsafe fn timer_init() {
    // enable supervisor mode timer interrupts
    mie::write(mie::read() | mie::STIE);

    // enable sstc extension (supervisor mode simplified timer control).
    // ie. stimecmp, which is where timer interrupt moment is stored
    // this enables us to access stimecmp, stime which are supervisor mode registers
    // otherwise we would have to use mtimecmp, mtime which are machine mode registers
    // and we would have to switch to machine mode everytime we want to set timer interrupt moment
    menvcfg::write(menvcfg::read() | menvcfg::SSTC);

    // allow supervisor mode to use stimecmp and stime registers
    mcounteren::write(mcounteren::read() | 2);

    // ask for the very first timer interrupt
    let interval = 1000000u64; // cycles; about 1/10th second in qemu.
    stimecmp::write(time::read() + interval as usize);
}
