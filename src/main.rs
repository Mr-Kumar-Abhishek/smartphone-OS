//! Redme 9A OS - Microkernel entry point
//!
//! This is the main entry point for the operating system kernel.
//! It uses `#![no_std]` and a custom boot process.

#![no_std]
#![no_main]
#![feature(asm_const)]

use core::arch::global_asm;
use cortex_a::{asm, registers::*};
use core::panic::PanicInfo;

use driver::DriverManagerHandle;
use driver::uart::UartDriver;
use driver::gpio::GpioDriver;
use service::ServiceManagerHandle;
use service::echo_service::EchoService;
use ipc::server;

mod uart;
mod panic;
mod driver;
mod ipc;
mod service;

// Include assembly startup code
global_asm!(include_str!("asm.s"));

// Symbols defined in linker script
extern "C" {
    static _stack_top: u8;
    static _bss_start: u8;
    static _bss_end: u8;
}

// Static driver instances
static mut UART_DRIVER: UartDriver = UartDriver;
static mut GPIO_DRIVER: GpioDriver = GpioDriver;
static mut ECHO_SERVICE: EchoService = EchoService::new();

/// Rust entry point called from assembly.
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Initialize UART for serial output
    uart::init();
    uart::write_line("\r\n========================================");
    uart::write_line("Redme-9A Microkernel Boot");
    uart::write_line("========================================");
    uart::write_line("Initializing...");

    // Set up exception vector table
    unsafe {
        let vector_table_addr = vectors as *const () as u64;
        VBAR_EL1.set(vector_table_addr);
        uart::write_str("VBAR_EL1 set to 0x");
        uart::write_line(&hex(vector_table_addr));
    }

    // Print some CPU information
    let current_el = CurrentEL.read_as_enum(CurrentEL::EL);
    match current_el {
        Some(CurrentEL::EL::Value::EL1) => uart::write_line("Current Exception Level: EL1"),
        Some(CurrentEL::EL::Value::EL2) => uart::write_line("Current Exception Level: EL2"),
        Some(CurrentEL::EL::Value::EL3) => uart::write_line("Current Exception Level: EL3"),
        _ => uart::write_line("Current Exception Level: Unknown"),
    }

    // Enable interrupts (optional)
    // asm::msr(DAIFClr, 0b1111); // clear all interrupt masks

    // --- Driver Framework Initialization ---
    uart::write_line("Initializing driver framework...");
    unsafe {
        // Register drivers
        DriverManagerHandle::register(&mut UART_DRIVER)
            .expect("Failed to register UART driver");
        DriverManagerHandle::register(&mut GPIO_DRIVER)
            .expect("Failed to register GPIO driver");
        DriverManagerHandle::init_all();
        uart::write_line("Drivers registered and initialized.");
    }

    // --- Service Manager Initialization ---
    uart::write_line("Initializing service manager...");
    unsafe {
        ServiceManagerHandle::register(&mut ECHO_SERVICE)
            .expect("Failed to register echo service");
        ServiceManagerHandle::init_all();
        uart::write_line("Services registered and initialized.");
    }

    // --- IPC Demonstration ---
    uart::write_line("Testing IPC with echo service...");
    // For simplicity we call the service directly (bypassing IPC channels)
    // In a real microkernel we would send a message via IPC.
    let request = b"Hello, Redme-9A!";
    let response = unsafe { ECHO_SERVICE.handle_request(&ipc::Message::from_slice(request).unwrap()) };
    if let Some(msg) = response {
        uart::write_str("Echo response: ");
        uart::write_line(core::str::from_utf8(msg.as_slice()).unwrap_or("?"));
    } else {
        uart::write_line("No response from echo service.");
    }

    uart::write_line("Kernel initialization complete.");
    uart::write_line("Entering idle loop...");

    // Main idle loop
    loop {
        // Wait for interrupt (low power)
        asm::wfe();
    }
}

/// Convert a number to hex string (simple, for debugging)
fn hex(mut n: u64) -> [u8; 16] {
    let mut buf = [b'0'; 16];
    let digits = b"0123456789abcdef";
    for i in (0..16).rev() {
        buf[i] = digits[(n & 0xF) as usize];
        n >>= 4;
    }
    buf
}

/// Exception vector table (defined in asm.s)
extern "C" {
    fn vectors();
}
