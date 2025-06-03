use log::debug;
use serde::{Deserialize, Serialize};

use crate::utils::parse_json_file;
use std::{io::Error, path::PathBuf}; // path buffer, to construct paths

#[derive(Serialize, Deserialize, Debug)]
pub struct ScheduleItem {
    start: i32,
    end: i32,
    title: String,
}

impl Clone for ScheduleItem {
    fn clone(&self) -> Self {
        ScheduleItem {
            start: self.start,
            end: self.end,
            title: self.title.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schedule {
    items: Vec<ScheduleItem>,
}

pub fn get_optimal_schedule(path: &PathBuf) -> Result<(), Error> {
    let schedule = parse_json_file::<Schedule>(path).unwrap();
    debug!("Extracted schedule: {:?}", schedule);
    let mut optimal_schedule_items: Vec<ScheduleItem> = Vec::new();
    let mut schedule_items = schedule.items.clone();

    while schedule_items.len() > 0 {
        let mut earliest_end = i32::MAX;
        let mut earliest_index = 0;

        for (i, item) in schedule_items.iter().enumerate() {
            if item.end < earliest_end {
                earliest_end = item.end;
                earliest_index = i;
            }
        }
        optimal_schedule_items.push(schedule_items.remove(earliest_index));
        let picked_item = optimal_schedule_items.last().unwrap();

        schedule_items = schedule_items
            .iter()
            .filter(|item| item.start >= picked_item.end)
            .cloned()
            .collect();
    }

    let optimal_schedule = Schedule {
        items: optimal_schedule_items,
    };

    let result = serde_json::to_string_pretty(&optimal_schedule)?;

    println!("{}", result);
    Ok(())
}
