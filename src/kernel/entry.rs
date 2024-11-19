use crate::kernel::start::start;
use crate::kernel::start::STACK0;
use crate::start::Stack;
use core::arch::asm;

pub extern "C" fn _entry() -> ! {
    // set up stack pointer for this hart
    // with 32kb bytes stack per CPU.
    // sp = stack0 + (hartid * 4096 * 8)
    unsafe {
        asm!(
            "la sp, STACK0",
            "li a0, {0}",
            "csrr a1, mhartid",
            "addi a1, a1, 1",
            "mul a0, a0, a1",
            "add sp, sp, a0",
            const 4096 * 8, // stack size at 4kb per page
        );

        start()
    }
}
