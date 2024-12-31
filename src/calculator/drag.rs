use std::f64;

pub trait DragModel {
    fn calculate_drag(&self, velocity: f64, air_density: f64) -> f64;
}

pub struct G1DragModel;
pub struct G7DragModel;

impl DragModel for G1DragModel {
    fn calculate_drag(&self, velocity: f64, air_density: f64) -> f64 {
        let mach = velocity / 1116.4;  // Speed of sound at sea level in ft/s
        let cd = if mach < 0.7 {
            0.2323 + (0.2323 * mach.powi(2))
        } else {
            0.2323 + (0.2323 * mach.powi(7))
        };
        0.5 * air_density * velocity.powi(2) * cd
    }
}

impl DragModel for G7DragModel {
    fn calculate_drag(&self, velocity: f64, air_density: f64) -> f64 {
        let mach = velocity / 1116.4;
        let cd = if mach < 0.7 {
            0.1198 + (0.1198 * mach.powi(2))
        } else {
            0.1198 + (0.1198 * mach.powi(7))
        };
        0.5 * air_density * velocity.powi(2) * cd
    }
}