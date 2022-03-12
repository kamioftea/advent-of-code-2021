//! This is my solution for [Advent of Code - Day 1 - _Sonar Sweep_](https://adventofcode.com/2021/day/1)
//!
//! The challenge today was to analyse a sequence of integer depths and determine a measure of
//! how often these were increasing.
//!
//! For part one, the challenge was to count the number of values that were greater than the
//! previous value in the list. [`count_increments`] has been written to do that by chaining
//! built-in methods on Rust's iterators.
//!
//! Part two expands on that, requiring the same count be run on the sums of a moving window of
//! three consecutive values over the input. [`sum_windows`] provides a vector of these sums, which
//! is suitable to then be passed to [`count_increments`] to produce the final answer. For this I
//! originally included itertools to use their `izip!` macro to zip three iterators together, each
//! offset by one more. I updated it to use [`slice::windows`] thanks to [@bjgill's](https://github.com/bjgill/advent-of-code-2021/blob/1f086dcb6d5cd9bc1152a9a0db87d16b67d2cdb2/src/bin/day1.rs#L20)
//! comment on the x-gov slack channel.
use std::fs;

/// This is the entry point for the day's puzzle solutions. It will load the input file, parse it
/// into a `Vec<i32>` and pass it to the relevant functions for each part.
pub fn run() {
    let contents = fs::read_to_string("res/day-1-input").expect("Failed to read file");
    let depths = contents
        .lines()
        .flat_map(|line| line.parse::<i32>().ok())
        .collect();

    println!(
        "There are {} steps that increment",
        count_increments(&depths)
    );

    println!(
        "There are {} summed windows that increment",
        count_increments(&sum_windows(&depths))
    );
}

/// Iterate over a moving window of pairs, returning the count where the second number is greater
/// that the first.
///
/// # Example from puzzle specification
/// ```rust
/// let input = vec![
///   199, // N/A - first item
///   200, // yes
///   208, // yes
///   210, // yes
///   200, // no
///   207, // yes
///   240, // yes
///   269, // yes
///   260, // no
///   263  // yes
/// ];
///
/// assert_eq!(count_increments(&input), 7);
/// ```
fn count_increments(depths: &Vec<i32>) -> usize {
    return depths
        .iter()
        // combine with itself, offset by one so that we're iterating over pairs of consecutive
        // values
        .zip(depths.iter().skip(1))
        // include only those that increment
        .filter(|(prev, curr)| curr > prev)
        // return the count of number of entries that increment
        .count();
}

/// Iterate over a moving window of three consecutive items, returning a vector where each item is
/// the sum of te current window.
///
/// # Example from puzzle specification
/// ```rust
/// assert_eq!(
///   sum_windows(&input),
///   vec!(
///     607, // 199 + 200 + 208
///     618, // 200 + 208 + 210
///     618, // 208 + 210 + 200
///     617, // 210 + 200 + 207
///     647, // 200 + 207 + 240
///     716, // 207 + 240 + 269
///     769, // 240 + 269 + 260
///     792  // 269 + 260 + 263
///   )
/// );
/// ```
fn sum_windows(depths: &Vec<i32>) -> Vec<i32> {
    // create the moving window by combining iterators over the input offset by 0, 1, and 2
    return depths
        .windows(3)
        // map those to the sum of the window
        .map(|window| window.iter().sum())
        // and coerce to the expected output type
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::day_1::{count_increments, sum_windows};

    #[test]
    fn can_count_increments() {
        assert_eq!(count_increments(&test_data()), 7)
    }

    fn test_data() -> Vec<i32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    #[test]
    fn can_iterate_windows() {
        assert_eq!(
            sum_windows(&test_data()),
            vec!(607, 618, 618, 617, 647, 716, 769, 792)
        );
        assert_eq!(count_increments(&sum_windows(&test_data())), 5);
    }
}
