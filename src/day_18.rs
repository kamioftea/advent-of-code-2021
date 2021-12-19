//! This is my solution for [Advent of Code - Day 18 - _Title_](https://adventofcode.com/2021/day/18)
//!
//! https://github.com/InfinityByTen/AoC-2021/blob/main/day18/src/main.rs

use itertools::Itertools;
use std::fs;

use crate::day_18::Direction::{LEFT, RIGHT};
use crate::day_18::SnailfishNumber::{Num, Pair};

#[derive(Eq, PartialEq, Debug, Clone)]
enum SnailfishNumber {
    Num(u8),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

impl<'a> From<&'a str> for SnailfishNumber {
    fn from(s: &str) -> Self {
        fn iter<'a>(chars: &mut dyn Iterator<Item = char>) -> SnailfishNumber {
            let chr = chars.next().unwrap();
            match chr {
                '[' => {
                    let first = iter(chars);
                    chars.next();
                    let second = iter(chars);
                    chars.next();
                    Pair(Box::new(first), Box::new(second))
                }
                num => Num(num.to_digit(10).unwrap() as u8),
            }
        }

        iter(&mut s.chars())
    }
}

impl SnailfishNumber {
    fn add(&self, other: &SnailfishNumber) -> SnailfishNumber {
        let mut combined = Pair(Box::new(self.clone()), Box::new(other.clone()));
        while combined.check_depth(0).is_some() || combined.check_numbers() {}
        combined
    }

    fn with(&mut self, dir: Direction, num: u8) {
        match (self, dir) {
            (Pair(_, b), LEFT) => b.with(LEFT, num),
            (Pair(a, _), RIGHT) => a.with(RIGHT, num),
            (Num(n), _) => {
                *n += num;
            }
        }
    }

    fn num(&self) -> u8 {
        match self {
            Num(num) => *num,
            _ => panic!("SmallfishNumber.num() called on Pair"),
        }
    }

