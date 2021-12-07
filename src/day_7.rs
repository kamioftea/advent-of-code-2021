//! This is my solution for [Advent of Code - Day 7 - _Title_](https://adventofcode.com/2021/day/7)
//!
//!

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
