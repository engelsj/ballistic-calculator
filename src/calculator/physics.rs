use crate::models::{Projectile, Environment, TrajectoryPoint};
use crate::calculator::drag::DragModel;
use crate::utils::conversions;
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
        let mut position = na::Vector3::new(0.0, 0.0, 0.0);
        let mut velocity = na::Vector3::new(
            self.projectile.initial_velocity_mps(),
            0.0,
            0.0
        );
        let mut time = 0.0;
        let g = na::Vector3::new(0.0, 0.0, -9.81);  // Gravity in m/s²
        
        while position.x <= conversions::yards_to_meters(range_yards) {
            let speed = velocity.magnitude();
            let drag_force = self.drag_model.calculate_drag(
                conversions::mps_to_fps(speed),
                self.environment.air_density()
            );
            
            // Update velocity and position using RK4 integration
            let drag_accel = -drag_force / self.projectile.weight_kg() * velocity.normalize();
            let total_accel = g + drag_accel;
            
            velocity += total_accel * step_size;
            position += velocity * step_size;
            time += step_size;
            
            points.push(TrajectoryPoint::new(
                conversions::meters_to_yards(position.x),
                conversions::meters_to_inches(position.z),
                conversions::meters_to_inches(position.y),
                conversions::mps_to_fps(speed),
                0.5 * self.projectile.weight_kg() * speed.powi(2),
                time,
            ));
        }
        
        points
    }
}