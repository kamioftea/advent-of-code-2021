//! This is my solution for [Advent of Code - Day 23 - _Title_](https://adventofcode.com/2021/day/23)
//!
//!

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Display, Formatter};
use std::fs;

const COSTS: [usize; 4] = [1, 10, 100, 1000];

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone)]
struct Burrow {
    len: usize,
    positions: u128,
}

impl From<&String> for Burrow {
    fn from(str: &String) -> Self {
        let (len, positions) = str
            .chars()
            .flat_map(parse_letter)
            .fold((0, 0), |(len, pos), num| (len + 1, (pos << 3) + num));
        Burrow { len, positions }
    }
}

impl Display for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut chars: String = "".to_string();
        for i in 0..self.len {
            chars = chars
                + (match self.get_at(i) {
                    0 => ".",
                    1 => "A",
                    2 => "B",
                    3 => "C",
                    4 => "D",
                    _ => "?",
                })
        }

        write!(f, "{}", chars)
    }
}

impl Debug for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Burrow {
    fn get_at(&self, pos: usize) -> u128 {
        if pos >= self.len {
            panic!("Burrow overflow")
        }
        (self.positions >> ((self.len - pos - 1) * 3)) & 7
    }

    fn set_at(&mut self, pos: usize, val: u128) {
        let offset = ((self.len - pos - 1) * 3) as u128;
        let bits = (1 << self.len * 3) - 1;
        let hole = 7u128 << offset;
        let mask = hole ^ bits;
        let zeroed = self.positions & mask;
        self.positions = zeroed | (val << offset);
    }

    fn swap(&self, a: usize, b: usize) -> Burrow {
        let mut burrow = self.clone();
        burrow.set_at(a, self.get_at(b));
        burrow.set_at(b, self.get_at(a));
        burrow
    }
}

#[derive(Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    burrow: Burrow,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.burrow.cmp(&other.burrow))
    }
}

