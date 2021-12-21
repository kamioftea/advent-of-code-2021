//! This is my solution for [Advent of Code - Day 21 - _Dirac Dice_](https://adventofcode.com/2021/day/21)
//!
//! Today was the first time I had to pretty much write entirely new code for part two, but looking
//! at what I ended up with I can't see much overlap that could be reused.
//!
//! For part one I just modelled the [`Game`] and [`Player`]s, with [`Game::from`] that parses the
//! input, and [`Game::play`] that runs the game until someone wins, returning the values needed for
//! the puzzle solution.
//!
//! For part two, I ended up with a rehash of the optimisations used for [`crate::day_6`] and
//! [`crate::day_14`], where I track the counts of each game state, rather than calculating them
//! individually. This is implemented in [`play_quantum`].

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

/// A player in the dice game, tracks their current score and the position of their pawn
#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Player {
    /// Position of the player's pawn
    position: usize,
    /// Players current total score
    score: usize,
}

impl From<&str> for Player {
    /// Players are listed in the input as "Player x starting position: p", and all of it can be
    /// ignored except the last number as they're listed in order.
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

/// Represents a deterministic game of Dirac dice, tracking the current value of the deterministic
/// d100, the players, whose turn it is, and how many rolls have occurred.
#[derive(Eq, PartialEq, Debug)]
struct Game {
    /// List of the players of the game
    players: Vec<Player>,
    /// The index of the player that will take the next turn
    current_player: usize,
    /// The next face that the deterministic d100 will roll
    next_die_face: usize,
    /// how many times the die has been rolled so far
    rolls: usize,
}

impl From<&String> for Game {
    /// Pass the lines of the input to [`Player::from`] to turn it into the player list and set the
    /// counters to their initial values.
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
    /// Get the next `number` rolls from the game's deterministic die, and update the next facing
    /// and roll count.
    fn roll(&mut self, number: usize) -> Vec<usize> {
        let start = self.next_die_face;
        self.next_die_face += number;
        self.rolls += number as usize;

        if self.next_die_face <= 100 {
            // if we're not overflowing return a simple range
            (start..self.next_die_face).collect()
        } else {
            // otherwise handle the overflow
            self.next_die_face -= 100;
            // and concatenate the two ranges either side of the overflow
            [(start..100), (1..self.next_die_face)]
                .iter()
                .flat_map(|a| a.to_owned())
                .collect()
        }
    }

    /// Play the game until a player reaches `target_score` returning the score of the loser at that
    /// point, and the number of rolls made.
    fn play(&mut self, target_score: usize) -> (usize, usize) {
        loop {
            // Roll the dice 3 times and sum them
            let spaces: usize = self.roll(3).iter().sum();
            let current_player = self.current_player;
            let player = self.players.get_mut(current_player).unwrap();
            // Move the pawn a number of spaces determined by the roll
            player.position = (player.position + spaces) % 10;
            // Positions are 1..10 so the 0 space needs special handling
            if player.position == 0 {
                player.score += 10
            } else {
                player.score += player.position as usize
            }
            // Check if the player wins
            if player.score >= target_score {
                // If so return the results needed
                return (
                    self.players
                        .get((current_player + 1) % self.players.len())
                        .unwrap()
                        .score,
                    self.rolls,
                );
            }

            // Otherwise, next player's turn
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
    // Grab the players for part two before they get updated by playing part one
    let players = game.players.clone();

    let (score, rolls) = game.play(1000);
    println!(
        "The loser scored {} after {} deterministic rolls = {}",
        score,
        rolls,
        score * rolls
    );

    let most_wins = play_quantum(players, 21);
    println!("The player with more quantum wins won {} times", most_wins);
}

/// Calculate the permutations of possible games with a quantum d3. Determine which player wins the
/// most times, and return the count of their wins.
fn play_quantum(players: Vec<Player>, target_score: usize) -> usize {
    // Seed the map of game states with the single starting position
    let mut games: HashMap<(Player, Player), usize> =
        HashMap::from([((players[0].clone(), players[1].clone()), 1)]);
    // Pre-calculate the number of rolls that give each possible sum
    let roll_counts: HashMap<usize, usize> = (1..=3)
        .cartesian_product(1..=3)
        .cartesian_product(1..=3)
        .map(|((a, b), c)| a + b + c)
        .counts();

    // initialise the rest of the counters
    let mut wins = [0usize, 0usize];
    let mut current_player_index: usize = 0;

    loop {
        // Create a new map to hold the iterated game state counts
        let mut new_games = HashMap::new();
        // For each current game state and possible dice roll sum
        games.iter().cartesian_product(roll_counts.iter()).for_each(
            |((&(current_player, other_player), &game_count), (&roll, &roll_count))| {
                // The first player in the pair is always going next as we swap them each iteration
                let Player { position, score } = current_player;
                // Work out the new position and score for the current game state/roll pair
                let new_position = (position + roll) % 10;
                let new_score = if new_position == 0 { 10 } else { new_position } + score;
                // the number of games that reach the new game state is the number of games in the
                // current game state multiplied by the number of times the current sum will be
                // rolled.
                let new_game_count = game_count * roll_count;

                if new_score >= target_score {
                    // If the state would win then the current player adds that many games to their
                    // win count
                    wins[current_player_index] += new_game_count
                } else {
                    // Otherwise upsert the count into the new map of game state counts
                    *new_games
                        .entry((
                            // Swap the order so that the player whose turn it is is always first
                            other_player,
                            Player {
                                position: new_position,
                                score: new_score,
                            },
                        ))
                        .or_insert(0) += new_game_count
                }
            },
        );

        // Once all permutations have found a winner the new map will be empty
        if new_games.is_empty() {
            return *wins.iter().max().unwrap();
        }

        // Otherwise update for the next iteration
        games = new_games;
        current_player_index = (current_player_index + 1) % 2;
    }
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
