use std::collections::BTreeMap;
use std::fmt;

use num_traits::FromPrimitive;

use crate::report::Report;
use crate::smart_device::Device;
use crate::utils::trait_alias::RandomNumber;

pub trait Subscriber<T: RandomNumber> {
    fn on_event(&mut self, device: &Device<T>);
}

impl<T, F> Subscriber<T> for F
where
    T: RandomNumber,
    F: FnMut(&Device<T>),
{
    fn on_event(&mut self, device: &Device<T>) {
        self(device);
    }
}

pub trait Room<T: RandomNumber> {
    #[must_use]
    fn new(devices: BTreeMap<String, Device<T>>) -> Self;

    #[must_use]
    fn get_device(&self, key: &str) -> Option<&Device<T>>;

    #[must_use]
    fn get_device_mut(&mut self, key: &str) -> Option<&mut Device<T>>;

    fn add_device(&mut self, key: impl Into<String>, device: impl Into<Device<T>>);

    fn remove_device(&mut self, key: &str);

    fn subscribe<S>(&mut self, subscriber: S)
    where
        S: Subscriber<T> + 'static;
}

pub struct SmartRoom<T: RandomNumber = f32> {
    devices: BTreeMap<String, Device<T>>,
    subscribers: Vec<Box<dyn Subscriber<T>>>,
}

impl<T: RandomNumber + fmt::Debug> fmt::Debug for SmartRoom<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SmartRoom")
            .field("devices", &self.devices)
            .field("subscribers", &self.subscribers.len())
            .finish()
    }
}

impl<T: RandomNumber> Default for SmartRoom<T> {
    fn default() -> Self {
        SmartRoom {
            devices: BTreeMap::new(),
            subscribers: Vec::new(),
        }
    }
}

impl<T> SmartRoom<T>
where
    T: RandomNumber + FromPrimitive + fmt::Debug,
{
    #[must_use]
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    #[must_use]
    pub fn devices(&self) -> &BTreeMap<String, Device<T>> {
        &self.devices
    }

    fn notify_subscribers(&mut self, device_key: &str) {
        let device = self
            .devices
            .get(device_key)
            .expect("just inserted device must exist");
        for subscriber in &mut self.subscribers {
            subscriber.on_event(device);
        }
    }
}

impl<T> Room<T> for SmartRoom<T>
where
    T: RandomNumber + FromPrimitive + fmt::Debug,
{
    fn new(devices: BTreeMap<String, Device<T>>) -> Self {
        SmartRoom {
            devices,
            subscribers: Vec::new(),
        }
    }

    fn get_device(&self, key: &str) -> Option<&Device<T>> {
        self.devices.get(key)
    }

    fn get_device_mut(&mut self, key: &str) -> Option<&mut Device<T>> {
        self.devices.get_mut(key)
    }

    fn add_device(&mut self, key: impl Into<String>, device: impl Into<Device<T>>) {
        let key = key.into();
        self.devices.insert(key.clone(), device.into());
        self.notify_subscribers(&key);
    }

    fn remove_device(&mut self, key: &str) {
        self.devices.remove(key);
    }

    fn subscribe<S>(&mut self, subscriber: S)
    where
        S: Subscriber<T> + 'static,
    {
        self.subscribers.push(Box::new(subscriber));
    }
}

impl<T> Report for SmartRoom<T>
where
    T: RandomNumber + fmt::Debug,
{
    fn report(&self) -> String {
        if self.devices.is_empty() {
            return String::from("Room is empty\n");
        }

        let mut result = String::new();
        for (device_name, device) in &self.devices {
            result.push_str(&format!("Device '{device_name}': {}\n", device.report()));
        }
        result
    }
}
