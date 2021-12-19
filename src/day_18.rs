//! This is my solution for [Advent of Code - Day 18 - _Snailfish_](https://adventofcode.com/2021/day/18)
//!
//! Today was doing convoluted arithmetic on 'Snailfish Numbers', which are made up of a binary tree with digits at the
//! leaves. The representation was easy enough, but I had to box the non-leaf nodes to prevent the compiler
//! complaining about the type being potentially infinite. I tried just using references, but it made satisfying the
//! borrow checker very difficult. This resulted in [`SnailfishNumber`] and [`SnailfishNumber::from`], a recursive
//! function that parses a line of the input (each is one number).
//!
//! The main difficulty of today was implementing [`SnailfishNumber::add`]. The actual addition is simple, but the
//! resulting checks that the invariants on the new [`SnailfishNumber`] hold: no pair deeper than level 4, no
//! leaves that are not single digits, and resolving them if they don't involved some complex recursion and pointer
//! manipulation. [`SnailfishNumber::check_depth`] handles checking if a pair is too deep, and also if it finds one,
//! handles passing the numbers of that pair left/right as it unwinds the recursion. [SnailfishNumber::check_digits]
//! handles splitting leaves with numbers that are not single digits into the relevant pair.
//!
//! Finally [`SnailfishNumber::magnitude`] implements recursively reducing a [`SnailfishNumber`] tree into a single
//! number for calculating the result. With these in place, [`add_numbers`] folds each line of the input into the first
//! number using [`SnailfishNumber::add`] for the solution to part one. [`max_sum`] uses [Itertools::permutations] to
//! match up each pair of numbers in both orders, map them to the magnitude of the sum, and reduce that to the maximum.

use itertools::Itertools;
use std::fs;

use crate::day_18::Direction::{LEFT, RIGHT};
use crate::day_18::SnailfishNumber::{Num, Pair};

/// Represents a snailfish number as a binary tree
#[derive(Eq, PartialEq, Debug, Clone)]
enum SnailfishNumber {
    /// Leaf node
    Num(u8),
    /// Branch node - branches need to be boxed so that it has a constant size
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

/// When a pair is exploding due to being too deep, the number that still needs to be assigned is passed up to the
/// parent. This indicates which way it is travelling / which half of the pair it came from.
#[derive(Eq, PartialEq, Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

impl<'a> From<&'a str> for SnailfishNumber {
    /// Parse a line of the input as a [`SnailfishNumber`]
    fn from(s: &str) -> Self {
        fn iter<'a>(chars: &mut dyn Iterator<Item = char>) -> SnailfishNumber {
            let chr = chars.next().unwrap();
            match chr {
                // Start of a pair, recursively build each side
                '[' => {
                    let first = iter(chars);
                    chars.next(); // The comma
                    let second = iter(chars);
                    chars.next(); // the closing brace
                    Pair(Box::new(first), Box::new(second))
                }
                num => Num(num.to_digit(10).unwrap() as u8),
            }
        }

        iter(&mut s.chars())
    }
}

impl SnailfishNumber {
    /// Combine the two halves into a new [`SnailfishNumber::Pair`], then repeatedly call
    /// [`SnailfishNumber::check_depth`], and [`SnailfishNumber::check_digits`] until neither change the tree.
    fn add(&self, other: &SnailfishNumber) -> SnailfishNumber {
        let mut combined = Pair(Box::new(self.clone()), Box::new(other.clone()));
        while combined.check_depth(0).is_some() || combined.check_digits() {}
        combined
    }

    /// Utility used by [`SnailfishNumber::check_depth`] to add one half of a pair to the next digit on the same side
    /// of the tree. `dir` indicates which way this number is travelling, and `num` is the actual digit to be added.
    fn with(&mut self, dir: Direction, num: u8) {
        // Once the digit has been 'dropped off', the `0` is still passed up to parents as a No-op so that the type is
        // consistent. If the number is 0 we can therefore abort early.
        if num == 0 {
            return;
        }

        // Slightly confusing, but of the digit is travelling leftward, i.e. it came from the left-hand side of a pair
        // then the next number leftwards will be on the right-hand side of its pair (if it's in one)
        match (self, dir) {
            (Pair(_, b), LEFT) => b.with(LEFT, num),
            (Pair(a, _), RIGHT) => a.with(RIGHT, num),
            // we've found where the number should be, add it to the current value. If this takes it over 9, that
            // will be handled in a subsequent check
            (Num(n), _) => {
                *n += num;
            }
        }
    }

