//! This is my solution for [Advent of Code - Day 21 - _Title_](https://adventofcode.com/2021/day/21)
//!
//!

use std::fs;

#[derive(Eq, PartialEq, Debug)]
struct Player {
    position: u8,
    score: usize,
}

impl From<&str> for Player {
    fn from(s: &str) -> Self {
        Player {
            position: s
                .split(" ")
                .last()
                .and_then(|pos| pos.parse().ok())
                .unwrap(),
            score: 0,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Game {
    players: Vec<Player>,
    current_player: usize,
    next_die_face: u8,
    rolls: usize,
}

impl From<&String> for Game {
    fn from(str: &String) -> Self {
        Game {
            players: str.lines().map(Player::from).collect(),
            current_player: 0,
            next_die_face: 1,
            rolls: 0,
        }
    }
}

impl Game {
    fn _play(&mut self) -> (usize, usize) {
        loop {
            let _player = self.players.get_mut(self.current_player).unwrap();
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-21-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 21.
pub fn run() {
    let _contents = fs::read_to_string("res/day-21-input").expect("Failed to read file");
}

#[cfg(test)]
mod tests {
    use crate::day_21::{Game, Player};

    #[test]
    fn can_parse() {
        let input = "Player 1 starting position: 4
Player 2 starting position: 8"
            .to_string();

        let expected = Game {
            players: Vec::from([
                Player {
                    position: 4,
                    score: 0,
                },
                Player {
                    position: 8,
                    score: 0,
                },
            ]),
            current_player: 0,
            next_die_face: 1,
            rolls: 0,
        };

        assert_eq!(Game::from(&input), expected);
    }
}
