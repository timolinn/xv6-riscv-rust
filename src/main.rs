#![no_std]
#![no_main]

mod kernel;

use core::sync::atomic::AtomicBool;

use kernel::*;
use riscv_registers::tp;

static STARTED: AtomicBool = AtomicBool::new(false);

fn cpuid() -> usize {
    unsafe { tp::read() }
}

// start jumps here in supervisor mode
fn main() {
    if cpuid() == 0 {
        println!("");
        println!("xv6 kernel is booting");
        println!("");
        STARTED.store(true, core::sync::atomic::Ordering::SeqCst);
    } else {
        while !STARTED.load(core::sync::atomic::Ordering::SeqCst) {
            core::hint::spin_loop();
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(out) = crate::stdio::panic_output() {
        let _ = out.write_fmt(format_args!("{}\n", _info));
    }
    sys::exit(-1)
}
