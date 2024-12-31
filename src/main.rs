use ballistic_calculator::{
    Projectile,
    Environment,
    TrajectoryCalculator,
    G1DragModel,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let projectile = Projectile::new(
        168.0,  // 168 grain
        0.223,  // G1 BC
        2750.0, // 2750 fps muzzle velocity
        0.308,  // .308 caliber
        1.2,    // 1.2 inch length
    )?;

    let environment = Environment::new(
        59.0,   // 59°F
        29.92,  // 29.92 inHg pressure
        78.0,   // 78% humidity
        10.0,   // 10 mph wind
        90.0,   // 90° wind angle (full value)
        1000.0, // 1000 ft altitude
        45.0,   // 45° latitude
    )?;

    let calculator = TrajectoryCalculator::new(
        projectile,
        environment,
        Box::new(G1DragModel)
    );

    let trajectory = calculator.calculate_trajectory(1000.0, 0.01);
    
    // Print results
    for point in trajectory.iter().step_by(100) {
        println!(
            "Distance: {:.1} yards, Drop: {:.1} inches, Windage: {:.1} inches, \
             Velocity: {:.0} fps, Time: {:.3} sec",
            point.distance,
            point.drop,
            point.windage,
            point.velocity,
            point.time
        );
    }

    Ok(())
}