use clap::Subcommand;
use log::debug;
use std::path::PathBuf; // path buffer, to construct paths

mod commands;

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
}

impl ToString for Commands {
    fn to_string(&self) -> String {
        match self {
            Commands::Schedule { json_file_path } => {
                format!("Schedule {{ json_file_path: {:?} }}", json_file_path)
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
    }
}
