//! This is my solution for [Advent of Code - Day 13 - _Transparent Origami_](https://adventofcode.com/2021/day/13)
//!
//! The hardest part of today's solution was parsing the input. I opted for a `HashSet` of
//! co-ordinates rather than representing the full grid as the dots were very sparse. I also created
//! a simple enum [`Axis`] to track the axis of each fold. [`parse_input`] is mostly checks to
//! confirm each line confirms to spec, as Rust is very explicit about making you handle everything.
//!
//! [`apply_fold`] iterates through the dots adding them to a new folded set. If they are left of /
//! above the relevant axis, they are inserted as is, otherwise the new position is calculated and
//! inserted. The `len()` of the resulting set when applying the first fold gives the answer to part
//! one. Part two requires two extra functions [`apply_folds`] uses [`apply_fold`] with each fold in
//! turn, and [`display_dots`] takes the resulting set and renders it as a grid so that the code can
//! be read by a human.

use crate::day_13::Axis::{X, Y};
use std::collections::HashSet;
use std::fs;

/// Controls the axis each fold will be applied using
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Axis {
    X,
    Y,
}

impl From<&str> for Axis {
    fn from(s: &str) -> Self {
        match s {
            "x" => X,
            "y" => Y,
            _ => panic!("unexpected axis: {}", s),
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-13-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 13.
pub fn run() {
    let contents = fs::read_to_string("res/day-13-input").expect("Failed to read file");
    let (dots, folds) = parse_input(contents);

    let new_count = apply_fold(&dots, folds[0]).len();
    println!("After the first fold there are {} dots", new_count);

    let folded = apply_folds(&dots, &folds);
    println!("The folded paper shows:\n{}", display_dots(&folded));
}

/// The puzzle input is in two sections separated by a blank line. Section one is the initial set of
/// dot co-ordinates, in the format `x,y`. Section two is a list of folds in the format
/// `fold along <axis>=<co-ordinate>`.
fn parse_input(input: String) -> (HashSet<(usize, usize)>, Vec<(Axis, usize)>) {
    // split on the blank line
    let (dots, folds) = input
        .split_once("\n\n")
        .expect("Invalid input - no section separator");
    (
        // for each co-ordinate line
        dots.lines()
            .map(|line| {
                // split at the comma
                let (x, y) = line
                    .split_once(",")
                    .expect(format!("Invalid dot {}", line).as_str());
                // and parse both as numbers
                (
                    x.parse::<usize>()
                        .expect(format!("Invalid dot x {}", line).as_str()),
                    y.parse::<usize>()
                        .expect(format!("Invalid dot y {}", line).as_str()),
                )
            })
            .collect(),
        // for each fold
        folds
            .lines()
            .map(|line| {
                // strip the superfluous prefix
                let definition = line.replace("fold along ", "");
                // split at the equals
                let (axis, pos) = definition
                    .split_once("=")
                    .expect(format!("Invalid fold {}", line).as_str());
                // parse as an [`Axis`] and a number
                (
                    Axis::from(axis),
                    pos.parse::<usize>()
                        .expect(format!("Invalid fold pos {}", line).as_str()),
                )
            })
            .collect(),
    )
}

/// Return a new set where the first has been folded along the given axis
fn apply_fold(dots: &HashSet<(usize, usize)>, fold: (Axis, usize)) -> HashSet<(usize, usize)> {
    let (axis, position) = fold;
    dots.iter()
        .map(|&(x, y)| match (axis, (x, y)) {
            // Folding by y and dot is right of the fold
            (X, (x1, y1)) if x1 > position => (2 * position - x1, y1),
            // Folding by y and dot is below the fold
            (Y, (x1, y1)) if y1 > position => (x1, 2 * position - y1),
            // otherwise leave as is
            (_, coords) => coords,
        })
        .collect()
}

/// Fold the list of folds into the starting set of dots #tooManyFolds
fn apply_folds(
    dots: &HashSet<(usize, usize)>,
    folds: &Vec<(Axis, usize)>,
) -> HashSet<(usize, usize)> {
    folds
        .iter()
        .fold(dots.clone(), |acc, &fold| apply_fold(&acc, fold))
}

/// This calculates the maximum x and y in the set to determine the grid bounds, then loops through
/// each row and column outputting a `▮` or ` ` based on if the current coordinate is in the set.
///
/// # Example from puzzle specification
/// ```rust
/// let dots = HashSet::from([
///     (6usize, 10usize),
///     (0usize, 14usize),
///     (9usize, 10usize),
///     (0usize, 3usize),
///     (10usize, 4usize),
///     (4usize, 11usize),
///     (6usize, 0usize),
///     (6usize, 12usize),
///     (4usize, 1usize),
///     (0usize, 13usize),
///     (10usize, 12usize),
///     (3usize, 4usize),
///     (3usize, 0usize),
///     (8usize, 4usize),
///     (1usize, 10usize),
///     (2usize, 14usize),
///     (8usize, 10usize),
///     (9usize, 0usize),
/// ]);
///
/// let folds = vec![(Y, 7usize), (X, 5usize)];
///
/// let expected = "▮▮▮▮▮\n\
///                 ▮   ▮\n\
///                 ▮   ▮\n\
///                 ▮   ▮\n\
///                 ▮▮▮▮▮\n\
///                 ".to_string();
///
/// assert_eq!(display_dots(&apply_folds(&dots, &folds)), expected);
/// ```
fn display_dots(dots: &HashSet<(usize, usize)>) -> String {
    // get bounds
    let max_x = dots.iter().map(|&(x, _)| x).max().expect("No dots");
    let max_y = dots.iter().map(|&(_, y)| y).max().expect("No dots");

    let mut out = "".to_string();
    for y in 0..=max_y {
        for x in 0..=max_x {
            out = format!("{}{}", out, if dots.contains(&(x, y)) { "▮" } else { " " })
        }
        out = format!("{}{}", out, "\n")
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::day_13::Axis::{X, Y};
    use crate::day_13::{apply_fold, apply_folds, display_dots, parse_input, Axis};
    use std::collections::HashSet;

    fn sample_puzzle() -> (HashSet<(usize, usize)>, Vec<(Axis, usize)>) {
        (
            HashSet::from([
                (6usize, 10usize),
                (0usize, 14usize),
                (9usize, 10usize),
                (0usize, 3usize),
                (10usize, 4usize),
                (4usize, 11usize),
                (6usize, 0usize),
                (6usize, 12usize),
                (4usize, 1usize),
                (0usize, 13usize),
                (10usize, 12usize),
                (3usize, 4usize),
                (3usize, 0usize),
                (8usize, 4usize),
                (1usize, 10usize),
                (2usize, 14usize),
                (8usize, 10usize),
                (9usize, 0usize),
            ]),
            vec![(Y, 7usize), (X, 5usize)],
        )
    }
    #[test]
    fn can_parse() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
            .to_string();

        let expected = sample_puzzle();

        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn can_fold() {
        let (dots, folds) = sample_puzzle();
        assert_eq!(apply_fold(&dots, folds[0]).len(), 17)
    }

    #[test]
    fn can_display_result() {
        let (dots, folds) = sample_puzzle();
        let expected = "▮▮▮▮▮\n\
                               ▮   ▮\n\
                               ▮   ▮\n\
                               ▮   ▮\n\
                               ▮▮▮▮▮\n\
                               "
        .to_string();
        assert_eq!(display_dots(&apply_folds(&dots, &folds)), expected);
    }
}