    /// Helper for extracting the value when you know you're at a leaf node.
    fn num(&self) -> u8 {
        match self {
            Num(num) => *num,
            _ => panic!("SnailfishNumber.num() called on Pair"),
        }
    }

    /// Recursively walk the tree, if a [`Pair`] is found that is too deep, replace it with a leaf node with value 0,
    /// then pass the two halves of the pair back up so that they can be added to the next leftmost and rightmost
    /// leaves. As this is unwinding, assign the relevant side of the returned digit pair to the other half before
    /// passing the rest back up. See also [`SnailfishNumber::with`] that helps with resolving the explosion.
    fn check_depth(&mut self, depth: u8) -> Option<(u8, u8)> {
        match self {
            // to deep, explode (the depth only increases by 1 with addition or digit checks, and all pairs
            // are exploded back to depth 3 before digit checks are run, so this can't increase beyond 4)
            Pair(left, right) if depth == 4 => {
                let to_return = Some((left.num(), right.num()));
                *self = Num(0);
                to_return
            }
            // Not too deep, so recur to the let then the right. If either return `Some` indicating that a pir deeper
            // in the tree exploded, pass the relevant number to the other half of the pair, and pass the other side
            // back up to be passed the other way by our parent.
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
            // Leaves can't be too deep
            _ => None,
        }
    }

    /// Recursively hunt for a leaf that is >9, i.e. not a digit, and of one is found split it into a pair, each leaf
    /// of which is half the original (rounding halves down and up respectively so that they sum to the original).
    /// Returns true if an oversize leaf was found and split, false otherwise.
    fn check_digits(&mut self) -> bool {
        match self {
            // Recursively check each half of a pair
            Pair(left, right) => left.check_digits() || right.check_digits(),
            // Found a leaf that is too large, split it in place and return true
            Num(n) if *n > 9 => {
                *self = Pair(Box::new(Num(*n / 2)), Box::new(Num(*n / 2 + *n % 2)));
                true
            }
            // Leaf is valid, no change needed
            _ => false,
        }
    }

    /// Recursively combine pairs into a single number using the formula `lhs x 3 + rhs x 2`.
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

/// Split the input into lines and parse each with [`SnailfishNumber::from`]
fn parse_input(input: &String) -> Vec<SnailfishNumber> {
    input.lines().map(SnailfishNumber::from).collect()
}

/// The solution to part one - fold the list of numbers into the first and return the resulting number. The puzzle
/// solution then converts this to its magnitude, but returning the full tree allows unit tests to compare this to the
/// expectation.
fn add_numbers(numbers: &Vec<SnailfishNumber>) -> SnailfishNumber {
    let mut iter = numbers.iter();
    let first = iter.next().unwrap();
    iter.fold(first.clone(), |acc, num| acc.add(num))
}

/// The solution to part two - uses [Itertools::permutations] to match up each pair of numbers in both orders, map
/// them to the magnitude of the sum, and reduce that to the maximum.
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
        assert_eq!(nine.check_digits(), false);
        assert_eq!(nine, Num(9));

        let mut ten = Num(10);
        assert_eq!(ten.check_digits(), true);
        assert_eq!(ten, Pair(Box::new(Num(5)), Box::new(Num(5))));

        let mut eleven = Num(11);
        assert_eq!(eleven.check_digits(), true);
        assert_eq!(eleven, Pair(Box::new(Num(5)), Box::new(Num(6))));

        let mut nested = Pair(Box::new(Num(1)), Box::new(Num(12)));
        assert_eq!(nested.check_digits(), true);
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
