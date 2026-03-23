//! IPC client for sending requests to services.

use crate::ipc::{Message, call_service};

/// Send a request to a service and wait for a response.
pub fn send_request(service_name: &str, request: &[u8]) -> Option<Message> {
    call_service(service_name, request)
}

/// Syscall stub for sending a message.
/// This will be invoked from user space via `svc` instruction.
pub fn sys_send(service_name_ptr: *const u8, service_name_len: usize, request_ptr: *const u8, request_len: usize) -> isize {
    // For now, just log and return success.
    // In a real implementation we would copy the data from user space.
    unsafe {
        crate::uart::write_str("[IPC] sys_send called\n");
    }
    0
}

/// Syscall stub for receiving a message.
pub fn sys_receive(service_name_ptr: *const u8, service_name_len: usize, buf_ptr: *mut u8, buf_len: usize) -> isize {
    unsafe {
        crate::uart::write_str("[IPC] sys_receive called\n");
    }
    0
}