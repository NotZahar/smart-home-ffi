use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum HomeError {
    RoomNotFound(String),
    DeviceNotFound(String),
}

impl fmt::Display for HomeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HomeError::RoomNotFound(room) => write!(formatter, "room '{room}' not found"),
            HomeError::DeviceNotFound(device) => write!(formatter, "device '{device}' not found"),
        }
    }
}

impl Error for HomeError {}
