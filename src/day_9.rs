//! This is my solution for [Advent of Code - Day 9 - _Smoke Basin_](https://adventofcode.com/2021/day/9)
//!
//! _**Note**: At the time of writing [`Grid`] was part of the Day 9 module. Parts that were reused for Day 11 have
//! since been extracted into a utility module._
//!
//! Today's task was to find local minima (part one) of a grid of digits and the area within the watershed of those
//! (part two). For this I built a type to represent a [`Grid`] and implemented a bunch of methods to build towards a
//! solution. I've tried to lean more on the standard library and built in traits to accomplish some of this,
//! specifically using `impl From<String> for Grid` for the initial parsing, and providing [`Grid::iter`] which returns
//! a [`crate::util::grid::GridCoords`] which has an implementation for [`Iterator`] as a standard way to iterate all
//! the cells in the grid.
//!
//! I had to work at getting [`Grid::iter`] correct and my original code was implementing [`IntoIterator`] for Grid,
//! and the iterator was contained the grid itself, which in turn was (because I was struggling with exactly where the
//! lifetime constraints needed to go to satisfy the compiler. This meant I needed to clone the whole grid each time
//! I wanted to iterate over it as the iterator took ownership of the grid it was created with. I eventually found an
//! [article on Iterators and Reference Lifetimes](https://medium.com/@wastedintel/reference-iterators-in-rust-5603a51b5192)
//! that explained how to get the lifetimes to work, and I got a small speed improvement now I wasn't copying the
//! grid all over the place.
//!
//! To outline my solution, [`Grid`] is implemented as a one-dimensional list of numbers, that also has a record of
//! the width of the grid to work out the correct offset in the list for a given x and y, implemented as [`Grid::get`].
//! Working the other way, [`Grid::get_with_coords`] is used by the iterator to work out the 2D co-ordinates of its
//! current position. [`Grid::get_low_points`] filters the iterator of all points in the grid to just the local minima,
//! this defers to [`Grid::is_lowest`] which in turn uses [`Grid::get_orthogonal_surrounds`] to check the current value
//! against its four neighbours. [`Grid::get_risk_level`] takes the result of [`Grid::get_low_points`] and reduces it to
//! the puzzle solution for part one.
//!
//! To solve part two, [`Grid::get_basin`] uses [`Grid::get_orthogonal_surrounds`], filtering to only larger numbers
//! less than the watershed of 9 to recursively build a set of co-ordinates by walking uphill.
//! [`Grid::get_largest_basin_sizes`] is a wrapper that calls [`Grid::get_basin`] for each low point, and the reduces
//! the returned data into the puzzle solution.

use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

#[doc(inline)]
pub use crate::util::grid::Grid;

impl Grid {
    /// Is the provided grid cell a local minimum
    fn is_lowest(&self, y: usize, x: usize) -> bool {
        self.get(y, x)
            .map(|val| {
                self.get_orthogonal_surrounds(y, x)
                    .iter()
                    .all(|&(_, adjacent)| val < adjacent)
            })
            .unwrap_or(false)
    }

    /// Return a list of the co-ordinates and values of all local minima
    fn get_low_points(&self) -> Vec<((usize, usize), u8)> {
        self.iter()
            .filter(|((y, x), _)| self.is_lowest(*y, *x))
            .collect()
    }

    /// The risk level of the grid is the sum of the risk level of each low point, which is the low point's height
    /// plus one.
    fn get_risk_level(&self) -> usize {
        self.get_low_points()
            .iter()
            .map(|&(_, height)| height as usize + 1)
            .sum()
    }

    /// Recursively walk to higher points from a starting minimum, stopping at the watershed of height 9. Returns the
    /// set of co-ordinates found.
    fn get_basin(&self, y: usize, x: usize) -> HashSet<(usize, usize)> {
        let mut basin = HashSet::new();
        if let Some(height) = self.get(y, x) {
            basin.insert((y, x));
            self.get_orthogonal_surrounds(y, x)
                .iter()
                .filter(|(_, h)| *h > height && *h < 9)
                .flat_map(|((y1, x1), _)| self.get_basin(*y1, *x1))
                .for_each(|coord| {
                    basin.insert(coord);
                })
        }

        basin
    }

    /// Iterate through the local minima, find the basin size of each, and return the highest three sizes found
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
        assert_eq!(grid.get(0, 0), Some(2));
        assert_eq!(grid.get(4, 9), Some(8));
        assert_eq!(grid.get(5, 10), None);

        assert_eq!(
            grid.iter().take(3).collect::<Vec<((usize, usize), u8)>>(),
            vec![((0, 0), 2), ((0, 1), 1), ((0, 2), 9)]
        );
        assert_eq!(
            grid.iter().skip(48).collect::<Vec<((usize, usize), u8)>>(),
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

        assert_eq!(
            grid.get_orthogonal_surrounds(0, 0),
            vec![((0, 1), 1), ((1, 0), 3)]
        );
        assert_eq!(
            grid.get_orthogonal_surrounds(0, 1),
            vec![((0, 2), 9), ((1, 1), 9), ((0, 0), 2)]
        );
        assert_eq!(
            grid.get_orthogonal_surrounds(1, 1),
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
        grid.iter().for_each(|((y, x), h)| {
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
