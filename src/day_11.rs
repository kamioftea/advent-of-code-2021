//! This is my solution for [Advent of Code - Day 11 - _Title_](https://adventofcode.com/2021/day/11)
//!
//!

use std::collections::HashSet;
use std::fs;

use crate::day_9::Grid;

impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            numbers: self.numbers.to_vec(),
            width: self.width,
        }
    }
}

impl Grid {
    fn set(&mut self, y: usize, x: usize, val: u8) -> bool {
        if x >= self.width {
            return false;
        }

        let pos = y * self.width + x;
        if pos < self.numbers.len() {
            self.numbers[pos] = val;
            return true;
        }
        return false;
    }

    /// Iterate through the four orthogonal cells, collecting the 2 - 4 values into a vector. Include the co-ordinates
    /// in the returned vector so that [`Grid::get_basin`] can recursively expand the set of cells in the basin.
    pub fn get_all_surrounds(&self, y: usize, x: usize) -> Vec<((usize, usize), u8)> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] // NW W SW N S NE E NW
        .iter()
        .flat_map(|&(dy, dx)| self.get_relative(y, x, dy, dx))
        .collect()
    }

    fn iterate_and_flash(&mut self) -> usize {
        let mut flashes: HashSet<(usize, usize)> = HashSet::new();
        let mut to_flash: Vec<(usize, usize)> = Vec::new();

        for i in 0..self.numbers.len() {
            if let Some(((y, x), val)) = self.get_with_coords(i) {
                self.set(y, x, val + 1);

                if val == 9 {
                    to_flash.push((y, x));
                }
            }
        }

        while let Some((y, x)) = to_flash.pop() {
            if !flashes.insert((y, x)) {
                // already flashed
                continue;
            }

            for ((y1, x1), val) in self.get_all_surrounds(y, x) {
                self.set(y1, x1, val + 1);
                if val == 9 {
                    to_flash.push((y1, x1))
                }
            }
        }

        for &(y, x) in &flashes {
            self.set(y, x, 0);
        }

        flashes.len()
    }

    fn _print(&self) {
        let mut line = 0;

        for ((y, _), v) in self.iter() {
            if y != line {
                line = y;
                print!("\n")
            }
            if v > 9 {
                print!("#")
            } else {
                print!("{}", v)
            }
        }
        print!("\n")
    }

    fn count_flashes(&mut self, cycles: usize) -> usize {
        let mut total: usize = 0;

        for _ in 0..cycles {
            total = total + self.iterate_and_flash()
        }

        total
    }

    fn run_until_sync(&mut self) -> usize {
        let target = self.numbers.len();
        let mut iteration: usize = 0;

        loop {
            iteration = iteration + 1;
            if self.iterate_and_flash() == target {
                return iteration;
            }
        }
    }
}
/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-11-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 11.
pub fn run() {
    let contents = fs::read_to_string("res/day-11-input").expect("Failed to read file");
    let grid = Grid::from(contents);

    let flashes = grid.clone().count_flashes(100);
    println!("There were {} flashes in 100 cycles", flashes);

    let iterations = grid.clone().run_until_sync();
    println!(
        "It took {} cycles for the flashes to synchronise.",
        iterations
    );
}

#[cfg(test)]
mod tests {
    use crate::day_9::Grid;
    use std::collections::HashSet;

    #[test]
    fn can_update_grid() {
        let mut grid = Grid::from(
            "11111
19991
19191
19991
11111"
                .to_string(),
        );
        let expected = Grid::from(
            "21111
19991
19291
19991
11111"
                .to_string(),
        );

        grid.set(0, 0, 2);
        grid.set(2, 2, 2);

        assert_eq!(grid, expected);
    }

    #[test]
    fn can_get_all_surrounds() {
        let grid = Grid::from("123\n456\n789".to_string());
        let surrounds: HashSet<u8> = grid
            .get_all_surrounds(1, 1)
            .iter()
            .map(|&(_, v)| v)
            .collect();
        assert_eq!(surrounds.len(), 8);
        assert!(!surrounds.contains(&5))
    }

    #[test]
    fn can_iterate_and_flash() {
        let mut grid = Grid::from(
            "11111
19991
19191
19991
11111"
                .to_string(),
        );

        let expected = Grid::from(
            "34543
40004
50005
40004
34543"
                .to_string(),
        );

        let flashes = grid.iterate_and_flash();

        assert_eq!(flashes, 9);
        assert_eq!(grid, expected);
    }

    #[test]
    fn can_count_flashes() {
        let grid = Grid::from(
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
                .to_string(),
        );

        assert_eq!(grid.clone().count_flashes(10), 204);
        assert_eq!(grid.clone().count_flashes(100), 1656);
    }

    #[test]
    fn can_run_until_sync() {
        let mut grid = Grid::from(
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
                .to_string(),
        );

        assert_eq!(grid.run_until_sync(), 195);
    }
}
