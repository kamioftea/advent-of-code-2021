//! This is my solution for [Advent of Code - Day 10 - _Syntax Scoring_](https://adventofcode.com/2021/day/10)
//!
//! Today was about matching different types of braces, firstly (part one) to check that the braces matched, and
//! secondly (part two) where closing braces had been missed off the end of the string to close them. Implementing
//! the state as a stack of characters seemed sensible. An opening brace triggers putting the expected closing brace
//! on the stack, a closing brace pops the expected brace off the stack and returns an error if it doesn't match.
//! Anything left on the stack at the end is returned as the required characters to 'autocomplete' the rest of the
//! line. I originally implemented this as putting the opening brace on the stack, but that required doing the mapping
//! from opening to closing brace character both when checking for a match and when returning outstanding characters.
//! It was cleaner to do the mapping upfront.
//!
//! [`check_line`] do most of the work for both parts. It was a good opportunity to use Rust's [`Result`] type, and I
//! implemented a custom enum [`ParseError`] to capture the errors due to syntax, vs the errors I needed to include
//! to satisfy edge cases that weren't in the puzzle input - namely characters that weren't any of the 8 braces, and
//! the case where there is one or more closing braces encountered when the stack is empty. [`sum_errors`] wraps
//! [`check_line`] for part one, filtering out ny lines that don't return a [`ParseError::MISMATCH`], mapping those
//! to the correct score, and summing the results. [`score_line_autocomplete`] takes the characters returned from a
//! successfully parsed line and folds them into the expected score. [`median_autocomplete_score`] handles the
//! plumbing of getting the list of successful [`check_line`] results, mapping them to the autocomplete score and
//! returning the median score required for part two's puzzle result.
//!
//! One final piece of trivia, I looked into using the characters' unicode points to avoid using a hash map, but they
//! were not consistent. `(` and `)` are consecutive, but the others are all separated by one character.
//! ```
//! println!("{}", "()[]{}<>".chars().map(|c| c as usize).join(", "));
//! // 40, 41, 91, 93, 123, 125, 60, 62
//! ```

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

use crate::day_10::ParseError::{MISMATCH, UNEXPECTED};

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-10-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 10.
pub fn run() {
    let contents = fs::read_to_string("res/day-10-input").expect("Failed to read file");

    let syntax_error_score = sum_errors(&contents);
    println!("Syntax error score: {}", syntax_error_score);

    let autocomplete_score = median_autocomplete_score(&contents);
    println!("Autocomplete score: {}", autocomplete_score)
}

/// Used to indicate an error when parsing strings of braces
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum ParseError {
    /// A closing brace was encountered that doesn't match the expected character from the corresponding opening brace
    MISMATCH { expected: char, actual: char },
    /// Any other unexpected character i.e. not part of one of the four brace pairs, or a closing brace without a
    /// corresponding opening brace.
    UNEXPECTED(char),
}

/// Find all the lines in the input that return a mismatch error and sum a score based on the character that was 
/// incorrect.
#[rustfmt::skip] // Keep match readable
fn sum_errors(input: &String) -> usize {
    input
        .lines()
        .map(check_line)
        .map(|res| match res {
            Err(MISMATCH { expected: _, actual: ')' }) => 3,
            Err(MISMATCH { expected: _, actual: ']' }) => 57,
            Err(MISMATCH { expected: _, actual: '}' }) => 1197,
            Err(MISMATCH { expected: _, actual: '>' }) => 25137,
            _ => 0usize,
        })
        .sum()
}

/// Given a string, either return the list of closing braces needed to completely match the opening braces in order,
/// or return a [`ParseError`] if a closing brace that doesn't match the expected value at any point in the string.
fn check_line(line: &str) -> Result<Vec<char>, ParseError> {
    // Stack of the currently expected closing braces
    let mut stack: Vec<char> = Vec::new();

    let braces = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    for chr in line.chars() {
        match chr {
            // It's easier to map the opening => closing brace here as it keeps it in one place
            '(' | '[' | '{' | '<' => stack.push(*braces.get(&chr).expect("Unreachable")),
            ')' | ']' | '}' | '>' => {
                if let Some(expected) = stack.pop() {
                    if chr != expected {
                        return Err(MISMATCH {
                            expected,
                            actual: chr,
                        });
                    }
                } else {
                    return Err(UNEXPECTED(chr));
                }
            }
            _ => return Err(UNEXPECTED(chr)),
        }
    }

    // We need t reveser the stack to keep the First In First Out ordering
    let autocomplete = stack.iter().map(|&c| c).rev().collect();

    return Ok(autocomplete);
}