impl State {
    fn new(cost: usize, burrow: Burrow) -> Self {
        State { cost, burrow }
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-23-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 23.
pub fn run() {
    let contents = fs::read_to_string("res/day-23-input").expect("Failed to read file");
    let burrow = parse_input(&contents);
    let result = find_shortest_path(&burrow).unwrap();
    println!("Lowest energy for small burrow is {}", result);

    let expanded_burrow = expand_burrow(&burrow);
    let expanded_result = find_shortest_path(&expanded_burrow).unwrap();
    println!("Lowest energy for expanded burrow is {}", expanded_result);
}

fn parse_letter(letter: char) -> Option<u128> {
    match letter {
        '.' => Some(0u128),
        'A' => Some(1u128),
        'B' => Some(2u128),
        'C' => Some(3u128),
        'D' => Some(4u128),
        _ => None,
    }
}

fn parse_input(input: &String) -> Burrow {
    let (len, positions) = input
        .lines()
        .skip(2)
        .flat_map(|line| line.chars().flat_map(parse_letter))
        .fold((7, 0), |(len, pos), num| (len + 1, (pos << 3) + num));

    Burrow { len, positions }
}

fn build_goal(depth: usize) -> Burrow {
    let len = depth * 4 + 7;
    let row = (1 << 9) + (2 << 6) + (3 << 3) + 4;
    let positions = (0..depth).fold(0, |acc, _| (acc << 12) + row);

    Burrow { len, positions }
}

fn build_states(burrow: &Burrow) -> Vec<(usize, Burrow)> {
    let mut out = Vec::new();

    'outer: for i in 0..7 {
        let curr = burrow.get_at(i);
        if curr == 0 {
            continue;
        }
        let cost = COSTS[curr as usize - 1];
        let delta: isize = if i <= curr as usize { 1 } else { -1 };
        let target = if i <= curr as usize { curr } else { curr + 1 };
        let mut h_pos = i as usize;
        let mut dist = 1; // corner into room must be travelled
        while h_pos != target as usize {
            if [0, 6].contains(&h_pos) {
                dist += 1
            } else {
                dist += 2
            };
            h_pos = (h_pos as isize + delta) as usize;
            if burrow.get_at(h_pos) != 0 {
                continue 'outer;
            }
        }
        let mut v_pos = curr as usize + 6;
        let mut final_pos = v_pos;
        while v_pos < burrow.len {
            if burrow.get_at(v_pos) == 0 {
                final_pos = v_pos;
                dist += 1
            } else if burrow.get_at(v_pos) != curr {
                continue 'outer;
            }
            v_pos += 4;
        }
        out.push((cost * dist, burrow.swap(i, final_pos)));
    }

    for i in 0..4 {
        let mut pos = 7 + i;
        let mut dist = 2;
        while pos < burrow.len {
            let curr = burrow.get_at(pos);
            if burrow.get_at(pos) != 0 {
                let cost = COSTS[curr as usize - 1];
                let mut left_pos = i + 1;
                let mut left_dist = 0;
                while burrow.get_at(left_pos) == 0 {
                    out.push((cost * (dist + left_dist), burrow.swap(pos, left_pos)));
                    if left_pos == 0 {
                        break;
                    }
                    left_pos -= 1;
                    left_dist += if left_pos == 0 { 1 } else { 2 };
                }
                let mut right_pos = i + 2;
                let mut right_dist = 0;
                while right_pos <= 6 && burrow.get_at(right_pos) == 0 {
                    out.push((cost * (dist + right_dist), burrow.swap(pos, right_pos)));
                    right_pos += 1;
                    right_dist += if right_pos == 6 { 1 } else { 2 };
                }
                break;
            }
            pos += 4;
            dist += 1
        }
    }

    out
}

fn find_shortest_path(start: &Burrow) -> Option<usize> {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut dist: HashMap<u128, usize> = HashMap::new();
    let mut from: HashMap<u128, (usize, u128)> = HashMap::new();

    let depth = (start.len - 7) / 4;
    let goal = build_goal(depth);

    dist.insert(start.positions, 0);
    heap.push(State::new(0, start.clone()));

    while let Some(State { cost, burrow }) = heap.pop() {
        if burrow == goal {
            return Some(cost);
        }

        if cost > *dist.get(&burrow.positions).unwrap_or(&usize::MAX) {
            continue;
        }

        for (energy, next_burrow) in build_states(&burrow) {
            let next_cost = cost + energy;
            let curr_cost = dist.get(&next_burrow.positions).unwrap_or(&usize::MAX);
            if next_cost < *curr_cost {
                heap.push(State::new(next_cost, next_burrow.clone()));
                dist.insert(next_burrow.positions, next_cost);
                from.insert(next_burrow.positions, (energy, burrow.positions));
            }
        }
    }

    None
}

fn expand_burrow(burrow: &Burrow) -> Burrow {
    let mut as_str = format!("{}", burrow);
    as_str.insert_str(11, "DCBADBAC");
    Burrow::from(&as_str)
}

#[cfg(test)]
mod tests {
    use crate::day_23::{
        build_goal, build_states, expand_burrow, find_shortest_path, parse_input, Burrow,
    };
    use std::collections::HashSet;

    fn sample_start() -> Burrow {
        Burrow::from(&".......BCBDADCA".to_string())
    }

    #[test]
    fn can_parse() {
        let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"
            .to_string();

        let burrow = parse_input(&input);
        assert_eq!(burrow, sample_start());
        assert_eq!(format!("{}", burrow), ".......BCBDADCA".to_string())
    }

    #[test]
    fn can_swap() {
        let burrow = sample_start();
        let swapped = burrow.swap(0, 14);
        assert_eq!(format!("{}", swapped), "A......BCBDADC.".to_string())
    }

