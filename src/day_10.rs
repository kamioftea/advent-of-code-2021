//! This is my solution for [Advent of Code - Day 10 - _Title_](https://adventofcode.com/2021/day/10)
//!
//!

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

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum ParseError {
    MISMATCH { expected: char, actual: char },
    UNEXPECTED(char),
}

fn sum_errors(input: &String) -> usize {
    input
        .lines()
        .map(check_line)
        .map(|res| match res {
            Err(MISMATCH {
                expected: _,
                actual: ')',
            }) => 3,
            Err(MISMATCH {
                expected: _,
                actual: ']',
            }) => 57,
            Err(MISMATCH {
                expected: _,
                actual: '}',
            }) => 1197,
            Err(MISMATCH {
                expected: _,
                actual: '>',
            }) => 25137,
            _ => 0usize,
        })
        .sum()
}

fn check_line(line: &str) -> Result<Vec<char>, ParseError> {
    let mut stack: Vec<char> = Vec::new();

    let braces = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    for chr in line.chars() {
        match chr {
            '(' | '[' | '{' | '<' => stack.push(chr),
            ')' | ']' | '}' | '>' => {
                if let Some(prev) = stack.pop() {
                    let &expected = braces.get(&prev).expect("Unexpected character on stack");

                    if chr != expected {
                        return Err(MISMATCH {
                            expected,
                            actual: chr,
                        });
                    }
                }
            }
            _ => return Err(UNEXPECTED(chr)),
        }
    }

    return Ok(stack
        .iter()
        .flat_map(|c| braces.get(c))
        .map(|&c| c)
        .rev() // FIFO
        .collect());
}

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
        .skip(mid)
        .next()
        .expect("Vec is long enough by def");
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
