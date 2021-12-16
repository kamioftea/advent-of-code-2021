//! This is my solution for [Advent of Code - Day 15 - _Chiton_](https://adventofcode.com/2021/day/15)
//!
//! I picked up pretty quickly that this needed a shortest-path graph traversal algorithm, and (very) vaguely
//! remembered Dijkstra's from when it was covered in my A-Level math course. I did some googling to refresh my memory,
//! noted the advice to use a [`BinaryHeap`], found that the Rust std implementation had Dijkstra's
//! as it's main example in the docs. I imported [`Grid`] from previous days, and updated the
//! example code to work with co-ordinates, and it just worked: [`find_shortest_path`].
//!
//! For part two I didn't want to store the much bigger and repeated graph in memory, so I wrote a wrapper
//! [`ExpandedGrid`] that would provide implementations for all the methods used by [`find_shortest_path`] and work out
//! how to translate that to methods on the underlying sub-grid, get methods being [`ExpandedGrid::pos_of`] and
//! [`ExpandedGrid::get`]. The wrapper ended up a little messy, but it'll do for AoC. If I was planning to need to
//! maintain this code, I'd maybe look into extracting some parts to a trait so that I'm not repeating code from
//! [`Grid`].

use crate::util::grid::Grid;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

/// This is juts copied from  the example [`std::collections::BinaryHeap`] with position swapped for coords.
#[derive(Copy, Clone, Eq, PartialEq)]
struct Cell {
    cost: usize,
    coords: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.coords.cmp(&other.coords))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A wrapper around [`Grid`] that handles tiling a smaller sub-grid.
struct ExpandedGrid<'a> {
    /// The wrapped sub-grid
    grid: &'a Grid,
    /// Cache the calculated height
    sub_grid_height: usize,
    /// Number of times the grid is tiled in the y axis
    copies_y: usize,
    /// Number of times the grid is tiled in the x axis
    copies_x: usize,
}

impl<'a> From<&'a Grid> for ExpandedGrid<'a> {
    /// Build an untiled wrapper from a given sub-grid. See also [`ExpandedGrid::with_copies`]
    fn from(grid: &'a Grid) -> Self {
        let (_, max_y) = grid.max_coords();

        return ExpandedGrid {
            grid,
            copies_y: 1,
            copies_x: 1,
            sub_grid_height: max_y + 1,
        };
    }
}

