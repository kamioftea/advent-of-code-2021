//! This is my solution for [Advent of Code - Day 8 - _Seven Segment Search_](https://adventofcode.com/2021/day/8)
//!
//! Today had a lot of description for a relatively simple part one, that seemed to be leading into
//! a much harder part two. The puzzle input is a sequence of lines in two parts, ten unique digits
//! (i.e. 0-9) in a random order, then 4 other digits. But each digit is shown as the set of bars
//! they light up on a digital display, each bar represented by a letter a-g, in a random order.
//! To further complicate things, the wires that light up the bars have got swapped around - and the
//! actual bars that light up is only consistent for each line.
//!
//! To show some examples:
//!
//! ```text
//! Wiring correct - Order: a,b,c,d,e,f,g
//!   1:      4:      8:     Sets shown when labelled in reading order
//!  ....    ....    aaaa    1: cf, fc
//! .    c  b    c  b    c   4: bcfd, cdbf, ...
//! .    c  b    c  b    c   8: abcdefg, gfacbed, ...
//!  ....    dddd    dddd  
//! .    f  .    f  e    f
//! .    f  .    f  e    f
//!  ....    ....    gggg
//!
//! Wiring mixed up - Order: e,c,d,g,f,a,b
//!   1:      4:      8:     Example sets that could be shown when labelled in reading order
//!  ....    ....    eeee    1: be, eb
//! c    .  c    d  c    d   4: bceg, gcbe, ...
//! c    .  c    d  c    d   8: abcdefg, gfacbed, ... // doesn't change all are lit up
//!  ....    ....    gggg  
//! f    .  f    .  f    a
//! f    .  f    .  f    a
//!  ....    bbbb    bbbb
//! ```
//!
//! The overall puzzle seemed to be leaning towards working out what numbers were showing on each
//! line. For part one it was noted that digits 1, 4, 7 and 8 each light up a unique number of bars.
//! Due to this they can be identified by just counting the bars in the set. After all that preamble
//! the challenge for part one was simple - count the number of digits in the output strings that
//! are known to be 1, 4, 7, or 8 by their length.
//!
//! One way to solve this would be to just discard the set of ten unique combinations, reduce the
//! outputs to their sizes filter out those of size 2 (1s), size 3 (7s), size 4 (4s), and size 7
//! (8s). I felt this would be wasted work however as for part two I was going to need to identify
//! which of the digits was which. I instead decided to implement an internal representation of each
//! line as a [`Display`] that had a map of the character set to the integer digit, plus the unknown
//! digits for later analysis. To implement the minimum needed for part one, just the map entries
//! for 1, 4, 7 and 8 would need to be created. The solution for part one is to filter the output
//! lists to those that map to 1, 4, 7, or 8 in the digit map, count each of those and sum those
//! counts for all displays. [`count_unique`] implements this.
//!
//! The key piece of functionality, where all the hard work for part one and two is done is
//! [`parse_line`]. This takes the string line, and builds that map of known character sets to
//! integer digits. I'd originally implemented the digits as `HashSet<char>`s. It needs to be a type
//! where the order of the characters doesn't matter. I also suspect it needs to be able to do
//! efficient intersections of the sets, as whilst getting to grips with the specification I'd
//! already noticed you can work out 0, 6 and 9 once you know 1 and 4 because 4 is a subset of 9,
//! and 1 is a subset of 0 and 9. This hit a snag in that a `HashSet` doesn't itself implement
//! `Hash` so can't be used as a key. I pondered sorting the strings and using that as the key, but
//! the logic to calculate their intersections is pretty complex in Rust. Then I remembered
//! [`crate::day_3`] and went with representing each of the 7 lines as a bit (giving a unique number
//! for each set). Further, bitwise `&` can be used to efficiently do the intersections. I gave this
//! it's own type [`Digit`], and hooked into the built-in FromStr trait to make creating these from
//! the input cleaner. For part one only the first pass through the 10 digits was implemented, but
//! I'm happy to report that my plan worked and I did not need to change my implementation of part
//! one to cope with the changes added to solve part two.
//!
//! Part two required working out the remaining six digits, interpreting the four output digits as a
//! 4 digit decimal number, and summing those to get the puzzle solution. My implementation for part
//! one was a good start to this. As noted above, 0, 6 and 9 can be worked out from 1 and 4. So I
//! added a second pass through the unique digits to grab the three with length 6 that correspond to
//! 0, 6 and 9 in some order. These could then be tested as follows:
//! - Intersect the set of digits for #4 with the current set. If that's unchanged, #4 is a subset
//!   and the current digit must be 9.
//! - Intersect the set of digits for #1 with the current set. If unchanged it's either 0 or 9, and
//!   it can't be 9 as it failed the first check. It must be 0.
//! - Otherwise it's 6 by process of elimination.
//!
//! I added these to the test suite and confirmed it was all working.
//!
//! For 2, 3, and 5 a similar argument can be made, so a third loop was made just considering the
//! digits with sets of length 5.
//! - 1 is a subset of 3, but not 2 or 5 so intersect with the set for #1, if unchanged it must be 3
//! - 5 is a subset of 9, but 2 (and 3) are not. This still intersects the current set with the set
//!   for #9, but instead check that the result is the same as the current set, since 5 is the
//!   subset.
//! - Again we can otherwise assume the current set represents 2 by process of elimination.
//!
//! Again this was added to the test suite and implemented. All ten digits are now known, so the
//! final step was to implement [`Display::get_output`] that converted the four output digits into
//! the equivalent decimal `usize`, and I used built in iterate -> map -> sum to reduce the input
//! to the solution.

