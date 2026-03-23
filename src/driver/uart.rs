//! PL011 UART driver implementing the Driver trait.

use crate::driver::{Driver, DriverError, DriverResult};
use crate::uart::Uart;

/// UART driver wrapper that delegates to the global UART0 instance.
pub struct UartDriver;

impl UartDriver {
    /// Returns a mutable reference to the global UART0 instance.
    /// SAFETY: This function is unsafe because it dereferences a static mut.
    /// The caller must ensure that no other references to UART0 exist simultaneously.
    unsafe fn uart0() -> &'static mut Uart {
        &mut crate::uart::UART0
    }
}

impl Driver for UartDriver {
    fn name(&self) -> &'static str {
        "pl011-uart"
    }

    fn device_type(&self) -> &'static str {
        "uart"
    }

    fn init(&mut self) -> DriverResult {
        // UART is already initialized by the kernel's early init.
        // We could re‑initialize, but for now just confirm it works.
        Ok(())
    }

    fn read(&mut self, _offset: usize, buf: &mut [u8]) -> DriverResult<usize> {
        let uart = unsafe { Self::uart0() };
        for (i, byte) in buf.iter_mut().enumerate() {
            *byte = uart.read_char();
            // Stop if no more characters available (non‑blocking would need a flag)
            // For simplicity we read exactly buf.len() characters, blocking.
        }
        Ok(buf.len())
    }

    fn write(&mut self, _offset: usize, buf: &[u8]) -> DriverResult<usize> {
        let uart = unsafe { Self::uart0() };
        for &byte in buf {
            uart.write_char(byte);
        }
        Ok(buf.len())
    }

    fn ioctl(&mut self, cmd: usize, arg: usize) -> DriverResult<usize> {
        match cmd {
            // Example: get baud rate, set baud rate, etc.
            0 => Ok(0), // dummy
            _ => Err(DriverError::NotSupported),
        }
    }

    fn as_any(&mut self) -> &mut dyn core::any::Any {
        self
    }
}