//! This is my solution for [Advent of Code - Day 5 - _Hydrothermal Venture_](https://adventofcode.com/2021/day/5)
//!
//! Today was filling co-ordinates on a grid. Both tasks only needed to know which points had been filled twice, so I
//! was able to implement that with two HashSets of co-ordinates, only setting the second set if that co-ordinate was
//! set in the first. This is implemented in [`get_intersections`]. The other key piece of logic is translating the
//! lines of the input into the points along their path, implemented by [`Line::get_points`].
//!
//! Part one is just a limited version of part two, and my solution works the same for both.
//! [`get_axial_intersections`] uses [`Line::is_axial`] to filter out the diagonal lines that are only used in part
//! two. To implement part two I just had to add the test cases for the diagonal lines, everything else just worked.

use regex::Regex;
use std::cmp::max;
use std::collections::HashSet;
use std::fs;

/// Represent a line using the co-ordinates of each end.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    #[cfg(test)]
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Line {
        return Line { x1, y1, x2, y2 };
    }

    /// True if the line is parallel to either the x or y axis
    fn is_axial(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    /// Return an iterator of the points on the grid this line intersects
    fn get_points(&self) -> HashSet<(usize, usize)> {
        // The number of points intersected by the line - we need the max as either d_x or d_y will be 0 for axial lines
        let length = max(
            (self.x1 as isize - self.x2 as isize).abs(),
            (self.y1 as isize - self.y2 as isize).abs(),
        );

        // Helper function so we don't have to repeat the step logic for x and y
        fn get_step(p1: usize, p2: usize) -> isize {
            match (p1, p2) {
                (a, b) if a < b => 1,
                (a, b) if a == b => 0,
                _ => -1,
            }
        }

        // because the input lines are always axial or diagonal they have a regular step for each point.
        let d_x = get_step(self.x1, self.x2);
        let d_y = get_step(self.y1, self.y2);

        // iterate through each point applying the calculated deltas
        (0..=length)
            .map(|i| {
                (
                    (self.x1 as isize + i * d_x) as usize,
                    (self.y1 as isize + i * d_y) as usize,
                )
            })
            .collect()
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-5-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 5.
pub fn run() {
    let contents = fs::read_to_string("res/day-5-input").expect("Failed to read file");
    let lines = parse_input(contents);

    let intersections = get_axial_intersections(&lines);
    println!("There are {} axial intersections", intersections.len());

    let intersections = get_intersections(&lines);
    println!("There are {} full intersections", intersections.len());
}

/// Takes a string with lines in the form `(x1,y1) -> (x2,y2)` and converts it into a list of [`Line`]s. Parsed
/// using a regular expression.
fn parse_input(input: String) -> Vec<Line> {
    let line_matcher = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    input
        .lines()
        .flat_map(|line| {
            line_matcher
                .captures(line)
                // Use zip to merge the individual capturing group Option into a single `Option((x1, y1),(x2,y2))`
                // form. The values are still strings here
                .and_then(|cap| cap.get(1).zip(cap.get(2)).zip(cap.get(3).zip(cap.get(4))))
                // Transform that option into the same shape, but with the strings parsed as `usize`s. Split out into
                // variables for clarity, but mostly because `rustfmt` mangles it otherwise.
                .and_then(|((x1, y1), (x2, y2))| {
                    let x1_res = x1.as_str().parse::<usize>().ok();
                    let y1_res = y1.as_str().parse::<usize>().ok();
                    let start = x1_res.zip(y1_res);

                    let x2_res = x2.as_str().parse::<usize>().ok();
                    let y2_res = y2.as_str().parse::<usize>().ok();
                    let end = x2_res.zip(y2_res);

                    start.zip(end)
                })
                // and match that shape, mapping it into the required line
                .map(|((x1, y1), (x2, y2))| Line { x1, y1, x2, y2 })
        })
        .collect()
}

/// Filter out diagonal lines before running the remaining lines through [`get_intersections`]
fn get_axial_intersections(lines: &Vec<Line>) -> HashSet<(usize, usize)> {
    let filtered = lines.iter().filter(|l| l.is_axial()).map(|&l| l).collect();
    get_intersections(&filtered)
}

/// For each line, iterate over it's points adding each to a set (visited at least once). If that insert fails, we've
/// already seen that point so add it to a second set (visited at least twice). Points repeated more than twice can
/// be ignored, as this is not needed to provide the puzzle solution. Return that set, the length of the set will
/// give the number of points where two of more lines intersect.
fn get_intersections(lines: &Vec<Line>) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut intersected: HashSet<(usize, usize)> = HashSet::new();

    lines.iter().flat_map(|l| l.get_points()).for_each(|point| {
        if !visited.insert(point) {
            intersected.insert(point);
        }
    });

    intersected
}

#[cfg(test)]
mod tests {
    use crate::day_5::{get_axial_intersections, get_intersections, parse_input, Line};
    use std::collections::HashSet;

    fn test_lines() -> Vec<Line> {
        vec![
            Line::new(0, 9, 5, 9),
            Line::new(8, 0, 0, 8),
            Line::new(9, 4, 3, 4),
            Line::new(2, 2, 2, 1),
            Line::new(7, 0, 7, 4),
            Line::new(6, 4, 2, 0),
            Line::new(0, 9, 2, 9),
            Line::new(3, 4, 1, 4),
            Line::new(0, 0, 8, 8),
            Line::new(5, 5, 8, 2),
        ]
    }

    #[test]
    fn can_parse() {
        let input = "0,9 -> 5,9\n\
                    8,0 -> 0,8\n\
                    9,4 -> 3,4\n\
                    2,2 -> 2,1\n\
                    7,0 -> 7,4\n\
                    6,4 -> 2,0\n\
                    0,9 -> 2,9\n\
                    3,4 -> 1,4\n\
                    0,0 -> 8,8\n\
                    5,5 -> 8,2"
            .to_string();

        let expected = test_lines();

        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn can_check_for_axial_lines() {
        assert_eq!(
            test_lines()
                .iter()
                .map(|l| l.is_axial())
                .collect::<Vec<bool>>(),
            vec![true, false, true, true, true, false, true, true, false, false]
        )
    }

    #[test]
    fn can_get_points() {
        let actual = test_lines()
            .iter()
            .map(|l| l.get_points())
            .collect::<Vec<HashSet<(usize, usize)>>>();

        let expected = vec![
            HashSet::from([(0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9)]),
            HashSet::from([
                (8, 0),
                (7, 1),
                (6, 2),
                (5, 3),
                (4, 4),
                (3, 5),
                (2, 6),
                (1, 7),
                (0, 8),
            ]),
            HashSet::from([(9, 4), (8, 4), (7, 4), (6, 4), (5, 4), (4, 4), (3, 4)]),
            HashSet::from([(2, 2), (2, 1)]),
            HashSet::from([(7, 0), (7, 1), (7, 2), (7, 3), (7, 4)]),
            HashSet::from([(6, 4), (5, 3), (4, 2), (3, 1), (2, 0)]),
            HashSet::from([(0, 9), (1, 9), (2, 9)]),
            HashSet::from([(3, 4), (2, 4), (1, 4)]),
            HashSet::from([
                (0, 0),
                (1, 1),
                (2, 2),
                (3, 3),
                (4, 4),
                (5, 5),
                (6, 6),
                (7, 7),
                (8, 8),
            ]),
            HashSet::from([(5, 5), (6, 4), (7, 3), (8, 2)]),
        ];

        actual
            .iter()
            .zip(expected.iter())
            .for_each(|(a, e)| assert_eq!(a, e));
    }

    #[test]
    fn can_get_axial_intersections() {
        let intersections = get_axial_intersections(&test_lines());
        assert_eq!(intersections.len(), 5);
        assert!(intersections.contains(&(3, 4)));
        assert!(intersections.contains(&(7, 4)));
        assert!(intersections.contains(&(0, 9)));
        assert!(intersections.contains(&(1, 9)));
        assert!(intersections.contains(&(2, 9)));
    }

    #[test]
    fn can_get_intersections() {
        let intersections = get_intersections(&test_lines());
        assert_eq!(intersections.len(), 12);
        assert!(intersections.contains(&(7, 1)));
        assert!(intersections.contains(&(2, 2)));
        assert!(intersections.contains(&(5, 3)));
        assert!(intersections.contains(&(7, 3)));
        assert!(intersections.contains(&(3, 4)));
        assert!(intersections.contains(&(4, 4)));
        assert!(intersections.contains(&(7, 4)));
        assert!(intersections.contains(&(6, 4)));
        assert!(intersections.contains(&(5, 5)));
        assert!(intersections.contains(&(0, 9)));
        assert!(intersections.contains(&(1, 9)));
        assert!(intersections.contains(&(2, 9)));
    }
}
