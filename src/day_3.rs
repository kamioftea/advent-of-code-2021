//! This is my solution for [Advent of Code - Day 3 - _Binary Diagnostic_](https://adventofcode.com/2021/day/3)
//!
//! Today the task was to analyse a sequence of binary data with a bunch of bitwise operators. I
//! ended up reworking both parts after I originally got them working with more convoluted code.
//!
//! [`parse_input`] is used to covert the binary strings into numbers, and also returns the number
//! of bits per string as that is needed for some of the bitwise tricks later.
//! [`analyse_diagnostics`] solves part one, deferring some logic to [`count_bit`]. Originally this
//! was a double for loop over data and bit position, storing the counts into a mutable Vec<usize>.
//! I needed [`count_bits`] for my original solution to part two, and once written I refactored
//! [`analyse_diagnostics`] to use it as well. When I later refactored [`analyse_life_support`] to
//! no longer need [`count_bits`], [`analyse_diagnostics`] was still cleaner when using
//! [`count_bits`] so I left it as is.
//!
//! [`analyse_life_support`] solves part two. Originally it used [`count_bits`] to determine if
//! the bits at the current position were majority set or not, then filtered the current subset
//! based on that. The current partition based approach is easier to understand what is going on.

use itertools::partition;
use std::fs;

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

/// Returns a pair of the parsed data and the length of the bit strings. Delegates to the built in
/// [`usize::from_str_radix`]. The length is needed for some of the bitwise tricks.
///
/// # Example from puzzle specification
/// ```rust
/// let input =
///     "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"
///         .to_string();
///
/// let (data, length) = parse_input(input);
///
/// assert_eq!(length, 5);
/// assert_eq!(
///     data,
///     vec![
///         0b00100,
///         0b11110,
///         0b10110,
///         0b10111,
///         0b10101,
///         0b01111,
///         0b00111,
///         0b11100,
///         0b10000,
///         0b11001,
///         0b00010,
///         0b01010,
///     ]
/// )
/// ```
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

/// Return the number of values in the data where all the bits set in the bitmask are also set in
/// the value. Utility function used by [`analyse_diagnostics`]. For this puzzle, only one bit is
/// set in the mask at a time.
///
/// # Example from puzzle specification
/// ```rust
/// let test_data = vec![
///     0b00100,
///     0b11110,
///     0b10110,
///     0b10111,
///     0b10101,
///     0b01111,
///     0b00111,
///     0b11100,
///     0b10000,
///     0b11001,
///     0b00010,
///     0b01010,
/// ];
///
/// assert_eq!(count_bit(&test_data, 1 << 0), 5);
/// assert_eq!(count_bit(&test_data, 1 << 1), 7);
/// assert_eq!(count_bit(&test_data, 1 << 2), 8);
/// assert_eq!(count_bit(&test_data, 1 << 3), 5);
/// assert_eq!(count_bit(&test_data, 1 << 4), 7);
/// ```
fn count_bit(data: &Vec<usize>, bitmask: usize) -> usize {
    data.iter()
        .filter(|&&value| value & bitmask == bitmask)
        .count()
}

