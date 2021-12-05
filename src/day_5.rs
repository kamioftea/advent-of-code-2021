//! This is my solution for [Advent of Code - Day 5 - _Hydrothermal Venture_](https://adventofcode.com/2021/day/5)
//!
//!

use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Line {
        return Line { x1, y1, x2, y2 };
    }

    fn is_axial(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn get_points(&self) -> HashSet<(usize, usize)> {
        let xs: Vec<usize> = if self.x1 < self.x2 {
            (self.x1..=self.x2).collect()
        } else if self.x1 == self.x2 {
            let len = self.y1 as isize - self.y2 as isize;
            vec![self.x1; len.abs() as usize + 1]
        } else {
            (self.x2..=self.x1).rev().collect()
        };

        let ys: Vec<usize> = if self.y1 < self.y2 {
            (self.y1..=self.y2).collect()
        } else if self.y1 == self.y2 {
            let len = self.x1 as isize - self.x2 as isize;
            vec![self.y1; len.abs() as usize + 1]
        } else {
            (self.y2..=self.y1).rev().collect()
        };

        xs.iter().map(|&x| x).zip(ys.iter().map(|&y| y)).collect()
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

fn parse_input(input: String) -> Vec<Line> {
    let line_matcher = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    input
        .lines()
        .flat_map(|line| {
            line_matcher
                .captures(line)
                .and_then(|cap| cap.get(1).zip(cap.get(2)).zip(cap.get(3).zip(cap.get(4))))
                .and_then(|((x1, y1), (x2, y2))| {
                    x1.as_str()
                        .parse::<usize>()
                        .ok()
                        .zip(y1.as_str().parse::<usize>().ok())
                        .zip(
                            x2.as_str()
                                .parse::<usize>()
                                .ok()
                                .zip(y2.as_str().parse::<usize>().ok()),
                        )
                })
                .map(|((x1, y1), (x2, y2))| Line { x1, y1, x2, y2 })
        })
        .collect()
}

fn get_axial_intersections(lines: &Vec<Line>) -> HashSet<(usize, usize)> {
    let filtered = lines.iter().filter(|l| l.is_axial()).map(|&l| l).collect();
    get_intersections(&filtered)
}

fn get_intersections(lines: &Vec<Line>) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut intersected: HashSet<(usize, usize)> = HashSet::new();

    lines.iter().flat_map(|l| l.get_points()).for_each(|point| {
        if visited.contains(&point) {
            intersected.insert(point);
        } else {
            visited.insert(point);
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
