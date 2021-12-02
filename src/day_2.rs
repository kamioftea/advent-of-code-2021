//! This is my solution for [Advent of Code - Day 2 - _Dive!_](https://adventofcode.com/2021/day/2)
//!
//! Today involves parsing a sequence of instructions that direct the submarine to move on a grid.
//! First [`parse_line`] is used to parse the string input into an internal representation
//! [`Instruction`]. Rust has powerful pattern matching but requires that it is exhaustive. Front
//! loading parsing the strings both makes the code more understandable, but also simplifies the
//! match statements as we don't need to repeatedly handle possible bad input.
//!
//! The two parts differ in how the input should be interpreted. I've implemented both as a fold
//! over the sequence of instructions, matching the Direction and updating the position as specified
//! for that part. Part one takes the instructions at face value, the logic is implemented by
//! [`navigate`]. Part two tracks a third variable 'aim', but is otherwise very similar. The logic
//! is implemented by [`navigate_and_aim`].

use std::fs;

use day_2::Direction::{DOWN, FORWARD, UP};

/// There are three direction strings expected in the input. Parsing those into an Enum type helps
/// doing exhaustive matches later
#[derive(Eq, PartialEq, Debug)]
enum Direction {
    FORWARD,
    UP,
    DOWN,
}

/// Each line of the input is a pair of direction and magnitude - alias this for clarity
type Instruction = (Direction, isize);

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-2-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 2
///
/// This also handles converting the raw input into a `Vec<Instruction>`, delegating the parsing to
/// [`parse_line`].
pub fn run() {
    let contents: Vec<Instruction> = fs::read_to_string("res/day-2-input")
        .expect("Failed to read file")
        .lines()
        .map(|line| parse_line(line))
        .collect();

    let (h1, d1) = navigate(&contents);
    println!("Final position ({}, {}) = {}", h1, d1, h1 * d1);

    let (h2, d2, _) = navigate_and_aim(&contents);
    println!("Final position with aiming ({}, {}) = {}", h2, d2, h2 * d2)
}

/// Parses a line in the format `(forward|up|down) \d+` into the internal representation
/// [`Instruction`]. Will panic if the provided line does not match the expected format.
///
/// # Example from puzzle specification
/// ```rust
/// assert_eq!(parse_line("forward 5"), (FORWARD, 5));
/// assert_eq!(parse_line("down 5"),    (DOWN,    5));
/// assert_eq!(parse_line("forward 8"), (FORWARD, 8));
/// assert_eq!(parse_line("up 3"),      (UP,      3));
/// assert_eq!(parse_line("down 8"),    (DOWN,    8));
/// assert_eq!(parse_line("forward 2"), (FORWARD, 2));
/// ```
fn parse_line(line: &str) -> Instruction {
    if let Some((direction, magnitude)) = line.split_once(" ") {
        return (
            match direction {
                "forward" => FORWARD,
                "up" => UP,
                "down" => DOWN,
                unexpected => panic!("Unexpected direction {}", unexpected),
            },
            magnitude
                .parse::<isize>()
                .expect("Magnitude was not a number"),
        );
    }

    panic!("Line '{}' was not in the expected format", line)
}

/// This starts with the submarine at the origin, and moves using the following rules:
/// - _Forward_: Increase the horizontal position by the magnitude
/// - _Up_: Decrease the depth by the magnitude
/// - _Down_: Increase the depth by the magnitude
///
/// The final position after applying all the instructions in order is returned as a tuple
/// `(horizontal_position, depth)`
/// # Example from puzzle specification
/// ```rust
/// let input = vec![
///     (FORWARD, 5),
///     (DOWN, 5),
///     (FORWARD, 8),
///     (UP, 3),
///     (DOWN, 8),
///     (FORWARD, 2),
/// ]
/// assert_eq!(navigate(&input), (15, 10))
/// ```
fn navigate(instructions: &Vec<Instruction>) -> (isize, isize) {
    instructions.iter().fold(
        (0, 0),
        |(horizontal, depth), (direction, magnitude)| match direction {
            FORWARD => (horizontal + magnitude, depth),
            UP => (horizontal, depth - magnitude),
            DOWN => (horizontal, depth + magnitude),
        },
    )
}

/// This starts with the submarine at the origin, with a third variable 'aim' also set to 0. The
/// position and aim are updated using the following rules:
/// - _Forward_: Increase the horizontal position by the magnitude, increase the depth by
///   `(magnitude x current aim)`
/// - _Up_: Decrease the aim by the magnitude
/// - _Down_: Increase the aim by the magnitude
///
/// The final position and aim after applying all the instructions in order is returned as a tuple
/// `(horizontal_position, depth, aim)`
/// # Example from puzzle specification
/// ```rust
/// let input = vec![
///     (FORWARD, 5),
///     (DOWN, 5),
///     (FORWARD, 8),
///     (UP, 3),
///     (DOWN, 8),
///     (FORWARD, 2),
/// ]
/// assert_eq!(navigate_and_aim(&input), (15, 60, 10))
/// ```
fn navigate_and_aim(instructions: &Vec<Instruction>) -> (isize, isize, isize) {
    instructions.iter().fold(
        (0, 0, 0),
        |(horizontal, depth, aim), (direction, magnitude)| match direction {
            FORWARD => (horizontal + magnitude, depth + (aim * magnitude), aim),
            UP => (horizontal, depth, aim - magnitude),
            DOWN => (horizontal, depth, aim + magnitude),
        },
    )
}

#[cfg(test)]
mod tests {
    use day_2::Direction::*;
    use day_2::{navigate, navigate_and_aim, parse_line, Instruction};

    #[test]
    fn can_parse() {
        assert_eq!(parse_line("forward 5"), (FORWARD, 5));
        assert_eq!(parse_line("down 5"), (DOWN, 5));
        assert_eq!(parse_line("forward 8"), (FORWARD, 8));
        assert_eq!(parse_line("up 3"), (UP, 3));
        assert_eq!(parse_line("down 8"), (DOWN, 8));
        assert_eq!(parse_line("forward 2"), (FORWARD, 2));
    }

    #[test]
    fn can_navigate() {
        assert_eq!(navigate(&test_data()), (15, 10))
    }

    #[test]
    fn can_navigate_and_aim() {
        assert_eq!(navigate_and_aim(&test_data()), (15, 60, 10))
    }

    fn test_data() -> Vec<Instruction> {
        vec![
            (FORWARD, 5),
            (DOWN, 5),
            (FORWARD, 8),
            (UP, 3),
            (DOWN, 8),
            (FORWARD, 2),
        ]
    }
}