/// Given the list of braces needed to complete a string, fold them into the autocomplete score
fn score_line_autocomplete(line: Vec<char>) -> usize {
    line.iter()
        .flat_map(|c| match c {
            ')' => Some(1),
            ']' => Some(2),
            '}' => Some(3),
            '>' => Some(4),
            _ => None,
        })
        .fold(0, |acc, score| acc * 5 + score)
}

/// Find all the lines in the input that are valid, work out the autocomplete score for each, and return the median
/// score.
fn median_autocomplete_score(input: &String) -> usize {
    let scores: Vec<usize> = input
        .lines()
        .flat_map(|l| check_line(l).ok())
        .map(score_line_autocomplete)
        .collect();

    let mid = scores.len() / 2; // always odd # by spec
    return *scores
        .iter()
        .sorted()
        // len() / 2 is always the floor, so skip that many ...
        .skip(mid)
        // And the midpoint will then be the next one
        .next()
        // Unless the input is empty this will always be set
        .unwrap_or(&0usize);
}

#[cfg(test)]
mod tests {
    use crate::day_10::ParseError::MISMATCH;
    use crate::day_10::{
        check_line, median_autocomplete_score, score_line_autocomplete, sum_errors,
    };

    #[test]
    fn can_check_valid_line() {
        let valid_lines = [
            "([])",
            "{()()()}",
            "<([{}])>",
            "[<>({}){}[([])<>]]",
            "(((((((((())))))))))",
        ];

        valid_lines
            .iter()
            .for_each(|&line| assert_eq!(check_line(line), Ok(vec![])));
    }

    #[test]
    fn can_check_invalid_line() {
        let invalid_lines = [
            (
                "{([(<{}[<>[]}>{[]{[(<()>",
                MISMATCH {
                    expected: ']',
                    actual: '}',
                },
            ),
            (
                "[[<[([]))<([[{}[[()]]]",
                MISMATCH {
                    expected: ']',
                    actual: ')',
                },
            ),
            (
                "[{[{({}]{}}([{[{{{}}([]",
                MISMATCH {
                    expected: ')',
                    actual: ']',
                },
            ),
            (
                "[<(<(<(<{}))><([]([]()",
                MISMATCH {
                    expected: '>',
                    actual: ')',
                },
            ),
            (
                "<{([([[(<>()){}]>(<<{{",
                MISMATCH {
                    expected: ']',
                    actual: '>',
                },
            ),
        ];

        invalid_lines
            .iter()
            .for_each(|&(line, err)| assert_eq!(check_line(line), Err(err)));
    }

    #[test]
    fn can_check_incomplete_line() {
        let incomplete_lines = [
            ("[({(<(())[]>[[{[]{<()<>>", "}}]])})]"),
            ("[(()[<>])]({[<{<<[]>>(", ")}>]})"),
            ("(((({<>}<{<{<>}{[]{[]{}", "}}>}>))))"),
            ("{<[[]]>}<{[{[{[]{()[[[]", "]]}}]}]}>"),
            ("<{([{{}}[<[[[<>{}]]]>[]]", "])}>"),
        ];

        incomplete_lines.iter().for_each(|&(line, expected)| {
            assert_eq!(check_line(line), Ok(expected.chars().collect()))
        })
    }

    #[test]
    fn can_score_incomplete_line() {
        let incomplete_lines: [(&str, usize); 5] = [
            ("}}]])})]", 288957),
            (")}>]})", 5566),
            ("}}>}>))))", 1480781),
            ("]]}}]}]}>", 995444),
            ("])}>", 294),
        ];

        incomplete_lines.iter().for_each(|&(remaining, expected)| {
            assert_eq!(
                score_line_autocomplete(remaining.chars().collect()),
                expected
            )
        })
    }

    fn sample_input() -> String {
        "[({(<(())[]>[[{[]{<()<>>\n\
             [(()[<>])]({[<{<<[]>>(\n\
             {([(<{}[<>[]}>{[]{[(<()>\n\
             (((({<>}<{<{<>}{[]{[]{}\n\
             [[<[([]))<([[{}[[()]]]\n\
             [{[{({}]{}}([{[{{{}}([]\n\
             {<[[]]>}<{[{[{[]{()[[[]\n\
             [<(<(<(<{}))><([]([]()\n\
             <{([([[(<>()){}]>(<<{{\n\
             <{([{{}}[<[[[<>{}]]]>[]]"
            .to_string()
    }

    #[test]
    fn can_sum_errors() {
        assert_eq!(sum_errors(&sample_input()), 26397);
    }

    #[test]
    fn can_get_median() {
        assert_eq!(median_autocomplete_score(&sample_input()), 288957)
    }
}
