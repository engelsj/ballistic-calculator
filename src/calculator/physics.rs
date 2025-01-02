use crate::models::{Projectile, Environment, TrajectoryPoint};
use crate::calculator::drag::DragModel;
use nalgebra as na;

pub struct TrajectoryCalculator {
    projectile: Projectile,
    environment: Environment,
    drag_model: Box<dyn DragModel>,
}

impl TrajectoryCalculator {
    pub fn new(
        projectile: Projectile,
        environment: Environment,
        drag_model: Box<dyn DragModel>
    ) -> Self {
        Self {
            projectile,
            environment,
            drag_model,
        }
    }
    
    pub fn calculate_trajectory(
        &self,
        range_yards: f64,
        step_size: f64
    ) -> Vec<TrajectoryPoint> {
        let mut points = Vec::new();
        let g = 32.174;  // Acceleration due to gravity in ft/s²
        
        // Initial conditions
        let mut x = 0.0;  // Distance traveled (feet)
        let mut y = 0.0;  // Height (feet)
        let mut vx = self.projectile.muzzle_velocity;  // Initial velocity x component (ft/s)
        let mut vy = 0.0;  // Initial velocity y component (ft/s)
        let mut time = 0.0;

        // Convert range to feet
        let range_feet = range_yards * 3.0;
        
        // Wind components
        let wind_angle_rad = self.environment.wind_angle * std::f64::consts::PI / 180.0;
        let wind_vx = self.environment.wind_speed * wind_angle_rad.cos();
        let wind_vy = self.environment.wind_speed * wind_angle_rad.sin();

        while x <= range_feet && y >= -1000.0 {  // Stop if drop exceeds 1000 feet
            // Calculate current velocity magnitude relative to air
            let v_rel_x = vx - wind_vx;
            let v_rel_y = vy - wind_vy;
            let v_total = (v_rel_x * v_rel_x + v_rel_y * v_rel_y).sqrt();

            // Calculate drag force
            let air_density = self.environment.air_density();
            let drag = self.drag_model.calculate_drag(v_total, air_density);
            
            // Calculate acceleration components
            let ax = -(drag * v_rel_x) / (v_total * self.projectile.weight_grains);
            let ay = -g - (drag * v_rel_y) / (v_total * self.projectile.weight_grains);

            // Update velocities (RK4 would be more accurate, but this is sufficient for now)
            vx += ax * step_size;
            vy += ay * step_size;

            // Update positions
            x += vx * step_size;
            y += vy * step_size;
            time += step_size;

            // Record point (converting back to yards/inches)
            points.push(TrajectoryPoint::new(
                x / 3.0,             // Convert feet to yards
                y * 12.0,            // Convert feet to inches
                (wind_vy * time) * 12.0,  // Windage in inches
                v_total,             // Velocity in ft/s
                0.5 * (self.projectile.weight_grains / 7000.0) * v_total * v_total / 32.174,  // Energy in ft-lbs
                time
            ));
        }
        
        points
    }
}