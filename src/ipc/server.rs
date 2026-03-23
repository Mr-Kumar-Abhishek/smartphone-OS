//! IPC server for service registration and request dispatching.

use crate::ipc::{Channel, Endpoint, Message};
use spin::Mutex;

const MAX_SERVICES: usize = 8;

/// Global service registry.
static SERVICE_REGISTRY: Mutex<ServiceRegistry> = Mutex::new(ServiceRegistry::new());

struct ServiceRegistry {
    endpoints: [Option<Endpoint>; MAX_SERVICES],
    count: usize,
}

impl ServiceRegistry {
    const fn new() -> Self {
        const NONE: Option<Endpoint> = None;
        Self {
            endpoints: [NONE; MAX_SERVICES],
            count: 0,
        }
    }

    /// Register a new endpoint.
    fn register(&mut self, endpoint: Endpoint) -> Result<(), &'static str> {
        // Check for duplicate name
        for i in 0..self.count {
            if let Some(ep) = &self.endpoints[i] {
                if ep.name == endpoint.name {
                    return Err("service name already registered");
                }
            }
        }
        if self.count >= MAX_SERVICES {
            return Err("service registry full");
        }
        self.endpoints[self.count] = Some(endpoint);
        self.count += 1;
        Ok(())
    }

    /// Look up an endpoint by name.
    fn find(&mut self, name: &str) -> Option<&mut Endpoint> {
        for i in 0..self.count {
            if let Some(ep) = &mut self.endpoints[i] {
                if ep.name == name {
                    return Some(ep);
                }
            }
        }
        None
    }
}

/// Register a service endpoint with the global registry.
pub fn register(endpoint: Endpoint) -> Result<(), &'static str> {
    SERVICE_REGISTRY.lock().register(endpoint)
}

/// Handle a request by sending it to the appropriate service and returning the response.
/// This is a synchronous call that blocks until the service responds.
pub fn call_service(service_name: &str, request: &[u8]) -> Option<Message> {
    let mut registry = SERVICE_REGISTRY.lock();
    let endpoint = registry.find(service_name)?;
    let req_msg = Message::from_slice(request)?;
    // For simplicity we assume the service will immediately respond on the same channel.
    // In a real system we would have separate request/response channels.
    endpoint.channel.send(req_msg);
    // Wait for response (the service should have sent something back)
    Some(endpoint.channel.receive())
}