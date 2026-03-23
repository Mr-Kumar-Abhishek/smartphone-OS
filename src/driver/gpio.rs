//! GPIO stub driver for Redme-9A.
//!
//! This is a placeholder driver that only logs via UART. In a real system
//! it would configure GPIO pins and handle interrupts.

use crate::driver::{Driver, DriverError, DriverResult};
use crate::uart;

/// GPIO driver stub.
pub struct GpioDriver;

impl GpioDriver {
    /// Log a message via UART (for demonstration).
    fn log(&self, msg: &str) {
        uart::write_line(msg);
    }
}

impl Driver for GpioDriver {
    fn name(&self) -> &'static str {
        "gpio-stub"
    }

    fn device_type(&self) -> &'static str {
        "gpio"
    }

    fn init(&mut self) -> DriverResult {
        self.log("[GPIO] stub driver initialized");
        Ok(())
    }

    fn read(&mut self, offset: usize, buf: &mut [u8]) -> DriverResult<usize> {
        self.log("[GPIO] read called");
        // Simulate reading zeroes
        for b in buf.iter_mut() {
            *b = 0;
        }
        Ok(buf.len())
    }

    fn write(&mut self, offset: usize, buf: &[u8]) -> DriverResult<usize> {
        self.log("[GPIO] write called");
        Ok(buf.len())
    }

    fn ioctl(&mut self, cmd: usize, arg: usize) -> DriverResult<usize> {
        self.log("[GPIO] ioctl called");
        match cmd {
            0 => Ok(0xDEAD_BEEF), // dummy return value
            _ => Err(DriverError::NotSupported),
        }
    }

    fn as_any(&mut self) -> &mut dyn core::any::Any {
        self
    }
}