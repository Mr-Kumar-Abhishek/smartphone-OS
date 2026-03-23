//! PL011 UART driver for QEMU virt machine.
//! UART0 base address: 0x0900_0000

use core::fmt;

const UART0_BASE: usize = 0x0900_0000;

/// PL011 registers (offsets from base)
#[repr(u32)]
enum Reg {
    DR      = 0x00, // Data register
    FR      = 0x18, // Flag register
    IBRD    = 0x24, // Integer baud rate divisor
    FBRD    = 0x28, // Fractional baud rate divisor
    LCR_H   = 0x2C, // Line control register
    CR      = 0x30, // Control register
    IMSC    = 0x38, // Interrupt mask set/clear
    ICR     = 0x44, // Interrupt clear register
}

/// Flag register bits
mod fr {
    pub const TXFF: u32 = 1 << 5; // Transmit FIFO full
    pub const RXFE: u32 = 1 << 4; // Receive FIFO empty
}

/// Line control register bits
mod lcr_h {
    pub const FEN: u32 = 1 << 4; // FIFO enable
    pub const WLEN_8: u32 = 3 << 5; // 8 bits
}

/// Control register bits
mod cr {
    pub const UARTEN: u32 = 1 << 0; // UART enable
    pub const TXE: u32 = 1 << 8; // Transmit enable
    pub const RXE: u32 = 1 << 9; // Receive enable
}

/// UART driver struct
pub struct Uart {
    base: usize,
}

impl Uart {
    /// Create a new UART instance at the given base address.
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    /// Initialize the UART with default settings (115200 baud, 8N1).
    pub fn init(&self) {
        unsafe {
            // Disable UART while configuring
            self.write_reg(Reg::CR, 0);

            // Set baud rate divisor for 115200 baud (assuming 48MHz UART clock)
            // divisor = 48_000_000 / (16 * 115200) = 26.041666...
            // IBRD = 26, FBRD = 1 (approx)
            self.write_reg(Reg::IBRD, 26);
            self.write_reg(Reg::FBRD, 1);

            // 8 bits, no parity, FIFO enabled
            self.write_reg(Reg::LCR_H, lcr_h::WLEN_8 | lcr_h::FEN);

            // Enable interrupts (optional)
            self.write_reg(Reg::IMSC, 0);

            // Enable UART, transmit, receive
            self.write_reg(Reg::CR, cr::UARTEN | cr::TXE | cr::RXE);
        }
    }

    /// Write a single character to the UART.
    pub fn write_char(&self, c: u8) {
        unsafe {
            // Wait until transmit FIFO is not full
            while self.read_reg(Reg::FR) & fr::TXFF != 0 {}

            self.write_reg(Reg::DR, c as u32);
        }
    }

    /// Read a single character from the UART (blocking).
    pub fn read_char(&self) -> u8 {
        unsafe {
            // Wait until receive FIFO is not empty
            while self.read_reg(Reg::FR) & fr::RXFE != 0 {}

            (self.read_reg(Reg::DR) & 0xFF) as u8
        }
    }

    /// Check if there is a character ready to read.
    pub fn has_char(&self) -> bool {
        unsafe {
            (self.read_reg(Reg::FR) & fr::RXFE) == 0
        }
    }

    /// Write a raw 32‑bit value to a register.
    unsafe fn write_reg(&self, reg: Reg, value: u32) {
        let addr = (self.base as *mut u32).offset(reg as isize);
        core::ptr::write_volatile(addr, value);
    }

    /// Read a raw 32‑bit value from a register.
    unsafe fn read_reg(&self, reg: Reg) -> u32 {
        let addr = (self.base as *mut u32).offset(reg as isize);
        core::ptr::read_volatile(addr)
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_char(byte);
        }
        Ok(())
    }
}

/// Global UART0 instance
pub static mut UART0: Uart = Uart::new(UART0_BASE);

/// Initialize the global UART0.
pub fn init() {
    unsafe {
        UART0.init();
    }
}

/// Write a string to UART0 (for early debugging).
pub fn write_str(s: &str) {
    unsafe {
        let _ = fmt::Write::write_str(&mut UART0, s);
    }
}

/// Write a line (string + newline).
pub fn write_line(s: &str) {
    write_str(s);
    write_str("\r\n");
}