use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
struct Display {
    /// Map of the sets of lines and the decimal digit they represent
    digits: HashMap<usize, usize>,
    /// The four output digits
    output: Vec<Digit>,
}

impl Display {
    /// Map each output digit to the corresponding decimal and combine by folding.
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
    /// The set of bits that are lit up with a being least significant and g being most
    bits: usize,
    /// The number of bits that are set. This is known at creation so cache to avoid recalculating
    /// later
    len: usize,
}

impl FromStr for Digit {
    type Err = ();

    /// Convert the string puzzle representation to a [`Digit`].
    ///
    /// Technically this accepts more than just sets of a-g, but that does not need to be handled
    /// for the puzzle input.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digit = Digit { bits: 0, len: 0 };
        s.chars().for_each(|c| {
            // chars can be converted to their ascii int just by caching - so this calculates the
            // offset from 'a'
            let pos = (c as usize) - ('a' as usize);
            // shift left once for each digit
            digit.bits = digit.bits | (1 << pos);
            // track the number of bits set
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

/// Utility for the whole puzzle input that just defers to [`parse_line`] for each line of the
/// input.
fn parse_input(input: String) -> Vec<Display> {
    input.lines().map(|line| parse_line(line)).collect()
}

/// This does all of the hard work. Once the input is turned into a [`Display`] the puzzle solution
/// is easy to calculate. The string is split up by the | and ' ' delimeteters and the resulting 14
/// digits are parsed using [`Digit::from_str`]. Then there are the three loops discussed in the
/// preamble that identify 1, 4, 7 and 8; 0, 6 and 9, then finally 2, 3, and 5. Building the digits
/// map needed for [`Display`] as numbers are found.
fn parse_line(line: &str) -> Display {
    // Extracted to avoid repetition, also can use more implicit typing this way.
    fn parse_digit(digit: &str) -> Digit {
        digit.parse().unwrap()
    }

    // First split into the digits and output
    if let Some((digit_strings, output_strings)) = line.split_once(" | ") {
        // Setup an empty map to be populated as we resolve each digit
        let mut digits: HashMap<usize, usize> = HashMap::new();

        // First interpret the two halves into the internal Digit representation
        let unassigned_digits: Vec<Digit> = digit_strings.split(' ').map(parse_digit).collect();
        let output = output_strings.split(' ').map(parse_digit).take(4).collect();

        // Cache for the bit sets we'll need to isolate other digits later
        let mut four: Option<usize> = None;
        let mut one: Option<usize> = None;
        let mut nine: Option<usize> = None;

        // First pass - capture digits that have a unique length
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

        // ---- Passes two and three were implemented for part two - part one stopped here ----

        // Second pass - capture 6, 9, 0 using their intersection with unique digits 1 and 4
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
                // Can only be 6 by process of elimination
                else {
                    digits.insert(digit.bits, 6);
                }
            });

        // Third pass - capture 2, 3, 5 using their intersections with digits 1 and 9
        unassigned_digits
            .iter()
            .filter(|digit| digit.len == 5)
            .for_each(|digit| {
                // 1 is included in 3, but not 2 or 5
                if digit.bits & one.expect("digits missing 1") == one.unwrap() {
                    digits.insert(digit.bits, 3);
                }
                // 5 is included in 9, but not in 2 and 3 are not
                else if digit.bits & nine.expect("digits missing 9") == digit.bits {
                    digits.insert(digit.bits, 5);
                }
                // Can only be 2 by process of elimination
                else {
                    digits.insert(digit.bits, 2);
                }
            });

        return Display { digits, output };
    }

    // Failed to match two sections split by | - should be unreachable for the puzzle input, so
    // just panic!
    panic!("Bad line: '{}'", line)
}

/// Given a list of parsed displays, count the total number of 1s, 4s, 7s, and 8s in their outputs
fn count_unique(displays: &Vec<Display>) -> usize {
    displays
        .iter()
        .map(|display| {
            return display
                .output
                .iter()
                // Flat map converts to the digit and filters out any unmatched digits to support
                // the part one implementation did not include a total mapping for each digit
                .flat_map(|digit| display.digits.get(&digit.bits))
                // Limit to just the four digits that part one cares about so that this still
                // works when part two is implemented.
                .filter(|digit| [1, 4, 7, 8].contains(digit))
                // Just need to know how many match
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
