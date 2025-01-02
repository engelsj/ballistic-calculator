use std::f64;

pub trait DragModel {
    fn calculate_drag(&self, velocity: f64, air_density: f64) -> f64;
}

pub struct G1DragModel;
pub struct G7DragModel;

impl DragModel for G1DragModel {
    fn calculate_drag(&self, velocity: f64, air_density: f64) -> f64 {
        let mach = velocity / 1116.4;  // Speed of sound at sea level in ft/s
        
        // G1 drag coefficient table (representative values)
        let cd = if mach <= 0.7 {
            0.225
        } else if mach <= 0.89 {
            0.225 + (mach - 0.7) * (0.275 - 0.225) / (0.89 - 0.7)
        } else if mach <= 1.0 {
            0.275 + (mach - 0.89) * (0.425 - 0.275) / (1.0 - 0.89)
        } else if mach <= 1.1 {
            0.425 + (mach - 1.0) * (0.295 - 0.425) / (1.1 - 1.0)
        } else if mach <= 1.2 {
            0.295 + (mach - 1.1) * (0.280 - 0.295) / (1.2 - 1.1)
        } else {
            0.280
        };

        let form_factor = 1.0 / (0.0001 + self.ballistic_coefficient);
        0.5 * air_density * velocity.powi(2) * cd * form_factor
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