use eframe::egui;
use eframe::egui::ViewportBuilder;
use ballistic_calculator::{
    Projectile,
    Environment,
    TrajectoryCalculator,
    TrajectoryPoint,
    G1DragModel,
    G7DragModel,
    DragModel,
};

struct BallisticCalculatorApp {
    // Input states
    projectile_weight: String,
    ballistic_coefficient: String,
    muzzle_velocity: String,
    caliber: String,
    bullet_length: String,
    temperature: String,
    pressure: String,
    humidity: String,
    wind_speed: String,
    wind_angle: String,
    altitude: String,
    latitude: String,
    range: String,
    drag_model: DragModelType,
    
    // Results
    calculation_results: Option<Vec<TrajectoryPoint>>,
    error_message: Option<String>,
}

#[derive(PartialEq)]
enum DragModelType {
    G1,
    G7,
}

impl Default for BallisticCalculatorApp {
    fn default() -> Self {
        Self {
            projectile_weight: "168.0".to_string(),
            ballistic_coefficient: "0.223".to_string(),
            muzzle_velocity: "2750.0".to_string(),
            caliber: "0.308".to_string(),
            bullet_length: "1.2".to_string(),
            temperature: "59.0".to_string(),
            pressure: "29.92".to_string(),
            humidity: "78.0".to_string(),
            wind_speed: "10.0".to_string(),
            wind_angle: "90.0".to_string(),
            altitude: "1000.0".to_string(),
            latitude: "45.0".to_string(),
            range: "1000.0".to_string(),
            drag_model: DragModelType::G1,
            calculation_results: None,
            error_message: None,
        }
    }
}

impl eframe::App for BallisticCalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Left panel - Inputs
                ui.vertical(|ui| {
                    ui.heading("Inputs");
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.heading("Projectile Data");
                            ui.horizontal(|ui| {
                                ui.label("Weight (grains):");
                                ui.text_edit_singleline(&mut self.projectile_weight);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Ballistic Coefficient:");
                                ui.text_edit_singleline(&mut self.ballistic_coefficient);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Muzzle Velocity (fps):");
                                ui.text_edit_singleline(&mut self.muzzle_velocity);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Caliber (inches):");
                                ui.text_edit_singleline(&mut self.caliber);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Bullet Length (inches):");
                                ui.text_edit_singleline(&mut self.bullet_length);
                            });
                        });
                    });

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.heading("Environmental Data");
                            ui.horizontal(|ui| {
                                ui.label("Temperature (°F):");
                                ui.text_edit_singleline(&mut self.temperature);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Pressure (inHg):");
                                ui.text_edit_singleline(&mut self.pressure);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Humidity (%):");
                                ui.text_edit_singleline(&mut self.humidity);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Wind Speed (mph):");
                                ui.text_edit_singleline(&mut self.wind_speed);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Wind Angle (deg):");
                                ui.text_edit_singleline(&mut self.wind_angle);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Altitude (ft):");
                                ui.text_edit_singleline(&mut self.altitude);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Latitude (deg):");
                                ui.text_edit_singleline(&mut self.latitude);
                            });
                        });
                    });

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.heading("Calculation Settings");
                            ui.horizontal(|ui| {
                                ui.label("Range (yards):");
                                ui.text_edit_singleline(&mut self.range);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Drag Model:");
                                ui.radio_value(&mut self.drag_model, DragModelType::G1, "G1");
                                ui.radio_value(&mut self.drag_model, DragModelType::G7, "G7");
                            });
                        });
                    });

                    if ui.button("Calculate").clicked() {
                        self.calculate_trajectory();
                    }

                    if let Some(error) = &self.error_message {
                        ui.colored_label(egui::Color32::RED, error);
                    }
                });

                // Right panel - Results
                ui.vertical(|ui| {
                    ui.heading("Results");
                    if let Some(results) = &self.calculation_results {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.group(|ui| {
                                for point in results.iter().step_by(100) {
                                    ui.label(format!(
                                        "Distance: {:.1} yards\nDrop: {:.1} inches\nWindage: {:.1} inches\nVelocity: {:.0} fps\nTime: {:.3} sec\n",
                                        point.distance,
                                        point.drop,
                                        point.windage,
                                        point.velocity,
                                        point.time
                                    ));
                                    ui.separator();
                                }
                            });
                        });
                    }
                });
            });
        });
    }
}

impl BallisticCalculatorApp {
    fn calculate_trajectory(&mut self) {
        self.error_message = None;
        
        let parse_input = |s: &str, field: &str| -> Result<f64, String> {
            s.parse::<f64>().map_err(|_| format!("Invalid {} value", field))
        };

        let result = (|| -> Result<Vec<TrajectoryPoint>, String> {
            // Parse projectile data
            let projectile_weight = parse_input(&self.projectile_weight, "projectile weight")?;
            let ballistic_coefficient = parse_input(&self.ballistic_coefficient, "ballistic coefficient")?;
            let muzzle_velocity = parse_input(&self.muzzle_velocity, "muzzle velocity")?;
            let caliber = parse_input(&self.caliber, "caliber")?;
            let bullet_length = parse_input(&self.bullet_length, "bullet length")?;
            
            let projectile = Projectile::new(
                projectile_weight,
                ballistic_coefficient,
                muzzle_velocity,
                caliber,
                bullet_length,
            ).map_err(|e| e.to_string())?;

            // Parse environmental data
            let temperature = parse_input(&self.temperature, "temperature")?;
            let pressure = parse_input(&self.pressure, "pressure")?;
            let humidity = parse_input(&self.humidity, "humidity")?;
            let wind_speed = parse_input(&self.wind_speed, "wind speed")?;
            let wind_angle = parse_input(&self.wind_angle, "wind angle")?;
            let altitude = parse_input(&self.altitude, "altitude")?;
            let latitude = parse_input(&self.latitude, "latitude")?;
            
            let environment = Environment::new(
                temperature,
                pressure,
                humidity,
                wind_speed,
                wind_angle,
                altitude,
                latitude,
            ).map_err(|e| e.to_string())?;

            // Create drag model
            let drag_model: Box<dyn DragModel> = match self.drag_model {
                DragModelType::G1 => Box::new(G1DragModel),
                DragModelType::G7 => Box::new(G7DragModel),
            };

            // Create calculator and compute trajectory
            let calculator = TrajectoryCalculator::new(
                projectile,
                environment,
                drag_model
            );

            let range = parse_input(&self.range, "range")?;
            Ok(calculator.calculate_trajectory(range, 0.01))
        })();

        match result {
            Ok(trajectory) => {
                self.calculation_results = Some(trajectory);
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(e);
                self.calculation_results = None;
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Ballistic Calculator"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Ballistic Calculator",
        options,
        Box::new(|_cc| Box::new(BallisticCalculatorApp::default())),
    )
}