    #[test]
    fn can_build_goal() {
        assert_eq!(build_goal(2), Burrow::from(&".......ABCDABCD".to_string()));
        assert_eq!(
            build_goal(4),
            Burrow::from(&".......ABCDABCDABCDABCD".to_string())
        );
    }

    #[test]
    fn can_calc_next_state() {
        let actual = build_states(&sample_start());
        let expected = HashSet::from([
            (30, Burrow::from(&"B.......CBDADCA".to_string())),
            (20, Burrow::from(&".B......CBDADCA".to_string())),
            (20, Burrow::from(&"..B.....CBDADCA".to_string())),
            (40, Burrow::from(&"...B....CBDADCA".to_string())),
            (60, Burrow::from(&"....B...CBDADCA".to_string())),
            (80, Burrow::from(&".....B..CBDADCA".to_string())),
            (90, Burrow::from(&"......B.CBDADCA".to_string())),
            (500, Burrow::from(&"C......B.BDADCA".to_string())),
            (400, Burrow::from(&".C.....B.BDADCA".to_string())),
            (200, Burrow::from(&"..C....B.BDADCA".to_string())),
            (200, Burrow::from(&"...C...B.BDADCA".to_string())),
            (400, Burrow::from(&"....C..B.BDADCA".to_string())),
            (600, Burrow::from(&".....C.B.BDADCA".to_string())),
            (700, Burrow::from(&"......CB.BDADCA".to_string())),
            (70, Burrow::from(&"B......BC.DADCA".to_string())),
            (60, Burrow::from(&".B.....BC.DADCA".to_string())),
            (40, Burrow::from(&"..B....BC.DADCA".to_string())),
            (20, Burrow::from(&"...B...BC.DADCA".to_string())),
            (20, Burrow::from(&"....B..BC.DADCA".to_string())),
            (40, Burrow::from(&".....B.BC.DADCA".to_string())),
            (50, Burrow::from(&"......BBC.DADCA".to_string())),
            (9000, Burrow::from(&"D......BCB.ADCA".to_string())),
            (8000, Burrow::from(&".D.....BCB.ADCA".to_string())),
            (6000, Burrow::from(&"..D....BCB.ADCA".to_string())),
            (4000, Burrow::from(&"...D...BCB.ADCA".to_string())),
            (2000, Burrow::from(&"....D..BCB.ADCA".to_string())),
            (2000, Burrow::from(&".....D.BCB.ADCA".to_string())),
            (3000, Burrow::from(&"......DBCB.ADCA".to_string())),
        ]);

        for entry in &actual {
            assert!(expected.contains(entry))
        }
        assert_eq!(actual.len(), expected.len());

        let actual2 = build_states(&Burrow::from(&"....D.............B...C".to_string()));
        let expected2 = HashSet::from([
            (40, Burrow::from(&"....DB................C".to_string())),
            (50, Burrow::from(&"....D.B...............C".to_string())),
        ]);
        for entry in &actual2 {
            assert!(expected2.contains(entry))
        }
        assert_eq!(actual2.len(), expected2.len());
    }

    #[test]
    fn can_calc_shortest_path() {
        assert_eq!(
            find_shortest_path(&Burrow::from(&".A......BCDABCD".to_string())),
            Some(2)
        );
        assert_eq!(
            find_shortest_path(&Burrow::from(&".B.....A.CDABCD".to_string())),
            Some(40)
        );
        assert_eq!(
            find_shortest_path(&Burrow::from(&".C.....AB.DABCD".to_string())),
            Some(600)
        );
        assert_eq!(
            find_shortest_path(&Burrow::from(&".......BACDABCD".to_string())),
            Some(46)
        );
        assert_eq!(find_shortest_path(&sample_start()), Some(12521));

        assert_eq!(
            find_shortest_path(&expand_burrow(&sample_start())),
            Some(44169)
        );
    }

    #[test]
    fn can_expand_burrow() {
        assert_eq!(
            format!("{}", expand_burrow(&sample_start())),
            ".......BCBDDCBADBACADCA"
        )
    }
}
