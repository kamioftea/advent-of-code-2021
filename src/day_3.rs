//! This is my solution for [Advent of Code - Day 3 - _Binary Diagnostic_](https://adventofcode.com/2021/day/3)
//!
//!

use std::fs;
use std::ops::BitXor;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-3-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 3.
pub fn run() {
    let contents = fs::read_to_string("res/day-3-input").expect("Failed to read file");
    let (data, length) = parse_input(contents);

    let (gamma, epsilon) = analyse_diagnostics(&data, length);
    println!(
        "Gamma: {} x Epsilon: {} = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let (oxygen, co2) = analyse_life_support(&data, length);
    println!("Oxygen: {} x CO2: {} = {}", oxygen, co2, oxygen * co2);
}

fn parse_input(contents: String) -> (Vec<usize>, usize) {
    let data: Vec<usize> = contents
        .lines()
        .map(|line| {
            usize::from_str_radix(line, 2).expect(format!("Unexpected input '{}'", line).as_str())
        })
        .collect();
    let length = contents.lines().next().expect("Input file is empty").len();

    return (data, length);
}

fn count_bit(data: &Vec<usize>, bitmask: usize) -> usize {
    data.iter().filter(|&&value| value & bitmask > 0).count()
}

fn analyse_diagnostics(data: &Vec<usize>, length: usize) -> (usize, usize) {
    let mut gamma: usize = 0;
    let threshold = data.len() / 2;

    for position in (0..length).rev() {
        gamma = (gamma << 1) + (count_bit(data, 1 << position) > threshold) as usize;
    }

    let max = (1 << length) - 1;
    let epsilon = gamma.bitxor(max);

    return (gamma, epsilon);
}

fn analyse_life_support(data: &Vec<usize>, length: usize) -> (usize, usize) {
    fn iter(current: Vec<usize>, position: isize, inverse_keep: bool) -> usize {
        if current.len() == 1 {
            return *(current.get(0).expect("Guaranteed, len == 1"));
        }

        if position == -1 || current.len() == 0 {
            panic!("Non-unique result")
        }

        let bitmask = 1 << position;
        let count = count_bit(&current, bitmask);
        let threshold = (current.len() - 1) / 2;
        let keep = (count > threshold) ^ inverse_keep;

        let filtered: Vec<usize> = current
            .iter()
            .filter(|&&value| ((value & bitmask) == 0) ^ keep)
            .map(|&v| v)
            .collect();
        iter(filtered, position - 1, inverse_keep)
    }

    let oxygen = iter(data.to_vec(), length as isize - 1, false);
    let co2 = iter(data.to_vec(), length as isize - 1, true);
    return (oxygen, co2);
}

#[cfg(test)]
mod tests {
    use crate::day_3::{analyse_diagnostics, analyse_life_support, count_bit, parse_input};

    fn test_data() -> Vec<usize> {
        vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10]
    }

    #[test]
    fn can_parse() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"
                .to_string();

        let (data, length) = parse_input(input);

        assert_eq!(length, 5);
        assert_eq!(data, test_data())
    }

    #[test]
    fn can_count_bits() {
        assert_eq!(count_bit(&test_data(), 1 << 0), 5);
        assert_eq!(count_bit(&test_data(), 1 << 1), 7);
        assert_eq!(count_bit(&test_data(), 1 << 2), 8);
        assert_eq!(count_bit(&test_data(), 1 << 3), 5);
        assert_eq!(count_bit(&test_data(), 1 << 4), 7);
    }

    #[test]
    fn can_analyse_diagnostics() {
        assert_eq!(analyse_diagnostics(&test_data(), 5), (22, 9));
    }

    #[test]
    fn can_analyse_life_support() {
        assert_eq!(analyse_life_support(&test_data(), 5), (23, 10));
    }
}
