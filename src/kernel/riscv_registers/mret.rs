pub unsafe fn exec() {
    core::arch::asm!("mret");
}
