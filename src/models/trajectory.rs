use serde::{Serialize, Deserialize};
use nalgebra as na;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryPoint {
    pub distance: f64,    // yards
    pub drop: f64,        // inches
    pub windage: f64,     // inches
    pub velocity: f64,    // ft/s
    pub energy: f64,      // ft-lbs
    pub time: f64,        // seconds
}

impl TrajectoryPoint {
    pub fn new(
        distance: f64,
        drop: f64,
        windage: f64,
        velocity: f64,
        energy: f64,
        time: f64,
    ) -> Self {
        Self {
            distance,
            drop,
            windage,
            velocity,
            energy,
            time,
        }
    }

    pub fn moa_adjustment(&self) -> (f64, f64) {
        let drop_moa = (self.drop / (self.distance * 1.047)) * 100.0;
        let windage_moa = (self.windage / (self.distance * 1.047)) * 100.0;
        (drop_moa, windage_moa)
    }

    pub fn mil_adjustment(&self) -> (f64, f64) {
        let drop_mil = (self.drop / (self.distance * 36.0)) * 1000.0;
        let windage_mil = (self.windage / (self.distance * 36.0)) * 1000.0;
        (drop_mil, windage_mil)
    }
}