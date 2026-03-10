//! Redme 9A OS - Microkernel entry point
//!
//! This is the main entry point for the operating system kernel.
//! Eventually this will be a `#![no_std]` binary with custom boot process.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Entry point for the kernel.
/// The bootloader will call this function after loading the kernel.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Kernel initialization will go here
    // For now, just loop forever
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