/// This solves part one, returning the pair of required values. It calculates the gamma value by
/// iterating through each bit in the input strings and comparing the result of [`count_bit`] to the
/// size of the data. The epsilon value is the bitwise inverse of that.
///
/// # Example from puzzle specification
/// ```rust
/// let test_data = vec![
///     0b00100,
///     0b11110,
///     0b10110,
///     0b10111,
///     0b10101,
///     0b01111,
///     0b00111,
///     0b11100,
///     0b10000,
///     0b11001,
///     0b00010,
///     0b01010,
/// ];
///
/// assert_eq!(analyse_diagnostics(&test_data, 5), (22, 9));
/// ```
fn analyse_diagnostics(data: &Vec<usize>, length: usize) -> (usize, usize) {
    let mut gamma: usize = 0;
    let threshold = data.len() / 2;

    // We need to shift the bitmap from left to right. As the bit map is generated by left shifting
    // we start with the largest shift and work down.
    // 1 << 4 = 0b10000
    // 1 << 3 = 0b01000
    // 1 << 2 = 0b00100
    // 1 << 1 = 0b00010
    // 1 << 0 = 0b00001
    for position in (0..length).rev() {
        // We're working left to right, so left shift the previous gamma, then set the new bit
        // based on if the count of bits in that position is above the halfway point. The bit map
        // for count_bit can be calculated by just left shifting 1 to thr correct position
        gamma = (gamma << 1) + (count_bit(data, 1 << position) > threshold) as usize;
    }

    // To bitwise inverse gamma we need a binary number with `length` bits all set to `1`.
    // If length is 5 then:
    // 1        == 0b000001
    // ... << 5 == 0b100000
    // ... -1   == 0b011111
    let max = (1 << length) - 1;
    // Then xor the above with the gamma to inverse each bit, giving the expected epsilon
    let epsilon = gamma ^ max;

    return (gamma, epsilon);
}

/// This solves part two, returning the pair of required values. The solution calls for successively
/// filtering the input array until only one value array, which works well as a recursive function.
/// The base case for this is when the provided array is of length 1, and returns the remaining
/// value if it is present. Otherwise split the input based on whether the bit we care about is set,
/// then recurse the largest/smallest half based on which value is being calculated, and the bit
/// position advanced by 1. Unlike part one, the recursion means there isn't a neat trick to invert
/// the first result to product the second, so the recursive function is called twice, with a
/// flag used to switch the mode.
///
/// The recursive function will panic! if it has an empty data array or the position is negative.
/// e.g an input of `vec![0b100, 0b101, 0b110]` would panic when calculating the CO2 scrubber rating
/// because all values share the same first bit.
///
/// # Example from puzzle specification
/// ```rust
/// let test_data = vec![
///     0b00100,
///     0b11110,
///     0b10110,
///     0b10111,
///     0b10101,
///     0b01111,
///     0b00111,
///     0b11100,
///     0b10000,
///     0b11001,
///     0b00010,
///     0b01010,
/// ];
///
/// assert_eq!(analyse_life_support(&test_data(), 5), (23, 10));
/// ```
fn analyse_life_support(data: &Vec<usize>, length: usize) -> (usize, usize) {
    fn iter(mut current: Vec<usize>, position: usize, keep_smallest: bool) -> usize {
        // base case
        if current.len() == 1 {
            return *(current.get(0).expect("Guaranteed, len == 1"));
        }

        // Sanity check to prevent infinite recursion.
        if current.len() == 0 || position == 0 {
            panic!(
                "Non-unique result found. current position {}. This can occur when all values have \
                 the same bit at a position, or when the input contains a duplicate value.",
                position
            )
        }

        let bitmask = 1 << position - 1;
        // partition in place, all the values before split_index have the bit set, the value at that
        // position and later do not.
        let split_index = partition(current.as_mut_slice(), |value| value & bitmask != 0);
        let (left, right) = current.split_at(split_index);

        // The xor here lets the keep_smallest flag invert the size comparison when set
        let keep_left = (left.len() >= right.len()) ^ keep_smallest;

        iter(
            (if keep_left { left } else { right }).to_vec(),
            position - 1,
            keep_smallest,
        )
    }

    let oxygen = iter(data.to_vec(), length, false);
    let co2 = iter(data.to_vec(), length, true);
    return (oxygen, co2);
}

#[cfg(test)]
mod tests {
    use crate::day_3::{analyse_diagnostics, analyse_life_support, count_bit, parse_input};

    fn test_data() -> Vec<usize> {
        vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ]
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

    #[test]
    #[should_panic(
        expected = "Non-unique result found. current position 2. This can occur when \
    all values have the same bit at a position, or when the input contains a duplicate value."
    )]
    fn does_not_infinitely_recurse_on_invalid_input() {
        analyse_life_support(&vec![0b100, 0b101, 0b110], 3);
    }
}
