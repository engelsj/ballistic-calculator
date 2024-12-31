use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    temperature: f64,    // Fahrenheit
    pressure: f64,       // inHg
    humidity: f64,       // %
    wind_speed: f64,     // mph
    wind_angle: f64,     // degrees
    altitude: f64,       // feet
    latitude: f64,       // degrees
}

impl Environment {
    pub fn new(
        temperature: f64,
        pressure: f64,
        humidity: f64,
        wind_speed: f64,
        wind_angle: f64,
        altitude: f64,
        latitude: f64,
    ) -> Result<Self, EnvironmentError> {
        // Validate inputs
        if humidity < 0.0 || humidity > 100.0 {
            return Err(EnvironmentError::InvalidHumidity);
        }
        if wind_angle < 0.0 || wind_angle > 360.0 {
            return Err(EnvironmentError::InvalidWindAngle);
        }
        if latitude < -90.0 || latitude > 90.0 {
            return Err(EnvironmentError::InvalidLatitude);
        }

        Ok(Self {
            temperature,
            pressure,
            humidity,
            wind_speed,
            wind_angle,
            altitude,
            latitude,
        })
    }

    pub fn air_density(&self) -> f64 {
        let temp_r = self.temperature + 459.67;  // Convert to Rankine
        let pressure_mb = self.pressure * 33.8639;  // Convert inHg to millibars
        let vapor_pressure = self.calculate_vapor_pressure();
        
        // Density formula from: http://www.shootingsoftware.com/atmospheric.htm
        (pressure_mb - (0.3783 * vapor_pressure)) / (1718.0 * (temp_r / 518.67))
    }

    fn calculate_vapor_pressure(&self) -> f64 {
        let temp_c = (self.temperature - 32.0) * 5.0 / 9.0;
        let es = 6.11 * 10.0_f64.powf((7.5 * temp_c) / (237.3 + temp_c));
        (self.humidity / 100.0) * es
    }
}

#[derive(Debug, Error)]
pub enum EnvironmentError {
    #[error("Humidity must be between 0 and 100")]
    InvalidHumidity,
    #[error("Wind angle must be between 0 and 360")]
    InvalidWindAngle,
    #[error("Latitude must be between -90 and 90")]
    InvalidLatitude,
}
