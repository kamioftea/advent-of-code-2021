//! This is my solution for [Advent of Code - Day 21 - _Title_](https://adventofcode.com/2021/day/21)
//!
//!

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Player {
    position: usize,
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
    next_die_face: usize,
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
    fn roll(&mut self, number: usize) -> Vec<usize> {
        let start = self.next_die_face;
        self.next_die_face += number;
        self.rolls += number as usize;

        if self.next_die_face <= 100 {
            (start..self.next_die_face).collect()
        } else {
            self.next_die_face -= 100;
            [(start..100), (1..self.next_die_face)]
                .iter()
                .flat_map(|a| a.to_owned())
                .collect()
        }
    }

    fn play(&mut self, target_score: usize) -> (usize, usize) {
        loop {
            let spaces: usize = self.roll(3).iter().sum();
            let current_player = self.current_player;
            let player = self.players.get_mut(current_player).unwrap();
            player.position = (player.position + spaces) % 10;
            if player.position == 0 {
                player.score += 10
            } else {
                player.score += player.position as usize
            }

            if player.score >= target_score {
                return (
                    self.players
                        .get((current_player + 1) % self.players.len())
                        .unwrap()
                        .score,
                    self.rolls,
                );
            }

            self.current_player = (current_player + 1) % self.players.len();
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-21-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 21.
pub fn run() {
    let contents = fs::read_to_string("res/day-21-input").expect("Failed to read file");

    let mut game = Game::from(&contents);
    let (score, rolls) = game.play(1000);
    println!(
        "The loser scored {} after {} deterministic rolls = {}",
        score,
        rolls,
        score * rolls
    );

    let players = Game::from(&contents).players;
    let most_wins = play_quantum(players, 21);
    println!("The player with more quantum wins won {} times", most_wins);
}

fn play_quantum(players: Vec<Player>, target_score: usize) -> usize {
    let mut games: HashMap<(Player, Player), usize> =
        HashMap::from([((players[0].clone(), players[1].clone()), 1)]);
    let roll_counts: HashMap<usize, usize> = (1..=3)
        .cartesian_product(1..=3)
        .cartesian_product(1..=3)
        .map(|((a, b), c)| a + b + c)
        .counts();

    let mut wins = [0usize, 0usize];
    let mut current_player_index: usize = 0;

    loop {
        let mut new_games = HashMap::new();
        games.iter().cartesian_product(roll_counts.iter()).for_each(
            |((&(current_player, other_player), &game_count), (&roll, &roll_count))| {
                let Player { position, score } = current_player;
                let new_position = (position + roll) % 10;
                let new_score = if new_position == 0 { 10 } else { new_position } + score;
                if new_score >= target_score {
                    wins[current_player_index] += game_count * roll_count
                } else {
                    *new_games
                        .entry((
                            other_player,
                            Player {
                                position: new_position,
                                score: new_score,
                            },
                        ))
                        .or_insert(0) += game_count * roll_count
                }
            },
        );

        if new_games.is_empty() {
            break;
        }

        games = new_games;
        current_player_index = (current_player_index + 1) % 2;
    }

    *wins.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_21::{play_quantum, Game, Player};

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

    #[test]
    fn can_play() {
        let mut game = Game {
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

        assert_eq!(game.play(1000), (745, 993))
    }

    #[test]
    fn can_play_quantum() {
        let players = Vec::from([
            Player {
                position: 4,
                score: 0,
            },
            Player {
                position: 8,
                score: 0,
            },
        ]);

        assert_eq!(play_quantum(players, 21), 444356092776315)
    }
}
