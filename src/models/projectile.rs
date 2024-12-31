use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::utils::conversions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projectile {
    weight_grains: f64,
    ballistic_coefficient: f64,
    muzzle_velocity: f64,
    caliber: f64,
    length: f64,
}

impl Projectile {
    pub fn new(
        weight_grains: f64,
        bc: f64,
        mv: f64,
        caliber: f64,
        length: f64
    ) -> Result<Self, ProjectileError> {
        // Validate inputs
        if weight_grains <= 0.0 || bc <= 0.0 || mv <= 0.0 || caliber <= 0.0 || length <= 0.0 {
            return Err(ProjectileError::InvalidParameters);
        }
        
        Ok(Self {
            weight_grains,
            ballistic_coefficient: bc,
            muzzle_velocity: mv,
            caliber,
            length,
        })
    }

    pub fn weight_kg(&self) -> f64 {
        conversions::grains_to_kg(self.weight_grains)
    }

    pub fn sectional_density(&self) -> f64 {
        self.weight_grains / (7000.0 * self.caliber.powi(2))
    }

    pub fn initial_velocity_mps(&self) -> f64 {
        conversions::fps_to_mps(self.muzzle_velocity)
    }
}

#[derive(Debug, Error)]
pub enum ProjectileError {
    #[error("Invalid parameters provided")]
    InvalidParameters,
}