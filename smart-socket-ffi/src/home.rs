use std::collections::BTreeMap;
use std::fmt;

use num_traits::FromPrimitive;

use crate::error::HomeError;
use crate::report::Report;
use crate::room::{Room, SmartRoom};
use crate::smart_device::Device;
use crate::utils::trait_alias::RandomNumber;

pub trait Home<T: RandomNumber> {
    #[must_use]
    fn new(rooms: BTreeMap<String, SmartRoom<T>>) -> Self;

    #[must_use]
    fn get_room(&self, key: &str) -> Option<&SmartRoom<T>>;

    #[must_use]
    fn get_room_mut(&mut self, key: &str) -> Option<&mut SmartRoom<T>>;

    fn add_room(&mut self, key: impl Into<String>, room: SmartRoom<T>);

    fn remove_room(&mut self, key: &str);

    fn get_device(&self, room_key: &str, device_key: &str) -> Result<&Device<T>, HomeError>;

    fn get_device_mut(
        &mut self,
        room_key: &str,
        device_key: &str,
    ) -> Result<&mut Device<T>, HomeError>;
}

pub struct SmartHome<T: RandomNumber = f32> {
    rooms: BTreeMap<String, SmartRoom<T>>,
}

impl<T: RandomNumber + fmt::Debug> fmt::Debug for SmartHome<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SmartHome")
            .field("rooms", &self.rooms)
            .finish()
    }
}

impl<T> SmartHome<T>
where
    T: RandomNumber + FromPrimitive + fmt::Debug,
{
    #[must_use]
    pub fn room_count(&self) -> usize {
        self.rooms.len()
    }

    #[must_use]
    pub fn rooms(&self) -> &BTreeMap<String, SmartRoom<T>> {
        &self.rooms
    }
}

impl<T> Home<T> for SmartHome<T>
where
    T: RandomNumber + FromPrimitive + fmt::Debug,
{
    fn new(rooms: BTreeMap<String, SmartRoom<T>>) -> Self {
        SmartHome { rooms }
    }

    fn get_room(&self, key: &str) -> Option<&SmartRoom<T>> {
        self.rooms.get(key)
    }

    fn get_room_mut(&mut self, key: &str) -> Option<&mut SmartRoom<T>> {
        self.rooms.get_mut(key)
    }

    fn add_room(&mut self, key: impl Into<String>, room: SmartRoom<T>) {
        self.rooms.insert(key.into(), room);
    }

    fn remove_room(&mut self, key: &str) {
        self.rooms.remove(key);
    }

    fn get_device(&self, room_key: &str, device_key: &str) -> Result<&Device<T>, HomeError> {
        let room = self
            .rooms
            .get(room_key)
            .ok_or_else(|| HomeError::RoomNotFound(room_key.to_string()))?;
        room.get_device(device_key)
            .ok_or_else(|| HomeError::DeviceNotFound(device_key.to_string()))
    }

    fn get_device_mut(
        &mut self,
        room_key: &str,
        device_key: &str,
    ) -> Result<&mut Device<T>, HomeError> {
        let room = self
            .rooms
            .get_mut(room_key)
            .ok_or_else(|| HomeError::RoomNotFound(room_key.to_string()))?;
        room.get_device_mut(device_key)
            .ok_or_else(|| HomeError::DeviceNotFound(device_key.to_string()))
    }
}

impl<T> Report for SmartHome<T>
where
    T: RandomNumber + fmt::Debug,
{
    fn report(&self) -> String {
        let mut result = String::from("Home:\n");
        for (room_name, room) in &self.rooms {
            result.push_str(&format!(" - Room '{room_name}':\n"));
            let room_report = room.report();
            for line in room_report.lines() {
                result.push_str(&format!("   - {line}\n"));
            }
        }
        result
    }
}
