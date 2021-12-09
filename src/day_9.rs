//! This is my solution for [Advent of Code - Day 9 - _Title_](https://adventofcode.com/2021/day/9)
//!
//!

use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Grid {
    numbers: Vec<u8>,
    width: usize,
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            numbers: self.numbers.to_vec(),
            width: self.width,
        }
    }
}

impl From<String> for Grid {
    fn from(string: String) -> Self {
        let mut width: usize = 0;

        let numbers = string
            .lines()
            .flat_map(|line| {
                width = line.len();
                return line.chars().map(|c| {
                    c.to_digit(10)
                        .expect(format!("{} is not a digit", c).as_str()) as u8
                });
            })
            .collect();

        Grid { numbers, width }
    }
}

struct GridCoords {
    grid: Grid,
    pos: usize,
}

impl Iterator for GridCoords {
    type Item = ((usize, usize), u8);

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.grid.get_with_coords(self.pos);
        self.pos = self.pos + 1;

        curr
    }
}

impl IntoIterator for Grid {
    type Item = ((usize, usize), u8);
    type IntoIter = GridCoords;

    fn into_iter(self) -> Self::IntoIter {
        return GridCoords {
            grid: self.clone(),
            pos: 0,
        };
    }
}

impl Grid {
    fn get_item(&self, y: usize, x: usize) -> Option<u8> {
        if x >= self.width {
            return None;
        }
        self.numbers.get(x + y * self.width).map(|&t| t)
    }

    fn get_with_coords(&self, pos: usize) -> Option<((usize, usize), u8)> {
        let x = pos % self.width;
        let y = pos / self.width;

        self.numbers.get(pos).map(|&val| ((y, x), val))
    }

    fn get_surrounds(&self, y: usize, x: usize) -> Vec<((usize, usize), u8)> {
        [(-1, 0), (0, 1), (1, 0), (0, -1)] // N E S W
            .iter()
            .flat_map(|(dy, dx)| {
                let y1 = (y as isize) + dy;
                let x1 = (x as isize) + dx;

                if y1 >= 0 && x1 >= 0 {
                    self.get_item(y1 as usize, x1 as usize)
                        .map(|val| ((y1 as usize, x1 as usize), val))
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_lowest(&self, y: usize, x: usize) -> bool {
        self.get_item(y, x)
            .map(|val| {
                self.get_surrounds(y, x)
                    .iter()
                    .all(|&(_, adjacent)| val < adjacent)
            })
            .unwrap_or(false)
    }

    fn get_low_points(&self) -> Vec<((usize, usize), u8)> {
        self.clone()
            .into_iter()
            .filter(|((y, x), _)| self.is_lowest(*y, *x))
            .collect()
    }

    fn get_risk_level(&self) -> usize {
        self.get_low_points()
            .iter()
            .map(|&(_, height)| height as usize + 1)
            .sum()
    }

    fn get_basin(&self, y: usize, x: usize) -> HashSet<(usize, usize)> {
        let mut basin = HashSet::new();
        if let Some(height) = self.get_item(y, x) {
            basin.insert((y, x));
            self.get_surrounds(y, x)
                .iter()
                .filter(|(_, h)| *h > height && *h < 9)
                .flat_map(|((y1, x1), _)| self.get_basin(*y1, *x1))
                .for_each(|coord| {
                    basin.insert(coord);
                })
        }

        basin
    }

    fn get_largest_basin_sizes(&self) -> Vec<usize> {
        self.get_low_points()
            .iter()
            .map(|((y, x), _)| self.get_basin(*y, *x).len())
            .sorted()
            .rev()
            .take(3)
            .collect()
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-9-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 9.
pub fn run() {
    let contents = fs::read_to_string("res/day-9-input").expect("Failed to read file");
    let grid = Grid::from(contents);

    println!("Total risk level: {}", grid.get_risk_level());

    let basin_sizes = grid.get_largest_basin_sizes();
    println!(
        "Largest Basins: {} * {} * {} = {}, ",
        basin_sizes.get(0).unwrap(),
        basin_sizes.get(1).unwrap(),
        basin_sizes.get(2).unwrap(),
        basin_sizes.iter().product::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day_9::Grid;

    #[test]
    fn can_parse() {
        let grid = get_sample_grid();

        assert_eq!(grid.width, 10);
        assert_eq!(grid.get_item(0, 0), Some(2));
        assert_eq!(grid.get_item(4, 9), Some(8));
        assert_eq!(grid.get_item(5, 10), None);

        assert_eq!(
            grid.clone()
                .into_iter()
                .take(3)
                .collect::<Vec<((usize, usize), u8)>>(),
            vec![((0, 0), 2), ((0, 1), 1), ((0, 2), 9)]
        );
        assert_eq!(
            grid.clone()
                .into_iter()
                .skip(48)
                .collect::<Vec<((usize, usize), u8)>>(),
            vec![((4, 8), 7), ((4, 9), 8)]
        );
    }

    fn get_sample_grid() -> Grid {
        let input = "2199943210\n\
             3987894921\n\
             9856789892\n\
             8767896789\n\
             9899965678"
            .to_string();

        let grid = Grid::from(input);
        grid
    }

    #[test]
    fn can_get_surrounds() {
        let grid = get_sample_grid();

        assert_eq!(grid.get_surrounds(0, 0), vec![((0, 1), 1), ((1, 0), 3)]);
        assert_eq!(
            grid.get_surrounds(0, 1),
            vec![((0, 2), 9), ((1, 1), 9), ((0, 0), 2)]
        );
        assert_eq!(
            grid.get_surrounds(1, 1),
            vec![((0, 1), 1), ((1, 2), 8), ((2, 1), 8), ((1, 0), 3)]
        );
    }

    #[test]
    fn can_determine_if_lowest() {
        let grid = get_sample_grid();

        assert_eq!(grid.is_lowest(0, 0), false);
        assert_eq!(grid.is_lowest(0, 1), true);
    }

    #[test]
    fn can_get_low_points() {
        let grid = get_sample_grid();

        assert_eq!(
            grid.get_low_points(),
            vec![((0, 1), 1), ((0, 9), 0), ((2, 2), 5), ((4, 6), 5)]
        )
    }

    #[test]
    fn can_get_risk_level() {
        let grid = get_sample_grid();

        assert_eq!(grid.get_risk_level(), 15)
    }

    #[test]
    fn can_get_basin() {
        let grid = get_sample_grid();

        assert_eq!(
            grid.get_basin(0, 1),
            HashSet::from([(0, 0), (0, 1), (1, 0)])
        );
    }

    #[test]
    fn can_get_basin_sizes() {
        let grid = get_sample_grid();

        assert_eq!(grid.get_largest_basin_sizes(), vec![14, 9, 9]);
    }

    fn _debug_basin(grid: Grid, basin: HashSet<(usize, usize)>) {
        let mut line = 0;
        grid.clone().into_iter().for_each(|((y, x), h)| {
            if line != y {
                print!("\n");
                line = y;
            }
            print!(
                "{}",
                if basin.contains(&(y, x)) {
                    h.to_string()
                } else {
                    "#".to_string()
                }
            )
        });
        print!("\n");
    }
}
