/// UART stands for Universal Asynchronous Receiver/Transmitter.
/// The UART hardware that the driver talks to is a 16550 chip emulated by QEMU. On a real
/// computer, a 16550 would manage an RS232 serial link connecting to a terminal or other computer.
/// To a software the UART appears as a set of memory-mapped control registers.
/// There are some physical addresses that RISC-V hardware connects to the UART device, so that
/// loads and stores interact with the device hardware rather than RAM.
/// The memory-mapped addresses for the UART start at 0x10000000, or UART0
/// the LSR register contains bits that indicate whether input characters
/// are waiting to be read by the software. These characters (if any) are available for reading from the RHR register
/// The UART transmit hardware is largely independent of the receive hardware; 
/// if software writes a byte to the THR, the UART transmits that byte