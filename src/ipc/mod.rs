//! Inter‑Process Communication (IPC) for the Redme‑9A microkernel.
//!
//! This module provides message‑passing primitives that allow user‑space
//! services and drivers to communicate.

use spin::Mutex;

/// Maximum size of a single IPC message in bytes.
pub const MAX_MSG_SIZE: usize = 256;

/// A message that can be sent over an IPC channel.
#[derive(Debug, Clone)]
pub struct Message {
    /// Raw message data.
    pub data: [u8; MAX_MSG_SIZE],
    /// Number of valid bytes in `data`.
    pub len: usize,
}

impl Message {
    /// Create an empty message.
    pub const fn empty() -> Self {
        Self {
            data: [0; MAX_MSG_SIZE],
            len: 0,
        }
    }

    /// Create a message from a slice.
    pub fn from_slice(slice: &[u8]) -> Option<Self> {
        if slice.len() > MAX_MSG_SIZE {
            return None;
        }
        let mut msg = Self::empty();
        msg.data[..slice.len()].copy_from_slice(slice);
        msg.len = slice.len();
        Some(msg)
    }

    /// Get the message content as a slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }
}

/// A synchronous IPC channel that can hold one message at a time.
/// This is a simple blocking queue for demonstration.
pub struct Channel {
    buffer: Mutex<Option<Message>>,
    // In a real implementation we would have wait queues.
}

impl Channel {
    /// Create a new empty channel.
    pub const fn new() -> Self {
        Self {
            buffer: Mutex::new(None),
        }
    }

    /// Send a message into the channel, overwriting any previous pending message.
    /// Returns the previous message if there was one.
    pub fn send(&self, msg: Message) -> Option<Message> {
        let mut guard = self.buffer.lock();
        guard.replace(msg)
    }

    /// Receive a message from the channel, waiting until a message is available.
    /// This is a spinning wait (busy loop) for simplicity.
    pub fn receive(&self) -> Message {
        loop {
            let mut guard = self.buffer.lock();
            if let Some(msg) = guard.take() {
                return msg;
            }
            // Release the lock and spin (yield could be added)
            core::hint::spin_loop();
        }
    }

    /// Try to receive a message without blocking.
    pub fn try_receive(&self) -> Option<Message> {
        let mut guard = self.buffer.lock();
        guard.take()
    }
}

/// A service endpoint that can be registered with the IPC server.
pub struct Endpoint {
    pub name: &'static str,
    pub channel: Channel,
}

impl Endpoint {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            channel: Channel::new(),
        }
    }
}

// Re‑export server and client modules.
pub mod server;
pub mod client;
pub use server::call_service;