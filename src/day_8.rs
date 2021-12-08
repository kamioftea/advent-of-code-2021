//! This is my solution for [Advent of Code - Day 8 - _Seven Segment Search_](https://adventofcode.com/2021/day/8)
//!
//!

use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
struct Display {
    digits: HashMap<usize, usize>,
    output: Vec<Digit>,
}

impl Display {
    fn get_output(&self) -> usize {
        self.output
            .iter()
            .map(|d| {
                self.digits
                    .get(&d.bits)
                    .expect(format!("Missing {:?}", d).as_str())
            })
            .fold(0, |acc, digit| acc * 10 + digit)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Digit {
    bits: usize,
    len: usize,
}

impl FromStr for Digit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digit = Digit { bits: 0, len: 0 };
        s.chars().for_each(|c| {
            let pos = (c as usize) - ('a' as usize);
            digit.bits = digit.bits | (1 << pos);
            digit.len = digit.len + 1;
        });

        Ok(digit)
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-8-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 8.
pub fn run() {
    let contents = fs::read_to_string("res/day-8-input").expect("Failed to read file");
    let displays = parse_input(contents);

    let unique_count = count_unique(&displays);
    println!(
        "There are {} digits with unique lengths in the output.",
        unique_count
    );

    let output_total: usize = displays.iter().map(Display::get_output).sum();

    println!("The sum of the outputs is {}.", output_total);
}

fn parse_input(input: String) -> Vec<Display> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Display {
    fn parse_digit(digit: &str) -> Digit {
        digit.parse().unwrap()
    }

    if let Some((digit_strings, output_strings)) = line.split_once(" | ") {
        let mut digits: HashMap<usize, usize> = HashMap::new();

        let unassigned_digits: Vec<Digit> = digit_strings.split(' ').map(parse_digit).collect();
        let mut four: Option<usize> = None;
        let mut one: Option<usize> = None;
        let mut nine: Option<usize> = None;

        //First pass - unique lengths
        unassigned_digits.iter().for_each(|digit| {
            match digit.len {
                2 => {
                    digits.insert(digit.bits, 1);
                    one = Some(digit.bits);
                }
                3 => {
                    digits.insert(digit.bits, 7);
                }
                4 => {
                    digits.insert(digit.bits, 4);
                    four = Some(digit.bits);
                }
                7 => {
                    digits.insert(digit.bits, 8);
                }
                _ => {}
            };
        });

        // Second pass - filter 6, 9, 0
        unassigned_digits
            .iter()
            .filter(|digit| digit.len == 6)
            .for_each(|digit| {
                // 9 intersects with 4, 6 and 0 don't.
                if digit.bits & four.expect("digits missing 4") == four.unwrap() {
                    digits.insert(digit.bits, 9);
                    nine = Some(digit.bits);
                }
                // 0 and 9 intersect with 1, but 9 is already captured above
                else if digit.bits & one.expect("digits missing 1") == one.unwrap() {
                    digits.insert(digit.bits, 0);
                }
                // Can only be 6
                else {
                    digits.insert(digit.bits, 6);
                }
            });

        // Third pass - filter 2, 3, 5
        unassigned_digits
            .iter()
            .filter(|digit| digit.len == 5)
            .for_each(|digit| {
                // 1 is included in 3, but not 2 or 5
                if digit.bits & one.expect("digits missing 1") == one.unwrap() {
                    digits.insert(digit.bits, 3);
                }
                // 5 is included in 9, but 5 and 2 are not
                else if digit.bits & nine.expect("digits missing 9") == digit.bits {
                    digits.insert(digit.bits, 5);
                }
                // Can only be 2
                else {
                    digits.insert(digit.bits, 2);
                }
            });

        let output = output_strings.split(' ').map(parse_digit).take(4).collect();

        return Display { digits, output };
    }

    panic!("Bad line: '{}'", line)
}

fn count_unique(displays: &Vec<Display>) -> usize {
    displays
        .iter()
        .map(|display| {
            return display
                .output
                .iter()
                .map(|digit| display.digits.get(&digit.bits))
                .filter(|maybe_digit| {
                    if let Some(d) = maybe_digit {
                        [1, 4, 7, 8].contains(d)
                    } else {
                        false
                    }
                })
                .count();
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;

    use crate::day_8::{count_unique, parse_input, parse_line, Digit, Display};

    #[test]
    fn can_parse_digit() {
        assert_eq!(
            Digit::from_str("acedgfb"),
            Ok(Digit {
                bits: 0b1111111,
                len: 7
            })
        );

        assert_eq!(
            Digit::from_str("cdfbe"),
            Ok(Digit {
                bits: 0b0111110,
                len: 5
            })
        );

        assert_eq!(
            Digit::from_str("gcdfa"),
            Ok(Digit {
                bits: 0b1101101,
                len: 5
            })
        );

        assert_eq!(
            Digit::from_str("fbcad"),
            Ok(Digit {
                bits: 0b0101111,
                len: 5
            })
        );

        assert_eq!(
            Digit::from_str("dab"),
            Ok(Digit {
                bits: 0b0001011,
                len: 3
            })
        );

        assert_eq!(
            Digit::from_str("cefabd"),
            Ok(Digit {
                bits: 0b0111111,
                len: 6
            })
        );

        assert_eq!(
            Digit::from_str("cdfgeb"),
            Ok(Digit {
                bits: 0b1111110,
                len: 6
            })
        );

        assert_eq!(
            Digit::from_str("eafb"),
            Ok(Digit {
                bits: 0b0110011,
                len: 4
            })
        );

        assert_eq!(
            Digit::from_str("cagedb"),
            Ok(Digit {
                bits: 0b1011111,
                len: 6
            })
        );

        assert_eq!(
            Digit::from_str("ab"),
            Ok(Digit {
                bits: 0b000011,
                len: 2
            })
        );
    }

    #[test]
    fn can_parse_lines() {
        let display = parse_line(get_sample_line());

        let digits = HashMap::from([
            (Digit::from_str("cagedb").unwrap().bits, 0usize),
            (Digit::from_str("ab").unwrap().bits, 1usize),
            (Digit::from_str("gcdfa").unwrap().bits, 2usize),
            (Digit::from_str("fbcad").unwrap().bits, 3usize),
            (Digit::from_str("eafb").unwrap().bits, 4usize),
            (Digit::from_str("cdfbe").unwrap().bits, 5usize),
            (Digit::from_str("cdfgeb").unwrap().bits, 6usize),
            (Digit::from_str("dab").unwrap().bits, 7usize),
            (Digit::from_str("acedgfb").unwrap().bits, 8usize),
            (Digit::from_str("cefabd").unwrap().bits, 9usize),
        ]);

        let output = vec![
            Digit::from_str("cdfeb").unwrap(),
            Digit::from_str("fcadb").unwrap(),
            Digit::from_str("cdfeb").unwrap(),
            Digit::from_str("cdbaf").unwrap(),
        ];

        assert_eq!(display, Display { digits, output });
    }

    #[test]
    fn can_calculate_output() {
        assert_eq!(parse_line(get_sample_line()).get_output(), 5353);

        let expected_outputs: Vec<usize> =
            vec![8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];

        parse_input(get_sample_input())
            .iter()
            .zip(expected_outputs)
            .for_each(|(display, expected_output)| {
                assert_eq!(display.get_output(), expected_output)
            })
    }

    fn get_sample_line() -> &'static str {
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
    }

    #[test]
    fn can_count_unique() {
        let displays: Vec<Display> = parse_input(get_sample_input());

        assert_eq!(count_unique(&displays), 26);
    }

    fn get_sample_input() -> String {
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            .to_string()
    }
}
