

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
/// Standard error for the interface
pub enum Error {
    /// Invalid data as an input
    #[error("Data given is not in floating point form, try adding a decimal point")]
    InvalidInput,
    /// Air temp out of range
    #[error("Air Temperature is outside of the range -50 < 0 < 50")]
    InvalidAirTemperature,
    /// Radiant temp out of range input
    #[error("Radiant Temperature is outside of the range -50 < 0 < 50")]
    InvalidRadiantTemperature,
    /// Humidity out of range input
    #[error("Humidity is out of range (0, 100)")]
    InvalidHumidity,
    /// Wind Speed out of range input
    #[error("Windspeed is out of range (0, 17)")]
    InvalidWindSpeed
}

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        match self {
            _ => io::Error::new(io::ErrorKind::Other, format!("{}", self)),
        }
    }
}