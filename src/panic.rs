//! Panic handler for the kernel.

use core::panic::PanicInfo;
use crate::uart;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Try to print panic message via UART
    uart::write_line("\n*** KERNEL PANIC ***");
    if let Some(location) = info.location() {
        uart::write_str("Panic at ");
        uart::write_str(location.file());
        uart::write_str(":");
        uart::write_str(&location.line().to_string());
        uart::write_str("\r\n");
    }
    if let Some(message) = info.message() {
        uart::write_str("Message: ");
        // We cannot format the message easily, just loop.
        // For simplicity, we ignore formatting.
    }
    uart::write_line("Halting...");

    // Loop forever
    loop {
        // maybe wait for interrupt
        cortex_a::asm::wfe();
    }
}