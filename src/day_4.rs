//! This is my solution for [Advent of Code - Day 4 - _Title_](https://adventofcode.com/2021/day/4)
//!
//!

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs;

#[derive(Eq, PartialEq, Clone)]
struct BingoCard {
    numbers: HashMap<u8, (usize, usize)>,
    rows: [u8; 5],
    columns: [u8; 5],
}

impl Debug for BingoCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = self
            .numbers
            .keys()
            .sorted()
            .map(|k| format!("{}: {:?}", k, self.numbers.get(k).unwrap()))
            .collect();

        write!(f, "{}", parts.join("\n"))
    }
}

impl BingoCard {
    fn mark_number(&mut self, number: u8) -> bool {
        match self.numbers.get(&number) {
            Some(&(x, y)) => {
                self.columns[x] = self.columns[x] + 1;
                self.rows[y] = self.rows[y] + 1;
                self.numbers.remove(&number);

                self.columns[x] == 5 || self.rows[y] == 5
            }
            None => false,
        }
    }

    fn sum_remaining(&self) -> usize {
        self.numbers.keys().map(|&k| k as usize).sum()
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-4-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 4.
pub fn run() {
    let contents = fs::read_to_string("res/day-4-input").expect("Failed to read file");
    let (numbers, cards) = parse_input(contents);

    let (winning_card, winning_number) = play_bingo(&numbers, &cards);
    let winning_remaining = winning_card.sum_remaining();
    println!(
        "Remaining Sum: {} x Winning Number: {} = {}",
        winning_remaining,
        winning_number,
        winning_remaining * winning_number as usize
    );

    let (losing_card, losing_number) = play_bingo_until_last(&numbers, &cards);
    let losing_remaining = losing_card.sum_remaining();

    println!(
        "Remaining Sum: {} x Losing Number: {} = {}",
        losing_remaining,
        losing_number,
        losing_remaining * losing_number as usize
    );
}

fn play_bingo(numbers: &Vec<u8>, cards: &Vec<BingoCard>) -> (BingoCard, u8) {
    let mut my_cards = cards.to_vec();
    let size = my_cards.len();
    for &number in numbers {
        for i in 0..size {
            if let Some(card) = my_cards.get_mut(i) {
                if card.mark_number(number) {
                    return (card.clone(), number);
                }
            }
        }
    }

    panic!("No winner after numbers exhausted")
}

fn play_bingo_until_last(numbers: &Vec<u8>, cards: &Vec<BingoCard>) -> (BingoCard, u8) {
    let mut my_cards = cards.to_vec();
    let mut size = my_cards.len();
    for &number in numbers {
        let mut to_remove: Vec<usize> = Vec::new();
        for i in 0..size {
            let card = my_cards.get_mut(i).unwrap();
            match (card.mark_number(number), size) {
                (true, 1) => {
                    return (card.clone(), number);
                }
                (true, _) => {
                    to_remove.push(i);
                    size = size - 1;
                }
                (false, _) => {}
            }
        }
        for (offset, &i) in to_remove.iter().enumerate() {
            my_cards.remove(i - offset);
        }
    }

    panic!("No winner after numbers exhausted")
}

fn parse_input(contents: String) -> (Vec<u8>, Vec<BingoCard>) {
    let mut sections = contents.split("\n\n");
    let numbers: Vec<u8> = sections
        .next()
        .expect("Input file was empty")
        .split(",")
        .map(|num| {
            num.parse::<u8>()
                .expect(format!("Invalid number: '{}'", num).as_str())
        })
        .collect();

    let cards: Vec<BingoCard> = sections.map(|input| parse_card(input)).collect();

    (numbers, cards)
}

fn parse_card(input: &str) -> BingoCard {
    let splitter = Regex::new(" +").unwrap();

    let numbers: HashMap<u8, (usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            splitter
                .split(line.trim())
                .enumerate()
                .flat_map(move |(x, num_s)| num_s.parse::<u8>().ok().map(|num| (num, (x, y))))
        })
        .collect();

    BingoCard {
        numbers,
        rows: [0; 5],
        columns: [0; 5],
    }
}

#[cfg(test)]
mod tests {
    use crate::day_4::{parse_card, parse_input, play_bingo, play_bingo_until_last, BingoCard};
    use std::collections::HashMap;

