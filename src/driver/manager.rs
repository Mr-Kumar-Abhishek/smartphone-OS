//! Driver manager that holds registered drivers and dispatches operations.

use crate::driver::Driver;
use spin::Mutex;

const MAX_DRIVERS: usize = 16;

/// Global driver manager instance.
static DRIVER_MANAGER: Mutex<DriverManager> = Mutex::new(DriverManager::new());

/// Stores all registered drivers in a fixed‑capacity array.
struct DriverManager {
    drivers: [Option<&'static mut dyn Driver>; MAX_DRIVERS],
    count: usize,
}

impl DriverManager {
    const fn new() -> Self {
        const NONE: Option<&'static mut dyn Driver> = None;
        Self {
            drivers: [NONE; MAX_DRIVERS],
            count: 0,
        }
    }

    /// Register a new driver. The driver must be `'static`.
    /// Returns an error if a driver with the same name already exists or capacity is exceeded.
    pub fn register(&mut self, driver: &'static mut dyn Driver) -> Result<(), &'static str> {
        // Check for duplicates
        for i in 0..self.count {
            if let Some(d) = self.drivers[i] {
                if d.name() == driver.name() {
                    return Err("driver with same name already registered");
                }
            }
        }
        if self.count >= MAX_DRIVERS {
            return Err("driver capacity exceeded");
        }
        self.drivers[self.count] = Some(driver);
        self.count += 1;
        Ok(())
    }

    /// Initialize all registered drivers in registration order.
    pub fn init_all(&mut self) {
        for i in 0..self.count {
            if let Some(driver) = self.drivers[i].as_mut() {
                let _ = driver.init();
            }
        }
    }

    /// Find a driver by name.
    pub fn find_by_name(&mut self, name: &str) -> Option<&mut dyn Driver> {
        for i in 0..self.count {
            if let Some(driver) = self.drivers[i].as_mut() {
                if driver.name() == name {
                    return Some(driver);
                }
            }
        }
        None
    }

    /// Find a driver by device type.
    pub fn find_by_type(&mut self, dev_type: &str) -> Option<&mut dyn Driver> {
        for i in 0..self.count {
            if let Some(driver) = self.drivers[i].as_mut() {
                if driver.device_type() == dev_type {
                    return Some(driver);
                }
            }
        }
        None
    }

    /// Number of registered drivers.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Iterator over all drivers (mutable).
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut dyn Driver> {
        self.drivers[0..self.count].iter_mut().filter_map(|opt| opt.as_deref_mut())
    }
}

/// Public interface to the global driver manager.
pub struct DriverManagerHandle;

impl DriverManagerHandle {
    /// Register a new driver. The driver must be `'static`.
    pub fn register(driver: &'static mut dyn Driver) -> Result<(), &'static str> {
        DRIVER_MANAGER.lock().register(driver)
    }

    /// Initialize all registered drivers.
    pub fn init_all() {
        DRIVER_MANAGER.lock().init_all();
    }

    /// Find a driver by name.
    pub fn find_by_name(name: &str) -> Option<DriverManagerHandleGuard> {
        let mut mgr = DRIVER_MANAGER.lock();
        mgr.find_by_name(name).map(|driver| DriverManagerHandleGuard { _lock: mgr, driver })
    }

    /// Find a driver by device type.
    pub fn find_by_type(dev_type: &str) -> Option<DriverManagerHandleGuard> {
        let mut mgr = DRIVER_MANAGER.lock();
        mgr.find_by_type(dev_type).map(|driver| DriverManagerHandleGuard { _lock: mgr, driver })
    }

    /// Number of registered drivers.
    pub fn len() -> usize {
        DRIVER_MANAGER.lock().len()
    }

    /// Print a list of all drivers (for debugging).
    pub fn list_drivers() {
        let mgr = DRIVER_MANAGER.lock();
        // TODO: use UART to print
        for i in 0..mgr.count {
            if let Some(driver) = mgr.drivers[i].as_ref() {
                // In a real kernel we would log via serial
            }
        }
    }
}

/// Guard that holds the manager lock and a reference to a driver.
pub struct DriverManagerHandleGuard<'a> {
    _lock: spin::MutexGuard<'a, DriverManager>,
    pub driver: &'a mut dyn Driver,
}