//! This is my solution for [Advent of Code - Day 12 - _Passage Pathing_](https://adventofcode.com/2021/day/12)
//!
//! Today was graph traversing. The puzzle was to find all the paths to traverse a set of caves, some of which could
//! be visited any number of times (large), and only once-ish (small). I say ish, because part two's extension was
//! that exactly one (1) small cave could be revisited in the path. I've previously struggled with graph
//! representation in Rust, and today was tricky, but I was able to apply the learnings from previous years and get the
//! borrow checker happy pretty quickly. This was a more impressive feat with my original solution that used the
//! string representation and a hash map rather than a vector and mapping the labels to indices.
//!
//! There are two main  structs used in today's solutions. [`Cave`] represents a node in the graph, tracking if it is
//! large or small (or one of the special types start and end), and the other cave(s) linked to that cave.
//! [`parse_input`] takes the puzzle input and converts it into a `Vec<Cave>`, using [`get_index`] to manage the
//! mapping of label -> index. [`Path`] tracks an in progress path using a set of the visited nodes (using a usize
//! as a bitmap), the current position of the head of the path, and (for part two) a flag tracking whether it has used
//! its one-off repeat visit.
//!
//! [`build_paths`] and [`Path::with_cave`] handle the logic for solving both parts. [`build_paths`] taking a flag to
//! control which part it is solving. The strategy is to have a stack of paths to analyse, pop one at a time, append
//! each linked cave to that path in turn using [`Path::with_cave`], and push the valid paths into the completed list
//! if we've appended 'end', otherwise back onto the stack of pending paths - so doing depth first search. Using a
//! queue would give breadth first search, but it's a moot point as we need the exhaustive list of paths anyway.
//!
//! Today was the worst in terms of initial performance. It was taking ~400ms to run both parts, compared to ~100ms
//! to run all of days 1 to 11. My initial implementation was using a `HashSet<&str>` for the visited nodes, and a
//! `HashMap<&str, Cave>` for the cave list. Switching this over to a `usize` bitmap, and `Vec<Cave>` brought it down
//! to ~130ms, better but still slower than the other days combined. The second optimisation was that I had
//! previously been tracking the full path up to that point as a `Vec<usize>` (originally `Vec<&str>`.) Because the
//! paths branch, adding a new cave to a path needs to copy it, so the original is left as is to have the next linked
//! cave appended. This had been useful for debugging, esp. when it was using `&str`s to track the caves, but wasn't
//! actually needed to calculate the solution - the visited set and current position are sufficient. Removing this
//! `Vec` and associated copying brought it down to ~10ms 🎉. I was very glad for the unit tests that let me refactor
//! each step with confidence. Getting [`Path::with_cave`] right took a few attempts, and the tests quickly helped me
//! identify where I'd gone wrong.

use std::collections::HashMap;
use std::fs;

use crate::day_12::CaveType::{END, LARGE, SMALL, START};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
/// Track the four possible node types that dictate how they can be visited
enum CaveType {
    /// First node in all valid paths, cannot be revisited
    START,
    /// Last node in all valid paths, path instantly ends on visiting so implicitly can't be revisited
    END,
    /// Can only be visited once, except in part two where a path can optionally revisit exactly one small node
    /// somewhere along its route
    SMALL,
    /// Can be revisited any number of times, regardless of mode
    LARGE,
}

impl From<&str> for CaveType {
    /// start and end are specific strings, otherwise large vs small is determined by the case of the string
    fn from(s: &str) -> Self {
        match s {
            "start" => START,
            "end" => END,
            label if label.to_uppercase() == label => LARGE,
            _ => SMALL,
        }
    }
}

/// Represents a node (cave) in the graph (cave system)
#[derive(Eq, PartialEq, Debug)]
struct Cave {
    /// Determines how many times this cave can be visited in a path
    cave_type: CaveType,
    /// The indices of the nodes linked to this one by an edge
    links: Vec<usize>,
}

impl From<&str> for Cave {
    /// initialise an unlinked cave, using the cave's label to determine its type
    fn from(s: &str) -> Self {
        return Cave {
            cave_type: CaveType::from(s),
            links: Vec::new(),
        };
    }
}

/// Represents a path from the start to the node at [`position`].
#[derive(Eq, PartialEq, Debug)]
struct Path {
    /// bitmap of visited nodes (13 puzzle input nodes - so works on 16+ bit architectures)
    visited: usize,
    /// current node index
    position: usize,
    /// flag to track if it has used its one allowed small cave revisit
    can_revisit: bool,
}

impl Path {
    /// If visiting the provided cave would be valid, return the path with that cave appended, otherwise None
    fn with_cave(&self, cave: usize, cave_type: CaveType) -> Option<Path> {
        // Set the visited bit for the provided cave
        let new_visited = self.visited | (1 << cave);
        if cave_type == LARGE // unlimited visits
            || new_visited != self.visited // if equal, this cave was already in the visited set
            || (self.can_revisit && cave_type == SMALL)
        // haven't yet used up the allowed revisit
        {
            Some(Path {
                visited: new_visited,
                position: cave,
                // once unset, can_revisit must stay unset, otherwise unset it only if revisiting a small cave
                can_revisit: self.can_revisit
                    && (self.visited != new_visited || cave_type != SMALL),
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
    println!(
        "There are {} paths through the {} caves.",
        paths.len(),
        caves.len()
    );

    let paths_with_revisit = build_paths(&caves, true);
    println!(
        "There are {} paths through the caves with revisit.",
        paths_with_revisit.len()
    );
}

/// Helper for parse_input that handles mapping a label to an index in the cave vector, initialising a cave and dding it
/// to the vector and lookup table if it's a new cave.
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

/// Split each line into the two ends of the edge, lookup/create the cave for each, and add each to the opposite
/// end's list of links.
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

/// Find the start and end indices, initialise a single path at the start position, and an empty list of complete
/// paths. Take nodes from the stack, iterating through the linked caves and pushing all valid new paths back onto the
/// stack (if incomplete) or into the list of complete paths if their updated position is the end node, repeat until
/// the stack is exhausted and return the completed path.
fn build_paths<'a>(caves: &Vec<Cave>, can_revisit: bool) -> Vec<Path> {
    // Lookup the start and end for later use
    let start = caves
        .iter()
        .position(|c| c.cave_type == START)
        .expect("No start cave");

    let end = caves
        .iter()
        .position(|c| c.cave_type == END)
        .expect("No end cave");

    // initialise the stack and result list
    let mut paths = vec![Path {
        visited: 1 << start,
        position: start,
        // if revisiting shouldn't be allowed, just don't set the flag in the first place
        can_revisit,
    }];

    let mut completed_paths: Vec<Path> = Vec::new();

    while let Some(path) = paths.pop() {
        // get the cave at the current node
        if let Some(cave) = caves.get(path.position) {
            cave.links
                .iter()
                // returns an option, so flat_map here filters out invalid paths
                .flat_map(|&next_cave| {
                    let next_cave_type = caves.get(next_cave).unwrap().cave_type;
                    path.with_cave(next_cave, next_cave_type)
                })
                // check if path has reached the end and add to the relevant list
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
