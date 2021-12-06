//! This is my solution for [Advent of Code - Day 6 - _Lanternfish_](https://adventofcode.com/2021/day/6)
//!
//! There is always at least one puzzle each year where a naive implementation is fairly simple, but
//! part two expands the problem size so that the naive solution will take too long to run. This is
//! one of those.
//!
//! Experience from previous years allowed me to spot that and implement a more performant solution
//! to part one, [`simulate`]. This requires the population count for each day, so there is also
//! [`parse_input`] that reduces the puzzle input to this format. Part two calls [`simulate`] again,
//! but with a higher number of days.

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-6-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 6.
pub fn run() {
    let contents = fs::read_to_string("res/day-6-input").expect("Failed to read file");
    let fish_pops = parse_input(contents);

    let part_1_pop = simulate(fish_pops, 80).iter().sum::<usize>();
    println!("Population count after 80 days: {}", part_1_pop);

    let part_2_pop = simulate(fish_pops, 256).iter().sum::<usize>();
    println!("Population count after 256 days: {}", part_2_pop);
}

/// Reduces a comma-separated list of numbers representing the number of days until that fish will
/// next reproduce, into a summary array that contains the count for each day.
fn parse_input(input: String) -> [usize; 9] {
    // parse the initial input to a list of `usize`
    let fish: Vec<usize> = input
        .trim()
        .split(',')
        .flat_map(|num| num.parse::<usize>().ok())
        .collect();

    // iterate through the fish, incrementing the relevant count for each one.
    let mut fish_population = [0usize; 9usize];
    for f in fish {
        fish_population[f] = fish_population[f] + 1;
    }

    fish_population
}

/// Recursive function that iterates the population `days` times, returning the resulting
/// population summary.
pub fn simulate(fish_pops: [usize; 9], days: usize) -> [usize; 9] {
    // base case - return the current population
    if days == 0 {
        return fish_pops;
    }

    // otherwise copy each of the populations is moved one day earlier
    let mut new_pops = [0usize; 9usize];
    for i in 1..=8 {
        new_pops[i - 1] = fish_pops[i];
    }

    // Fish in the 0-day population reproduce - creating an equal number of fish that will reproduce
    // in 9 days.
    new_pops[8] = fish_pops[0];
    // These fish also reset, and will reproduce again themselves in 7 days.
    new_pops[6] = new_pops[6] + fish_pops[0];

    // Recursively call, decrementing the remaining days
    return simulate(new_pops, days - 1);
}

#[cfg(test)]
mod tests {
    use crate::day_6::{parse_input, simulate};

    #[test]
    fn can_parse() {
        assert_eq!(
            parse_input("3,4,3,1,2".to_string()),
            [0, 1, 1, 2, 1, 0, 0, 0, 0]
        );
    }

    #[test]
    fn can_simulate() {
        assert_eq!(
            simulate([0, 1, 1, 2, 1, 0, 0, 0, 0], 1),
            [1, 1, 2, 1, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            simulate([0, 1, 1, 2, 1, 0, 0, 0, 0], 2),
            [1, 2, 1, 0, 0, 0, 1, 0, 1]
        );
        assert_eq!(
            simulate([0, 1, 1, 2, 1, 0, 0, 0, 0], 18)
                .iter()
                .sum::<usize>(),
            26
        );
        assert_eq!(
            simulate([0, 1, 1, 2, 1, 0, 0, 0, 0], 80)
                .iter()
                .sum::<usize>(),
            5934
        );
        assert_eq!(
            simulate([0, 1, 1, 2, 1, 0, 0, 0, 0], 256)
                .iter()
                .sum::<usize>(),
            26984457539
        );
    }
}
