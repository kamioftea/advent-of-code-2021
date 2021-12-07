//! This is my solution for [Advent of Code - Day 7 - _The Treachery of Whales_](https://adventofcode.com/2021/day/7)
//!
//! Today's input was a bunch of 🦀 crabs 🦀 (in submarines ) who can only move sideways (despite
//! being in submarines 🤷.) The challenge was to find the most efficient position to align them to,
//! given a particular cost function for a crab submarine to move a certain distance.
//!
//! For part one this was a linear cost, which reduces to finding the median, and calculating the
//! cost for each crab to move to that median. This is implemented by [`find_distance_to_median`].
//! As an informal proof for this method consider: for any given position value, incrementing that
//! position by one increases the overall cost by one per crab below that value, and decreases it by
//! one per crab above that value. Decrementing the position has the opposite effect. For any crab
//! other than the median, if you move the value towards the median, more crabs are on the side that
//! decreases the cost (by definition), and so the overall cost decreases. An interesting note in
//! the case of an even number of crabs, the value of either crab in the middle pair, and any values
//! between them will produce the same optimal cost, so in that case we can just pick one
//! arbitrarily.
//!
//! For part two the cost function was the triangular number of the distance, `1 => 1`, `2 => 3`,
//! `3 => 6`, ... or `(n * (n+1) ) / 2`. This is implemented by [`find_triangular_distance_to_mean`].
//! I have to admit I guessed the mean here. It seems reasonable - if the cost increases as you move
//! further you want a lot of crabs to move the shortest distance they can. Given there was an issue
//! where the rounding was slightly off (rounding the calculated mean worked for the sample, but
//! floor was needed for the puzzle input) I suspect the mean is the limit as the steepness of the
//! cost function increases, and it's close enough (±1) for triangular distance. But equally that
//! may just be a weirdness of integer maths. If anyone has information on more concrete theory
//! about this I'd be interested in a link.

use std::cmp::min;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-7-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 7.
pub fn run() {
    let contents = fs::read_to_string("res/day-7-input").expect("Failed to read file");
    let positions: Vec<usize> = contents
        .trim()
        .split(',')
        .flat_map(|pos| pos.parse())
        .collect();

    let total_fuel = find_distance_to_median(&positions);
    println!("Total fuel to align - linear: {}", total_fuel);

    let total_fuel = find_triangular_distance_to_mean(&positions);
    println!("Total fuel to align - triangular: {}", total_fuel);
}

/// First find the median by sorting the list and taking the value at the midpoint. As discussed in
/// the summary, either midpoint is fine in the case of an even length list, so just use the default
/// rounding. Secondly iterate through the list to total the distance to the median and sum those
/// values.
fn find_distance_to_median(positions: &Vec<usize>) -> usize {
    let mut sorted = positions.to_vec();
    sorted.sort();
    let mid = sorted.len() / 2;
    let &median = sorted.get(mid).unwrap();

    positions
        .iter()
        .map(|&pos| (pos as isize - median as isize).abs() as usize)
        .sum()
}

/// Very similar to [`find_distance_to_median`] with three differences:
/// - Calculate mean instead of median for the target position
/// - Map the resulting fuel cost using the triangular number distance
/// - Calculate the total for the integer values both sides of the mean and take the lowest (see
///   main description)
fn find_triangular_distance_to_mean(positions: &Vec<usize>) -> usize {
    let mean = (positions.iter().sum::<usize>() as f64 / positions.len() as f64).floor() as usize;

    min(
        positions
            .iter()
            .map(|&pos| (pos as isize - mean as isize).abs() as usize)
            .map(|distance| (distance * (distance + 1)) / 2)
            .sum(),
        positions
            .iter()
            .map(|&pos| (pos as isize - (mean as isize + 1)).abs() as usize)
            .map(|distance| (distance * (distance + 1)) / 2)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use crate::day_7::{find_distance_to_median, find_triangular_distance_to_mean};

    #[test]
    fn can_find_distance_to_median() {
        assert_eq!(
            find_distance_to_median(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]),
            37
        )
    }

    #[test]
    fn can_find_triangular_distance_to_mean() {
        assert_eq!(
            find_triangular_distance_to_mean(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]),
            168
        )
    }
}