    fn check_depth(&mut self, depth: u8) -> Option<(u8, u8)> {
        match self {
            Pair(left, right) if depth >= 4 => {
                let to_return = Some((left.num(), right.num()));
                *self = Num(0);
                to_return
            }
            Pair(left, right) => {
                if let Some((to_left, to_right)) = left.check_depth(depth + 1) {
                    right.with(RIGHT, to_right);
                    Some((to_left, 0))
                } else if let Some((to_left, to_right)) = right.check_depth(depth + 1) {
                    left.with(LEFT, to_left);
                    Some((0, to_right))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn check_numbers(&mut self) -> bool {
        match self {
            Pair(left, right) => left.check_numbers() || right.check_numbers(),
            Num(n) if *n > 9 => {
                *self = Pair(Box::new(Num(*n / 2)), Box::new(Num(*n / 2 + *n % 2)));
                true
            }
            _ => false,
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
            Num(n) => *n as usize,
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-18-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 18.
pub fn run() {
    let contents = fs::read_to_string("res/day-18-input").expect("Failed to read file");
    let numbers = parse_input(&contents);

    let sum = add_numbers(&numbers);
    println!("The magnitude of the sum is: {}.", sum.magnitude());

    let max_sum = max_sum(&numbers);
    println!("The maximum sum of the permutations is: {}.", max_sum);
}

fn parse_input(input: &String) -> Vec<SnailfishNumber> {
    input.lines().map(SnailfishNumber::from).collect()
}

fn add_numbers(numbers: &Vec<SnailfishNumber>) -> SnailfishNumber {
    let mut iter = numbers.iter();
    let first = iter.next().unwrap();
    iter.fold(first.clone(), |acc, num| acc.add(num))
}

fn max_sum(numbers: &Vec<SnailfishNumber>) -> usize {
    numbers
        .iter()
        .permutations(2)
        .map(|permutation| permutation[0].add(permutation[1]).magnitude())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_18::SnailfishNumber::{Num, Pair};
    use crate::day_18::{add_numbers, parse_input};
    use crate::day_18::{max_sum, SnailfishNumber};

    #[test]
    fn can_parse() {
        let input = "[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]"
            .to_string();

        let expected = Vec::from([
            Pair(Box::new(Num(1)), Box::new(Num(2))),
            Pair(
                Box::new(Pair(Box::new(Num(1)), Box::new(Num(2)))),
                Box::new(Num(3)),
            ),
            Pair(
                Box::new(Num(9)),
                Box::new(Pair(Box::new(Num(8)), Box::new(Num(7)))),
            ),
            Pair(
                Box::new(Pair(Box::new(Num(1)), Box::new(Num(9)))),
                Box::new(Pair(Box::new(Num(8)), Box::new(Num(5)))),
            ),
            Pair(
                Box::new(Pair(
                    Box::new(Pair(
                        Box::new(Pair(Box::new(Num(1)), Box::new(Num(2)))),
                        Box::new(Pair(Box::new(Num(3)), Box::new(Num(4)))),
                    )),
                    Box::new(Pair(
                        Box::new(Pair(Box::new(Num(5)), Box::new(Num(6)))),
                        Box::new(Pair(Box::new(Num(7)), Box::new(Num(8)))),
                    )),
                )),
                Box::new(Num(9)),
            ),
        ]);

        parse_input(&input)
            .iter()
            .zip(expected.iter())
            .for_each(|(actual, expected)| assert_eq!(actual, expected))
    }

    #[test]
    fn can_explode() {
        let mut sfn = SnailfishNumber::from("[[[[[9,8],1],2],3],4]");
        assert_eq!(sfn.check_depth(0), Some((9, 0)));
        assert_eq!(sfn, SnailfishNumber::from("[[[[0,9],2],3],4]"));

        let mut sfn = SnailfishNumber::from("[[6,[5,[4,[3,2]]]],1]");
        assert_eq!(sfn.check_depth(0), Some((0, 0)));
        assert_eq!(sfn, SnailfishNumber::from("[[6,[5,[7,0]]],3]"));

        let mut sfn = SnailfishNumber::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        assert_eq!(sfn.check_depth(0), Some((0, 0)));
        assert_eq!(
            sfn,
            SnailfishNumber::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        );
    }

    #[test]
    fn can_check_numbers() {
        let mut nine = Num(9);
        assert_eq!(nine.check_numbers(), false);
        assert_eq!(nine, Num(9));

        let mut ten = Num(10);
        assert_eq!(ten.check_numbers(), true);
        assert_eq!(ten, Pair(Box::new(Num(5)), Box::new(Num(5))));

        let mut eleven = Num(11);
        assert_eq!(eleven.check_numbers(), true);
        assert_eq!(eleven, Pair(Box::new(Num(5)), Box::new(Num(6))));

        let mut nested = Pair(Box::new(Num(1)), Box::new(Num(12)));
        assert_eq!(nested.check_numbers(), true);
        assert_eq!(
            nested,
            Pair(
                Box::new(Num(1)),
                Box::new(Pair(Box::new(Num(6)), Box::new(Num(6))))
            )
        );
    }

    #[test]
    fn can_add() {
        let lhs = SnailfishNumber::from("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let rhs = SnailfishNumber::from("[1,1]");
        let result = lhs.add(&rhs);
        assert_eq!(
            result,
            SnailfishNumber::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        )
    }

    #[test]
    fn can_add_lines() {
        let input = "[1,1]
[2,2]
[3,3]
[4,4]"
            .to_string();
        assert_eq!(
            add_numbers(&parse_input(&input)),
            SnailfishNumber::from("[[[[1,1],[2,2]],[3,3]],[4,4]]")
        );

        let input2 = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"
            .to_string();

        assert_eq!(
            add_numbers(&parse_input(&input2)),
            SnailfishNumber::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn can_calculate_magnitude() {
        Vec::from([
            (SnailfishNumber::from("[[1,2],[[3,4],5]]"), 143usize),
            (
                SnailfishNumber::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
                1384,
            ),
            (SnailfishNumber::from("[[[[1,1],[2,2]],[3,3]],[4,4]]"), 445),
            (SnailfishNumber::from("[[[[3,0],[5,3]],[4,4]],[5,5]]"), 791),
            (SnailfishNumber::from("[[[[5,0],[7,4]],[5,5]],[6,6]]"), 1137),
            (
                SnailfishNumber::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
                3488,
            ),
        ])
        .iter()
        .for_each(|(num, result)| assert_eq!(num.magnitude(), *result));

        let homework = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            .to_string();

        assert_eq!(add_numbers(&parse_input(&homework)).magnitude(), 4140);
    }

    #[test]
    fn can_find_max_sum() {
        let homework = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            .to_string();

        assert_eq!(max_sum(&parse_input(&homework)), 3993);
    }
}
