use num_traits::FromPrimitive;
use std::fmt;

use crate::report::Report;
use crate::utils::random::{RandomGenerator, SimpleRandomGenerator};
use crate::utils::trait_alias::{Number, RandomNumber};

pub trait Thermometer<TemperatureT: Number> {
    #[must_use]
    fn new(
        initial_temperature: TemperatureT,
        min_temperature_offset: TemperatureT,
        max_temperature_offset: TemperatureT,
    ) -> Self;

    #[must_use]
    fn get_temperature(&mut self) -> TemperatureT;
}

#[derive(Debug)]
pub struct CelsiusThermometer<TemperatureT: RandomNumber = f32> {
    min_temperature_offset: TemperatureT,
    max_temperature_offset: TemperatureT,
    current_temperature: TemperatureT,
    random_offset_generator: SimpleRandomGenerator<TemperatureT>,
}

pub type SmartThermo = CelsiusThermometer<f32>;

impl Default for CelsiusThermometer<f32> {
    fn default() -> Self {
        Self::new(22.0, -5.0, 8.0)
    }
}

impl<TemperatureT> Thermometer<TemperatureT> for CelsiusThermometer<TemperatureT>
where
    TemperatureT: RandomNumber + FromPrimitive + fmt::Debug,
{
    fn new(
        initial_temperature: TemperatureT,
        min_temperature_offset: TemperatureT,
        max_temperature_offset: TemperatureT,
    ) -> Self {
        assert!(min_temperature_offset < max_temperature_offset);

        CelsiusThermometer {
            min_temperature_offset,
            max_temperature_offset,
            current_temperature: initial_temperature,
            random_offset_generator: SimpleRandomGenerator::new(),
        }
    }

    fn get_temperature(&mut self) -> TemperatureT {
        self.current_temperature
            + self
                .random_offset_generator
                .generate(self.min_temperature_offset, self.max_temperature_offset)
    }
}

impl<TemperatureT: RandomNumber + fmt::Debug> Report for CelsiusThermometer<TemperatureT> {
    fn report(&self) -> String {
        format!(
            "Thermometer {{ current: {:?}, offset_min: {:?}, offset_max: {:?} }}",
            self.current_temperature, self.min_temperature_offset, self.max_temperature_offset
        )
    }
}
