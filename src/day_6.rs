//! This is my solution for [Advent of Code - Day 6 - _Lanternfish_](https://adventofcode.com/2021/day/6)
//!
//!

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

fn parse_input(input: String) -> [usize; 9] {
    let fish: Vec<usize> = input
        .trim()
        .split(',')
        .flat_map(|num| num.parse::<usize>().ok())
        .collect();
    let mut fish_population = [0usize; 9usize];
    for f in fish {
        fish_population[f] = fish_population[f] + 1;
    }

    fish_population
}

pub fn simulate(fish_pops: [usize; 9], days: usize) -> [usize; 9] {
    if days == 0 {
        return fish_pops;
    }

    let mut new_pops = [0usize; 9usize];
    for i in 1..=8 {
        new_pops[i - 1] = fish_pops[i];
    }

    new_pops[6] = new_pops[6] + fish_pops[0];
    new_pops[8] = fish_pops[0];

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
