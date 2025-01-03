use std::f64;

pub trait DragModel {
    fn calculate_drag(&self, velocity: f64, air_density: f64, ballistic_coefficient: f64) -> f64;
    fn get_cd(&self, mach: f64) -> f64;
}

pub struct G1DragModel;
pub struct G7DragModel;

fn calculate_drag_common(velocity: f64, air_density: f64, ballistic_coefficient: f64, cd: f64) -> f64 {
    let form_factor = 1.0 / (0.0001 + ballistic_coefficient);
    0.5 * air_density * velocity.powi(2) * cd * form_factor
}

impl DragModel for G1DragModel {
    fn calculate_drag(&self, velocity: f64, air_density: f64, ballistic_coefficient: f64) -> f64 {
        let mach = velocity / 1116.4;  // Speed of sound at sea level in ft/s
        let cd = self.get_cd(mach);
        calculate_drag_common(velocity, air_density, ballistic_coefficient, cd)
    }

    fn get_cd(&self, mach: f64) -> f64 {
        if mach <= 0.7 {
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
        }
    }
}

impl DragModel for G7DragModel {
    fn calculate_drag(&self, velocity: f64, air_density: f64, ballistic_coefficient: f64) -> f64 {
        let mach = velocity / 1116.4;
        let cd = self.get_cd(mach);
        calculate_drag_common(velocity, air_density, ballistic_coefficient, cd)
    }

    fn get_cd(&self, mach: f64) -> f64 {
        if mach <= 0.7 {
            0.1198
        } else if mach <= 0.89 {
            0.1198 + (mach - 0.7) * (0.1575 - 0.1198) / (0.89 - 0.7)
        } else if mach <= 1.0 {
            0.1575 + (mach - 0.89) * (0.2255 - 0.1575) / (1.0 - 0.89)
        } else if mach <= 1.1 {
            0.2255 + (mach - 1.0) * (0.1955 - 0.2255) / (1.1 - 1.0)
        } else if mach <= 1.2 {
            0.1955 + (mach - 1.1) * (0.1880 - 0.1955) / (1.2 - 1.1)
        } else {
            0.1880
        }
    }
}