impl<'a> ExpandedGrid<'a> {
    /// Return a new wrapper with different tiling
    fn with_copies(&self, copies_y: usize, copies_x: usize) -> ExpandedGrid<'a> {
        ExpandedGrid {
            grid: self.grid,
            sub_grid_height: self.sub_grid_height,
            copies_y,
            copies_x,
        }
    }

    /// The number of cells in the grid
    fn len(&self) -> usize {
        self.grid.len() * self.copies_y * self.copies_x
    }

    /// The total width of the grid
    fn width(&self) -> usize {
        self.grid.width * self.copies_x
    }

    /// The co-ordinates of the bottom right corner of the grid in (y, x) format
    fn max_coords(&self) -> (usize, usize) {
        ((self.len() - 1) / self.width(), self.width() - 1)
    }

    /// Find the meta-co-ordinates of the tile a cell co-ordinate is on
    fn tile_coords(&self, y: usize, x: usize) -> (usize, usize) {
        (y / self.sub_grid_height, x / self.grid.width)
    }

    /// Translate a grid-levey co-ordinates to the sub-grid co-ordinates, i.e. the co-ordinates within the current tile.
    fn sub_grid_coords(&self, y: usize, x: usize) -> (usize, usize) {
        (y % self.sub_grid_height, x % self.grid.width)
    }

    /// Turn co-ordinates into the offset in the underlying list of cell values.
    fn pos_of(&self, y: usize, x: usize) -> Option<usize> {
        let (tile_y, tile_x) = self.tile_coords(y, x);
        if tile_y >= self.copies_y || tile_x >= self.copies_x {
            return None;
        }

        let tile_pos = tile_y * self.copies_x + tile_x;
        let (sub_grid_y, sub_grid_x) = self.sub_grid_coords(y, x);

        self.grid
            .pos_of(sub_grid_y, sub_grid_x)
            .map(|sub_grid_pos| tile_pos * self.grid.len() + sub_grid_pos)
    }

    /// Given grid co-ordinates, get the value from the referenced cell in the sub-grid, and apply the cost modifier
    /// based on the tile position.
    fn get(&self, y: usize, x: usize) -> Option<u8> {
        let (tile_y, tile_x) = self.tile_coords(y, x);
        let (sub_grid_y, sub_grid_x) = self.sub_grid_coords(y, x);

        if tile_y >= self.copies_y || tile_x >= self.copies_x {
            return None;
        }

        self.grid
            .get(sub_grid_y, sub_grid_x)
            // + offset based on tile Manhattan distance - note this overflows to 1 not 0
            .map(|v| (((v as usize - 1) + tile_y + tile_x) % 9) as u8 + 1)
    }

    //noinspection DuplicatedCode
    /// Copied from grid, but needs to use the [`ExpandedGrid::get_relative`] to manage crossing tile boundaries
    fn get_orthogonal_surrounds(&self, y: usize, x: usize) -> Vec<((usize, usize), u8)> {
        [(-1, 0), (0, 1), (1, 0), (0, -1)] // N E S W
            .iter()
            .flat_map(|&(dy, dx)| self.get_relative(y, x, dy, dx))
            .collect()
    }

    //noinspection DuplicatedCode
    /// Copied from grid, but needs to use the [`ExpandedGrid::get`] to manage crossing tile boundaries
    fn get_relative(
        &self,
        y: usize,
        x: usize,
        dy: isize,
        dx: isize,
    ) -> Option<((usize, usize), u8)> {
        let y1 = (y as isize) + dy;
        let x1 = (x as isize) + dx;

        if y1 >= 0 && x1 >= 0 {
            self.get(y1 as usize, x1 as usize)
                .map(|val| ((y1 as usize, x1 as usize), val))
        } else {
            None
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-15-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 15.
pub fn run() {
    let contents = fs::read_to_string("res/day-15-input").expect("Failed to read file");
    let sub_grid = Grid::from(contents);

    let grid = ExpandedGrid::from(&sub_grid);
    let cost = find_shortest_path(&grid, (0, 0), grid.max_coords());
    println!("The cost to traverse the grid is: {:?}", cost);

    let grid2 = grid.with_copies(5, 5);
    let cost2 = find_shortest_path(&grid2, (0, 0), grid2.max_coords());
    println!("The cost to traverse the grid tiles is: {:?}", cost2);
}

/// Implement Dijkstra's shortest path algorithm. Copied from [`BinaryHeap`] example and modified to get the edge
/// costs from the provided grid. Originally accepted  [`Grid`] but it was easier to use one type/method for both parts
/// and the [`ExpandedGrid`] works the same as a [`Grid`] if it only has one tile on each axis.
fn find_shortest_path(
    grid: &ExpandedGrid,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<usize> {
    let mut heap: BinaryHeap<Cell> = BinaryHeap::new();
    let mut dist: Vec<usize> = (0..grid.len()).map(|_| usize::MAX).collect();

    dist[grid.pos_of(start.0, start.1).unwrap()] = 0;
    heap.push(Cell {
        cost: 0,
        coords: start,
    });

    while let Some(Cell { cost, coords }) = heap.pop() {
        if coords == goal {
            return Some(cost);
        }

        if cost > dist[grid.pos_of(coords.0, coords.1).unwrap()] {
            continue;
        }

        for (next_coords, v) in grid.get_orthogonal_surrounds(coords.0, coords.1) {
            let next_cost = cost + v as usize;
            let next_pos = grid.pos_of(next_coords.0, next_coords.1).unwrap();
            if next_cost < dist[next_pos] {
                heap.push(Cell {
                    cost: next_cost,
                    coords: next_coords,
                });
                dist[next_pos] = next_cost
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::day_11::Grid;
    use crate::day_15::{find_shortest_path, ExpandedGrid};

    #[test]
    fn can_find_path() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            .to_string();

        let sub_grid = Grid::from(input);
        let grid = ExpandedGrid::from(&sub_grid);
        assert_eq!(
            find_shortest_path(&grid, (0, 0), grid.max_coords()),
            Some(40)
        );

        let grid2 = grid.with_copies(5, 5);

        assert_eq!(
            find_shortest_path(&grid2, (0, 0), grid2.max_coords()),
            Some(315)
        );
    }
}
