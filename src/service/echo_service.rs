//! Example echo service that returns the same message it receives.

use crate::service::Service;
use crate::ipc::Message;

/// Echo service.
pub struct EchoService;

impl EchoService {
    pub const fn new() -> Self {
        Self
    }
}

impl Service for EchoService {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn init(&mut self) {
        crate::uart::write_line("[EchoService] initialized");
    }

    fn handle_request(&mut self, request: &Message) -> Option<Message> {
        crate::uart::write_str("[EchoService] received: ");
        crate::uart::write_line(core::str::from_utf8(request.as_slice()).unwrap_or("?"));
        // Echo back the same data
        Some(Message::from_slice(request.as_slice()).unwrap())
    }
}