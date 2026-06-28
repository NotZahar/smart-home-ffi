use std::collections::BTreeMap;
use std::fmt;

use num_traits::FromPrimitive;

use crate::home::{Home, SmartHome};
use crate::room::{Room, SmartRoom};
use crate::smart_device::Device;
use crate::utils::trait_alias::RandomNumber;

pub struct HomeBuilder<T: RandomNumber = f32> {
    rooms: BTreeMap<String, SmartRoom<T>>,
}

pub struct HomeBuilderWithRoom<T: RandomNumber = f32> {
    rooms: BTreeMap<String, SmartRoom<T>>,
    current_room: String,
}

impl HomeBuilder<f32> {
    /// Creates a builder without active rooms
    ///
    /// Adding a device before the first room is forbidden by the type system:
    ///
    /// ```compile_fail
    /// use sh::builder::HomeBuilder;
    /// use sh::smart_device::SmartSocket;
    ///
    /// let _home = HomeBuilder::new()
    ///     .add_device("Socket_1", SmartSocket::default())
    ///     .build();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            rooms: BTreeMap::new(),
        }
    }
}

impl Default for HomeBuilder<f32> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> HomeBuilder<T>
where
    T: RandomNumber + FromPrimitive + fmt::Debug,
{
    #[must_use]
    pub fn add_room(mut self, room_name: impl Into<String>) -> HomeBuilderWithRoom<T> {
        let room_name = room_name.into();
        self.rooms.entry(room_name.clone()).or_default();

        HomeBuilderWithRoom {
            rooms: self.rooms,
            current_room: room_name,
        }
    }

    #[must_use]
    pub fn build(self) -> SmartHome<T> {
        SmartHome::new(self.rooms)
    }
}

impl<T> HomeBuilderWithRoom<T>
where
    T: RandomNumber + FromPrimitive + fmt::Debug,
{
    #[must_use]
    pub fn add_room(mut self, room_name: impl Into<String>) -> Self {
        let room_name = room_name.into();
        self.rooms.entry(room_name.clone()).or_default();
        self.current_room = room_name;
        self
    }

    #[must_use]
    pub fn add_device<D>(mut self, device_name: impl Into<String>, device: D) -> Self
    where
        D: Into<Device<T>>,
    {
        if let Some(room) = self.rooms.get_mut(&self.current_room) {
            room.add_device(device_name, device);
        }
        self
    }

    #[must_use]
    pub fn build(self) -> SmartHome<T> {
        SmartHome::new(self.rooms)
    }
}
