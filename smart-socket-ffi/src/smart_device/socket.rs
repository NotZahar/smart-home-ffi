use num_traits::FromPrimitive;
use std::fmt;

use crate::report::Report;
use crate::utils::random::{RandomGenerator, SimpleRandomGenerator};
use crate::utils::trait_alias::{Number, RandomNumber};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocketState {
    On = 1,
    Off = 0,
}

pub trait Socket<PowerT: Number> {
    const DEFAULT_INACTIVE_POWER: PowerT;

    #[must_use]
    fn new(default_active_power: PowerT, max_power_offset: PowerT) -> Self;

    fn turn_on(&mut self);

    fn turn_off(&mut self);

    #[must_use]
    fn get_state(&self) -> SocketState;

    #[must_use]
    fn get_power(&mut self) -> PowerT;
}

#[derive(Debug)]
pub struct PowerSocket<PowerT: RandomNumber = f32> {
    state: SocketState,
    max_power_offset: PowerT,
    default_active_power: PowerT,
    random_offset_generator: SimpleRandomGenerator<PowerT>,
}

pub type SmartSocket = PowerSocket<f32>;

impl Default for PowerSocket<f32> {
    fn default() -> Self {
        Self::new(100.0, 20.0)
    }
}

impl<PowerT> Socket<PowerT> for PowerSocket<PowerT>
where
    PowerT: RandomNumber + FromPrimitive + fmt::Debug,
{
    const DEFAULT_INACTIVE_POWER: PowerT = PowerT::ZERO;

    fn new(default_active_power: PowerT, max_power_offset: PowerT) -> Self {
        assert!(Self::DEFAULT_INACTIVE_POWER <= max_power_offset);

        PowerSocket {
            state: SocketState::Off,
            max_power_offset,
            default_active_power,
            random_offset_generator: SimpleRandomGenerator::new(),
        }
    }

    fn turn_on(&mut self) {
        self.state = SocketState::On;
    }

    fn turn_off(&mut self) {
        self.state = SocketState::Off;
    }

    fn get_state(&self) -> SocketState {
        self.state
    }

    fn get_power(&mut self) -> PowerT {
        match self.state {
            SocketState::On => {
                self.default_active_power
                    + self
                        .random_offset_generator
                        .generate(Self::DEFAULT_INACTIVE_POWER, self.max_power_offset)
            }
            SocketState::Off => Self::DEFAULT_INACTIVE_POWER,
        }
    }
}

impl<PowerT: RandomNumber + fmt::Debug> Report for PowerSocket<PowerT> {
    fn report(&self) -> String {
        format!(
            "Socket {{ state: {:?}, default_power: {:?}, max_offset: {:?} }}",
            self.state, self.default_active_power, self.max_power_offset
        )
    }
}
