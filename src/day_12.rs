//! This is my solution for [Advent of Code - Day 12 - _Passage Pathing_](https://adventofcode.com/2021/day/12)
//!
//!

use std::collections::HashMap;
use std::fs;

use crate::day_12::CaveType::{END, LARGE, SMALL, START};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum CaveType {
    START,
    END,
    SMALL,
    LARGE,
}

impl From<&str> for CaveType {
    fn from(s: &str) -> Self {
        match s {
            "start" => START,
            "end" => END,
            label if label.to_uppercase() == label => LARGE,
            _ => SMALL,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Cave {
    cave_type: CaveType,
    links: Vec<usize>,
}

impl From<&str> for Cave {
    fn from(s: &str) -> Self {
        return Cave {
            cave_type: CaveType::from(s),
            links: Vec::new(),
        };
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Path {
    visited: usize,
    position: usize,
    can_revisit: bool,
}

impl Path {
    fn with_cave(&self, cave: usize, cave_type: CaveType) -> Option<Path> {
        let new_visited = self.visited | (1 << cave);
        if cave_type == LARGE
            || new_visited != self.visited
            || (self.can_revisit && cave_type != START)
        {
            Some(Path {
                visited: new_visited,
                position: cave,
                can_revisit: self.can_revisit
                    && (self.visited != new_visited || cave_type == LARGE),
            })
        } else {
            None
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-12-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 12.
pub fn run() {
    let contents = fs::read_to_string("res/day-12-input").expect("Failed to read file");
    let caves = parse_input(&contents);

    let paths = build_paths(&caves, false);
    println!("There are {} paths through the caves.", paths.len());

    let paths_with_revisit = build_paths(&caves, true);
    println!(
        "There are {} paths through the caves with revisit.",
        paths_with_revisit.len()
    );
}

fn get_index<'a>(
    caves: &mut Vec<Cave>,
    lookup: &mut HashMap<&'a str, usize>,
    label: &'a str,
) -> usize {
    match lookup.get(label) {
        Some(&i) => i,
        None => {
            let i = caves.len();
            caves.push(Cave::from(label));
            lookup.insert(label, i);
            i
        }
    }
}

fn parse_input(input: &String) -> Vec<Cave> {
    let mut caves = Vec::new();
    let mut lookup: HashMap<&str, usize> = HashMap::new();

    input
        .lines()
        .flat_map(|line| line.split_once("-"))
        .for_each(|(a, b)| {
            let a_i = get_index(&mut caves, &mut lookup, a);
            let b_i = get_index(&mut caves, &mut lookup, b);

            caves.get_mut(a_i).unwrap().links.push(b_i);
            caves.get_mut(b_i).unwrap().links.push(a_i);
        });

    return caves;
}

fn build_paths<'a>(caves: &Vec<Cave>, can_revisit: bool) -> Vec<Path> {
    let start = caves
        .iter()
        .position(|c| c.cave_type == START)
        .expect("No start cave");

    let end = caves
        .iter()
        .position(|c| c.cave_type == END)
        .expect("No end cave");

    let mut paths = vec![Path {
        visited: 1 << start,
        position: start,
        can_revisit,
    }];

    let mut completed_paths: Vec<Path> = Vec::new();
    while let Some(path) = paths.pop() {
        if let Some(cave) = caves.get(path.position) {
            cave.links
                .iter()
                .flat_map(|&next_cave| {
                    let next_cave_type = caves.get(next_cave).unwrap().cave_type;
                    path.with_cave(next_cave, next_cave_type)
                })
                .for_each(|path| {
                    if path.position == end {
                        completed_paths.push(path)
                    } else {
                        paths.push(path)
                    }
                })
        }
    }

    return completed_paths;
}

#[cfg(test)]
mod tests {
    use crate::day_12::CaveType::{END, LARGE, SMALL, START};
    use crate::day_12::{build_paths, parse_input, Cave};

    fn sample_input1() -> String {
        "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            .to_string()
    }

    fn sample_input2() -> String {
        "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            .to_string()
    }

    fn sample_input3() -> String {
        "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
            .to_string()
    }

    #[test]
    fn can_parse() {
        let input = sample_input1();
        #[rustfmt::skip]
            let expected = vec![
            Cave { cave_type: START, links: vec![1, 2] }, // start = 0
            Cave { cave_type: LARGE, links: vec![0, 3, 2, 5] }, // A = 1
            Cave { cave_type: SMALL, links: vec![0, 1, 4, 5] }, // b = 2
            Cave { cave_type: SMALL, links: vec![1] }, // c = 3
            Cave { cave_type: SMALL, links: vec![2] }, // d = 4
            Cave { cave_type: END, links: vec![1, 2] }, // end = 5
        ];

        assert_eq!(parse_input(&input), expected);
    }

    #[test]
    fn can_build_paths() {
        assert_eq!(build_paths(&parse_input(&sample_input1()), false).len(), 10);
        assert_eq!(build_paths(&parse_input(&sample_input2()), false).len(), 19);
        assert_eq!(
            build_paths(&parse_input(&sample_input3()), false).len(),
            226
        );
    }

    #[test]
    fn can_build_paths_with_revisit() {
        assert_eq!(build_paths(&parse_input(&sample_input1()), true).len(), 36);
        assert_eq!(build_paths(&parse_input(&sample_input2()), true).len(), 103);
        assert_eq!(
            build_paths(&parse_input(&sample_input3()), true).len(),
            3509
        );
    }
}
