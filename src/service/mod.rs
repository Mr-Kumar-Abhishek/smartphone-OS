//! System service manager for the Redme‑9A microkernel.
//!
//! Services are long‑running tasks that provide functionality via IPC.

use crate::ipc::Message;
use spin::Mutex;

pub mod echo_service;

const MAX_SERVICES: usize = 8;

/// Trait that a system service must implement.
pub trait Service: Send + Sync {
    /// Unique name of the service.
    fn name(&self) -> &'static str;

    /// Initialize the service (called once on startup).
    fn init(&mut self) {}

    /// Handle an incoming IPC message.
    /// The service should process the request and optionally send a response.
    fn handle_request(&mut self, request: &Message) -> Option<Message>;
}

/// Global service manager.
static SERVICE_MANAGER: Mutex<ServiceManager> = Mutex::new(ServiceManager::new());

struct ServiceManager {
    services: [Option<&'static mut dyn Service>; MAX_SERVICES],
    count: usize,
}

impl ServiceManager {
    const fn new() -> Self {
        const NONE: Option<&'static mut dyn Service> = None;
        Self {
            services: [NONE; MAX_SERVICES],
            count: 0,
        }
    }

    /// Register a service.
    pub fn register(&mut self, service: &'static mut dyn Service) -> Result<(), &'static str> {
        // Check duplicate
        for i in 0..self.count {
            if let Some(s) = &self.services[i] {
                if s.name() == service.name() {
                    return Err("service already registered");
                }
            }
        }
        if self.count >= MAX_SERVICES {
            return Err("service manager full");
        }
        self.services[self.count] = Some(service);
        self.count += 1;
        Ok(())
    }

    /// Initialize all registered services.
    pub fn init_all(&mut self) {
        for i in 0..self.count {
            if let Some(service) = self.services[i].as_mut() {
                service.init();
            }
        }
    }

    /// Find a service by name.
    pub fn find(&mut self, name: &str) -> Option<&mut dyn Service> {
        for i in 0..self.count {
            if let Some(service) = self.services[i].as_mut() {
                if service.name() == name {
                    return Some(service);
                }
            }
        }
        None
    }
}

/// Public interface to the service manager.
pub struct ServiceManagerHandle;

impl ServiceManagerHandle {
    /// Register a service.
    pub fn register(service: &'static mut dyn Service) -> Result<(), &'static str> {
        SERVICE_MANAGER.lock().register(service)
    }

    /// Initialize all services.
    pub fn init_all() {
        SERVICE_MANAGER.lock().init_all();
    }

}