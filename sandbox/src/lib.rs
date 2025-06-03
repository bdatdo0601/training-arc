use clap::Subcommand;
use log::debug;
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
    }
}
