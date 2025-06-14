use clap::{value_parser, Subcommand};
use log::{debug, info};
use std::path::PathBuf; // path buffer, to construct paths

mod commands;
mod utils;

use commands::*;

#[derive(Subcommand)]
pub enum Commands {
    /// Optimal Scheduling of picking tasks with overlapping intervals
    Schedule {
        /// Path to the JSON file containing the tasks in the form of
        /// { "tasks": [
        ///     { "start": 1, "end": 3, "name": "Task 1" },
        ///     { "start": 2, "end": 4, "name": "Task 2" }
        /// ] }
        #[arg(short, long)]
        json_file_path: PathBuf,
    },
    /// Sufficient Coverage Set Problem
    SufficientCoverageSet {
        /// Path to the JSON file containing the items in the form of
        /// { "items": [
        ///     { "tickets": [[1, 2], [2, 3], [3, 4]], "target_coverage": { "numbers": [1, 2, 3, 4], "min_numbers_to_cover": 2 }, "expected": true },
        ///     { "tickets": [[1, 2], [2, 3], [3, 4]], "target_coverage": { "numbers": [1, 2, 3, 4], "min_numbers_to_cover": 3 }, "expected": false }
        /// ] }
        #[arg(short, long)]
        json_file_path: PathBuf,
    },
    DailyTemperature {
        /// numbers containing the temperatures separated by commas
        /// 33, 74, 75, 71, 69, 72, 76, 73
        #[arg(short, long, required = true, value_delimiter = ',', value_parser = value_parser!(i32))]
        temperatures: Vec<i32>,

        /// algorithm mode: either "stack" or "reverse"
        #[arg(short, long)]
        mode: String,
    },
    /// Rotating a list by k rotations
    RotatingList {
        /// numbers containing the temperatures separated by commas
        /// 33, 74, 75, 71, 69, 72, 76, 73
        #[arg(short, long, required = true, value_delimiter = ',', value_parser = value_parser!(i32))]
        numbers: Vec<i32>,

        /// algorithm mode: either "stack" or "reverse"
        #[arg(short, long, value_parser = value_parser!(i32))]
        k: i32,

        /// algorithm mode: either "Right" or "Left"
        #[arg(short, long, default_value = "right")]
        rotation_direction: String,
    },
}

impl ToString for Commands {
    fn to_string(&self) -> String {
        match self {
            Commands::Schedule { json_file_path } => {
                format!("Schedule {{ json_file_path: {:?} }}", json_file_path)
            }
            Commands::SufficientCoverageSet { json_file_path } => {
                format!(
                    "SufficientCoverageSet {{ json_file_path: {:?} }}",
                    json_file_path
                )
            }
            Commands::DailyTemperature { temperatures, mode } => {
                format!(
                    "DailyTemperature {{ temperatures: {:?}, mode: {:?} }}",
                    temperatures, mode
                )
            }
            Commands::RotatingList {
                numbers,
                k,
                rotation_direction,
            } => {
                format!(
                    "RotatingList {{ numbers: {:?}, k: {:?}, rotation_direction: {:?} }}",
                    numbers, k, rotation_direction
                )
            }
        }
    }
}

pub fn run_command(cmd: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Commands::Schedule { json_file_path } => {
            // Implement the scheduling algorithm here
            debug!("Scheduling tasks from {}", json_file_path.display());
            schedule::get_optimal_schedule(&json_file_path)?;
            Ok(())
        }
        Commands::SufficientCoverageSet { json_file_path } => {
            // Implement the sufficient coverage set algorithm here
            debug!(
                "Finding sufficient coverage set from {}",
                json_file_path.display()
            );
            sufficient_coverage_set::evaluate_sufficient_coverage(&json_file_path)?;
            Ok(())
        }
        Commands::DailyTemperature { temperatures, mode } => {
            // Implement the daily temperature algorithm here
            info!(
                "Finding daily temperature from {:?} using {:?} method",
                temperatures, mode
            );
            let time_elapsed = std::time::Instant::now();
            let result = daily_temperature::get_daily_temperature(&temperatures, &mode)?;
            debug!("Time elapsed: {:?}", time_elapsed.elapsed());
            info!("Result: {:?}", result);
            Ok(())
        }
        Commands::RotatingList {
            numbers,
            k,
            rotation_direction,
        } => {
            // Implement the rotating list algorithm here
            info!("Rotating list from {:?} by {:?} rotations", numbers, k);
            let time_elapsed = std::time::Instant::now();
            let result = rotating_list::rotating_list(numbers, k, &rotation_direction)?;
            debug!("Time elapsed: {:?}", time_elapsed.elapsed());
            info!("Result: {:?}", result);
            Ok(())
        }
    }
}
