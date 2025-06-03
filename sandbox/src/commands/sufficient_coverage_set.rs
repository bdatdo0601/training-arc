use std::{collections::HashSet, io::Error, path::PathBuf};

use crate::utils::parse_json_file;
use log::{debug, info};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TargetCoverage {
    numbers: Vec<u32>,
    min_numbers_to_cover: u32,
}

#[derive(Deserialize, Debug)]
pub struct ItemSet {
    tickets: Vec<Vec<u32>>,
    target_coverage: TargetCoverage,
    expected: bool,
}

#[derive(Deserialize, Debug)]
pub struct SufficientCoverageProblemSet {
    items: Vec<ItemSet>,
}

fn get_combinations(numbers: &Vec<u32>, k: usize) -> Vec<HashSet<u32>> {
    if k > numbers.len() {
        return vec![];
    }
    if k == 1 {
        return numbers
            .iter()
            .map(|&num| HashSet::from([num]))
            .collect::<Vec<HashSet<u32>>>();
    }

    if k == numbers.len() {
        return vec![HashSet::from_iter(numbers.iter().cloned())];
    }

    let mut result = Vec::new();

    let first = numbers[0];
    let rest = &numbers[1..].to_vec();

    // # Combinations that include the first element
    for combo in get_combinations(rest, k - 1) {
        let mut new_combo = HashSet::new();
        new_combo.extend(combo);
        new_combo.insert(first);
        result.push(new_combo);
    }
    // # Combinations that exclude the first element
    let without_first = get_combinations(rest, k);
    result.extend(without_first);

    result
}

/**
 * Checks if a ticket covers the target coverage set.
 *
 * Assumption:
 *  numbers in ticket is unique
 *  numbers in target coverage is unique
 */
fn does_ticket_cover_target(item_set: &ItemSet) -> bool {
    // convert each ticket to a set as we just need to check existence of each number
    let tickets = item_set
        .tickets
        .iter()
        .map(|ticket| HashSet::from_iter(ticket.iter().cloned()))
        .collect::<Vec<HashSet<u32>>>();
    let numbers = item_set.target_coverage.numbers.clone();
    debug!("tickets: {:?}", tickets);
    // generate all permutation of numbers in target coverage with length equal to min_numbers_to_cover
    let combinations = get_combinations(
        &numbers,
        item_set.target_coverage.min_numbers_to_cover as usize,
    );
    debug!("winning combinations: {:?}", combinations);

    let mut combo_covered_count = 0;

    // check if any permutation is covered by any ticket
    for combo in combinations.iter() {
        // if it is a direct subset of any ticket then it is covered
        if tickets.iter().any(|ticket| ticket.is_superset(&combo)) {
            combo_covered_count += 1;
            continue;
        }

        let is_implied_covered = tickets.iter().any(|ticket| {
            // get all the value that is matched in both the ticket and combo
            // we know this is less than min_numbers_to_cover since combo is not a subset of ticket
            let intersected_value: HashSet<u32> =
                HashSet::from_iter(ticket.intersection(&combo).cloned());
            debug!(
                "Intersected value: {:?}, ticket: {:?}",
                intersected_value, ticket
            );
            if intersected_value.is_empty() {
                return false;
            }
            let remaining_numbers = HashSet::from_iter(numbers.iter().cloned())
                .difference(&combo)
                .map(|x| x.clone())
                .collect();
            debug!(
                "Remaining numbers: {:?}, ticket: {:?}",
                remaining_numbers, ticket
            );

            // get all combinations from the remaining numbers in combo with length equal to min_numbers_to_cover - intersected_value.len()
            let remaining_combinations = get_combinations(
                &remaining_numbers,
                item_set.target_coverage.min_numbers_to_cover as usize - intersected_value.len(),
            );
            if remaining_combinations
                .iter()
                .any(|combo| ticket.is_superset(&combo))
            {
                return true;
            }
            false
        });

        if is_implied_covered {
            combo_covered_count += 1;
            continue;
        }
        info!("Uncovered combo: {:?}", combo);
        return false;
    }
    debug!(
        "combo covered count: {}, combination length: {:?}",
        combo_covered_count,
        combinations.len()
    );
    return combo_covered_count == combinations.len();
}

pub fn evaluate_sufficient_coverage(path: &PathBuf) -> Result<(), Error> {
    let test_set = parse_json_file::<SufficientCoverageProblemSet>(path).unwrap();
    for problem in test_set.items {
        info!("Evaluating Problem: {:?}", problem);
        let result = does_ticket_cover_target(&problem);
        info!(
            "Actual Result: {}, Expected Result: {}",
            result, problem.expected
        );
    }
    Ok(())
}