    fn test_card() -> BingoCard {
        #[rustfmt::skip] // keep map literal in grid format
            let expected_numbers: HashMap<u8, (usize, usize)> =
            HashMap::from([
                (22, (0, 0)), (13, (1, 0)), (17, (2, 0)), (11, (3, 0)),  (0, (4, 0)),
                (8, (0, 1)),  (2, (1, 1)), (23, (2, 1)),  (4, (3, 1)), (24, (4, 1)),
                (21, (0, 2)),  (9, (1, 2)), (14, (2, 2)), (16, (3, 2)),  (7, (4, 2)),
                (6, (0, 3)), (10, (1, 3)),  (3, (2, 3)), (18, (3, 3)),  (5, (4, 3)),
                (1, (0, 4)), (12, (1, 4)), (20, (2, 4)), (15, (3, 4)), (19, (4, 4)),
            ]);

        let expected_card = BingoCard {
            numbers: expected_numbers,
            rows: [0; 5],
            columns: [0; 5],
        };
        expected_card
    }

    #[test]
    fn can_parse_card() {
        let expected_card = test_card();

        let parsed_card = parse_card(
            "22 13 17 11  0\n\
                    8  2 23  4 24\n\
                   21  9 14 16  7\n\
                    6 10  3 18  5\n\
                    1 12 20 15 19",
        );

        assert_eq!(parsed_card, expected_card)
    }

    #[test]
    fn can_parse() {
        let (numbers, cards) = parse_input(test_input());

        assert_eq!(
            numbers,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );

        assert_eq!(cards.get(0), Some(&test_card()));

        assert_eq!(
            cards.get(1),
            Some(&parse_card(
                " 3 15  0  2 22\n\
                  9 18 13 17  5\n\
                 19  8  7 25 23\n\
                 20 11 10 24  4\n\
                 14 21 16 12  6",
            ))
        );

        assert_eq!(
            cards.get(2),
            Some(&parse_card(
                "14 21 17 24  4\n\
                 10 16 15  9 19\n\
                 18  8 23 26 20\n\
                 22 11 13  6  5\n\
                  2  0 12  3  7",
            ))
        );
    }

    fn test_input() -> String {
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
            .to_string()
    }

    #[test]
    fn can_mark_card() {
        let mut card = test_card();
        let result = card.mark_number(22);
        assert_eq!(result, false);
        assert_eq!(card.rows, [1, 0, 0, 0, 0]);
        assert_eq!(card.columns, [1, 0, 0, 0, 0]);
        assert_eq!(card.numbers.get(&22), None);

        card.mark_number(13);
        card.mark_number(17);
        card.mark_number(11);
        let result = card.mark_number(0);

        assert_eq!(result, true);
        assert_eq!(card.rows, [5, 0, 0, 0, 0]);
        assert_eq!(card.columns, [1, 1, 1, 1, 1]);

        card.mark_number(99);
        assert_eq!(result, true);
        assert_eq!(card.rows, [5, 0, 0, 0, 0]);
        assert_eq!(card.columns, [1, 1, 1, 1, 1]);
    }

    #[test]
    fn can_play_bingo() {
        let (numbers, cards) = parse_input(test_input());
        let (winning_card, number) = play_bingo(&numbers, &cards);

        assert_eq!(number, 24);
        assert_eq!(winning_card.sum_remaining(), 188)
    }

    #[test]
    fn can_play_bingo_until_exhausted() {
        let (numbers, cards) = parse_input(test_input());
        let (losing_card, number) = play_bingo_until_last(&numbers, &cards);

        assert_eq!(number, 13);
        assert_eq!(losing_card.sum_remaining(), 148)
    }
}
