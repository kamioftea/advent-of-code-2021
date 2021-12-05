//! This is my solution for [Advent of Code - Day 4 - _Giant Squid_](https://adventofcode.com/2021/day/4)
//!
//! Today we had to build an engine to track games of Bingo. Given a set of numbers that would be
//! called, and a set of 100 bingo cards, the challenge was to work out the one that would win first
//! (part one), and the one that would win last (part two). I'm quite pleased with the data
//! structure I came up with to store the bingo cards, but there was a lot of fighting with the
//! borrow checker so I've ended up with some ugly bits of code.
//!
//! The key to the solution is [`BingoCard`] and [`parse_card`] that turns the raw input into this
//! internal representation. The game is then simulated by repeatedly calling
//! [`BingoCard::mark_number`] until the criteria for the current part have been met.
//! [`play_bingo`] implements part one and just runs until a card wins, [`play_bingo_until_last`]
//! implements part two and removes cards from the set as they win until none are left. There is
//! a final small helper [`BingoCard::sum_remaining`] that calculates the number needed for the
//! final submission.

use regex::Regex;
use std::collections::HashMap;
use std::fs;

/// This represents the key information to know if a 5 x 5 bingo card has won.
#[derive(Eq, PartialEq, Debug, Clone)]
struct BingoCard {
    /// A Map indexing the remaining numbers to their co-ordinates on the grid
    numbers: HashMap<u8, (usize, usize)>,
    /// A counter for each row, tracking how many numbers in that row have been removed
    rows: [u8; 5],
    /// A counter for each column, tracking how many numbers in that column have been removed
    columns: [u8; 5],
}

impl BingoCard {
    /// If the card contains the provided number, remove it from the unmarked numbers, increment
    /// the count of marked numbers in the relevant row and column, then if either of these are
    /// now 5, the card has won - return true, otherwise return false.
    ///
    /// If the number is not on the card, nothing changes, and return false.
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

    /// The remaining numbers are the keys of the numbers hash map, as marked numbers are removed
    /// from the map.
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

/// Iterate through the numbers, marking each card as appropriate. Return the first card to win and
/// the number that triggered it, as both are needed to calculate the puzzle solution.
fn play_bingo(numbers: &Vec<u8>, cards: &Vec<BingoCard>) -> (BingoCard, u8) {
    // Create a mutable copy. The cards need to be mutable as marking a number on a card mutates it.
    let mut my_cards = cards.to_vec();
    // Cache the size of the card list
    let size = my_cards.len();
    for &number in numbers {
        // The borrow checker can't guarantee safety when iterating mutable values, so we need to
        // iterate over the indexes...
        for i in 0..size {
            // and do the mutable borrow within the loop.
            let card = my_cards.get_mut(i).unwrap();
            if card.mark_number(number) {
                // mark number returns true if the card won
                return (card.clone(), number);
            }
        }
    }

    // This is unreachable for the puzzle input
    panic!("No winner after numbers exhausted")
}

/// Iterate through the numbers, marking each card as appropriate. Very similar to [`play_bingo`]
/// except it needs to keep going until all cards have won. This leads to some complexity to
/// manage removing the cards from the iterator as we're looping over the same list.
fn play_bingo_until_last(numbers: &Vec<u8>, cards: &Vec<BingoCard>) -> (BingoCard, u8) {
    // Create a mutable copy
    let mut my_cards = cards.to_vec();
    // Track the current length of the active cards
    let mut size = my_cards.len();
    for &number in numbers {
        // The card index we get out of the inner for loop gets out of sync as cards are removed.
        // Track these removals so that we can compensate when indexing into the Vec.
        let mut removal_offset = 0;
        for i in 0..size {
            let actual_index = i - removal_offset;
            let card = my_cards.get_mut(actual_index).unwrap();
            // If the card wins it needs to be removed from the active set
            if card.mark_number(number) {
                // if it is the last one, were done - return the data needed for the puzzle result.
                if size == 1 {
                    return (card.clone(), number);
                }

                // otherwise remove the card from the active list, and keep the numbers used to
                // iterate over them in sync.
                my_cards.remove(actual_index);
                removal_offset = removal_offset + 1;
                size = size - 1;
            }
        }
    }

    // This is unreachable for the puzzle input
    panic!("No winner after numbers exhausted")
}

/// Parse the puzzle input into the internal representation. first there is a line of numbers in
/// the sequence the will be called to mark on the cards, then 100 5 x 5 grids of numbers
/// representing each card. The first line and each card are separated by blank lines.
fn parse_input(contents: String) -> (Vec<u8>, Vec<BingoCard>) {
    // Split on the double new lines that separate each section.
    let mut sections = contents.split("\n\n");
    // The first section is comma separated numbers
    let numbers: Vec<u8> = sections
        .next()
        .expect("Input file was empty")
        .split(",")
        .map(|num| {
            num.parse::<u8>()
                .expect(format!("Invalid number: '{}'", num).as_str())
        })
        .collect();

    // Each remaining section is a bing card
    let cards: Vec<BingoCard> = sections.map(|input| parse_card(input)).collect();

    (numbers, cards)
}

/// This takes a string with 5 lines, each with 5 space-separated numbers, representing a 5 x 5
/// bingo card. A regex is used to split the numbers on a line as single digit numbers cause
/// there to be two spaces prefixing those numbers. [`Iterator::enumerate`] is used to track the
/// current co-ordinates for building the map of unmarked numbers. The row and column arrays are
/// initialised to 0s as no numbers have yet been marked.
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

        // missing number ignored
        card.mark_number(99);
        // duplicate number ignored
        card.mark_number(22);
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
        // The real result set has multiple cards that win with some numbers, so include duplicates
        // in the test to ensure this is covered.
        let cards_with_duplicates = cards.iter().flat_map(|c| [c.clone(), c.clone()]).collect();
        let (losing_card, number) = play_bingo_until_last(&numbers, &cards_with_duplicates);

        assert_eq!(number, 13);
        assert_eq!(losing_card.sum_remaining(), 148)
    }
}
