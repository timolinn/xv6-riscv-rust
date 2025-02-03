/// The console driver accepts characters typed by a human, via the UART serial-port hardware attached to the RISC-V.
/// The console driver accumulates a line of input at a time, processing special input characters such as
/// backspace and control-u. User processes, such as the shell, use the read system call to fetch lines
// of input from the console. When you type input to xv6 in QEMU, your keystrokes are delivered to
/// xv6 by way of QEMU’s simulated UART hardware.
/// The UART hardware that the driver talks to is a 16550 chip emulated by QEMU. On a real
/// computer, a 16550 would manage an RS232 serial link connecting to a terminal or other computer.
/// When running QEMU, it’s connected to your keyboard and display.
/// The UART hardware appears to software as a set of memory-mapped control registers.
/// there are some physical addresses that RISC-V hardware connects to the UART device, so that
/// loads and stores interact with the device hardware rather than RAM.
