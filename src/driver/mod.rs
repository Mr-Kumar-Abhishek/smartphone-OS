//! Driver framework for the Redme-9A microkernel.
//!
//! This module provides a trait for hardware drivers and a manager
//! that can register, initialize, and dispatch operations to drivers.

use core::any::Any;

pub mod manager;
pub mod uart;
pub mod gpio;
pub use manager::DriverManagerHandle;

/// Result type for driver operations.
pub type DriverResult<T = ()> = Result<T, DriverError>;

/// Errors that can occur in driver operations.
#[derive(Debug, Clone, Copy)]
pub enum DriverError {
    /// Operation not supported by this driver.
    NotSupported,
    /// Device is busy or not ready.
    Busy,
    /// Invalid argument passed.
    InvalidArgument,
    /// Hardware error (e.g., timeout, checksum).
    Hardware,
    /// No such device.
    NoDevice,
    /// Internal driver error.
    Other,
}

/// A trait that all hardware drivers must implement.
pub trait Driver: Send + Sync {
    /// Returns the human‑readable name of the driver.
    fn name(&self) -> &'static str;

    /// Returns the device type (e.g., "uart", "gpio", "block").
    fn device_type(&self) -> &'static str;

    /// Initialize the hardware and make it ready for operation.
    /// Called once during system startup.
    fn init(&mut self) -> DriverResult;

    /// Called when the system is shutting down (optional).
    fn shutdown(&mut self) -> DriverResult {
        Ok(())
    }

    /// Handle an interrupt, if the driver uses interrupts.
    /// The `irq` parameter identifies the interrupt line.
    fn interrupt_handler(&mut self, _irq: usize) -> DriverResult {
        Ok(())
    }

    /// Perform a read operation. The meaning of `offset` and `buf` is driver‑specific.
    fn read(&mut self, _offset: usize, _buf: &mut [u8]) -> DriverResult<usize> {
        Err(DriverError::NotSupported)
    }

    /// Perform a write operation. The meaning of `offset` and `buf` is driver‑specific.
    fn write(&mut self, _offset: usize, _buf: &[u8]) -> DriverResult<usize> {
        Err(DriverError::NotSupported)
    }

    /// Perform a device‑specific control operation (ioctl).
    fn ioctl(&mut self, _cmd: usize, _arg: usize) -> DriverResult<usize> {
        Err(DriverError::NotSupported)
    }

    /// Downcast to `Any` for dynamic type inspection.
    fn as_any(&mut self) -> &mut dyn Any;
}

/// Dummy driver that does nothing, used for testing.
pub struct NullDriver;

impl Driver for NullDriver {
    fn name(&self) -> &'static str {
        "null"
    }

    fn device_type(&self) -> &'static str {
        "null"
    }

    fn init(&mut self) -> DriverResult {
        Ok(())
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}