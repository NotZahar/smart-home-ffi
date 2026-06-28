use std::fmt;

use crate::report::Report;
use crate::smart_device::{CelsiusThermometer, PowerSocket};
use crate::utils::trait_alias::RandomNumber;

pub enum Device<T: RandomNumber = f32> {
    Socket(PowerSocket<T>),
    Thermometer(CelsiusThermometer<T>),
}

impl Default for Device<f32> {
    fn default() -> Self {
        Device::Socket(PowerSocket::default())
    }
}

impl<T: RandomNumber + fmt::Debug> From<PowerSocket<T>> for Device<T> {
    fn from(socket: PowerSocket<T>) -> Self {
        Device::Socket(socket)
    }
}

impl<T: RandomNumber + fmt::Debug> From<CelsiusThermometer<T>> for Device<T> {
    fn from(thermometer: CelsiusThermometer<T>) -> Self {
        Device::Thermometer(thermometer)
    }
}

impl<T: RandomNumber + fmt::Debug> fmt::Debug for Device<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Device::Socket(socket) => formatter.debug_tuple("Socket").field(socket).finish(),
            Device::Thermometer(thermometer) => formatter
                .debug_tuple("Thermometer")
                .field(thermometer)
                .finish(),
        }
    }
}

impl<T: RandomNumber + fmt::Debug> Report for Device<T> {
    fn report(&self) -> String {
        match self {
            Device::Socket(socket) => socket.report(),
            Device::Thermometer(thermometer) => thermometer.report(),
        }
    }
}
