//! This is my solution for [Advent of Code - Day 25 - _Sea Cucumber_](https://adventofcode.com/2021/day/25)
//!
//! After the last two days, today was a refreshing change back to a more 'normal' Advent of Code puzzle. We were given
//! a grid representing herds of Sea Cucumbers wanting to move in a certain direction and had to iterate those moves
//! until the grid stabilised with all the sea cucumbers blocked by another one. A slight twist was some 'strong
//! currents' that caused the grid to wrap around in both x and y.
//!
//! [`Cell`] represents the three possibilities for any cell in the grid: Empty, Rightwards moving cucumber,
//! downwards moving cucumber. [`Grid`] stores the whole grid similar to [`crate::util::grid`], but different enough
//! that it was easier to just re-implement it. [Grid::from] parses the puzzle input with help from [`Cell::try_from`].
//! [`Grid::fmt`] and [`Cell::fmt`] go the other way for ease of testing. [`Grid::get`], [`Grid::pos_of`],
//! [`Grid::swap`], and [`Grid::can_move`] are all utilities that help with iterating the grid. [`Grid::iterate`]
//! completes a single iteration step of each herd trying to move. This is where the one efficiency trick of the day is
//! apparent. The grid has a caches of the cucumbers that *might* be able to move. Starting with all the cucumbers
//! assigned to a cache for their direction, only 1) the sea cucumbers that have moved, and 2) the sea cucumbers
//! positioned so they could move into the vacated cell, are added to the active set for the next iteration. This
//! achieves two things: It limits the cells we need to check each iteration, and it is a handy indicator of if the
//! grid has stabilised. As only moving cucumbers cause additions to the active sets, the grid is stable if and only
//! if the two caches are empty.
//!
//! That solves part one, and part two was the traditional "finish all the tasks and click the button to resolve the
//! plot" task. I was able to complete each task on the day this year (just - day 24 was finally done at 2am on 25th
//! UTC, so 3 hours before the cutoff), so this was already complete for me.
//!
//! ```text
//!       --------Part 1--------   --------Part 2--------
//! Day       Time   Rank  Score       Time   Rank  Score
//!  25   15:06:43   8974      0   15:07:35   5460      0
//!  24   21:15:47   5841      0   21:16:30   5687      0
//!  23   08:47:15   5672      0   19:27:15   6218      0
//!  22   07:00:04   9762      0   07:01:31   3769      0
//!  21   05:37:25  11193      0   06:36:50   6331      0
//!  20   07:51:38   8898      0   07:58:09   8586      0
//!  19   13:14:31   5465      0   13:37:20   5294      0
//!  18   19:49:26  12383      0   20:00:08  12192      0
//!  17   04:47:21  10448      0   05:52:20  10205      0
//!  16   07:07:32   9928      0   07:16:09   8599      0
//!  15   05:47:19  12724      0   07:05:51  10514      0
//!  14   05:39:34  20863      0   07:04:42  14630      0
//!  13   05:19:33  17344      0   05:40:01  16739      0
//!  12   14:30:26  26961      0   14:55:24  24961      0
//!  11   08:27:10  21478      0   08:32:38  21179      0
//!  10   07:15:30  27171      0   08:06:12  26391      0
//!   9   08:32:37  32148      0   11:48:09  27638      0
//!   8   07:01:34  32052      0   08:06:36  19511      0
//!   7   04:14:51  28137      0   04:40:14  27200      0
//!   6   04:50:17  27578      0   04:53:08  20498      0
//!   5   07:05:00  26127      0   07:56:49  24779      0
//!   4   14:56:36  42423      0   15:45:56  40155      0
//!   3   06:10:06  46408      0   08:06:50  34497      0
//!   2   04:22:10  39520      0   04:29:07  37024      0
//!   1   08:11:39  47103      0   09:01:48  43667      0
//! ```

use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use Cell::{DOWN, EMPTY, RIGHT};
/// Represent the current state of a cell in the grid
#[derive(Eq, PartialEq, Copy, Clone)]
enum Cell {
    EMPTY,
    DOWN,
    RIGHT,
}

impl TryFrom<char> for Cell {
    type Error = ();
    /// Parse the the ascii for a cell as the relevant [`Cell`] value
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(EMPTY),
            '>' => Ok(RIGHT),
            'v' => Ok(DOWN),
            _ => Err(()),
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Cell {
    /// Display cells as their puzzle inout character representation
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EMPTY => ".",
                DOWN => "v",
                RIGHT => ">",
            }
        )
    }
}

/// Represent a grid as a vector of cells, with a width and height to enable quick lookups from x/y co-ordinates, and
/// to help with wrapping around logic. Also keep [`HashSet`]s of the RIGHT and DOWN cells that may be able to move,
/// to limit the cells we need to check when iterating the grid
#[derive(Eq, PartialEq, Debug)]
struct Grid {
    /// The cells of the grid as a single list
    cells: Vec<Cell>,
    /// Cache the grid width
    width: usize,
    /// Cache teh grid height
    height: usize,
    /// The cells with a RIGHTwards moving sea cucumber that may be able to move
    active_right: HashSet<(usize, usize)>,
    /// The cells with a DOWNwards moving sea cucumber that may be able to move
    active_down: HashSet<(usize, usize)>,
}

impl From<&String> for Grid {
    /// Parse the puzzle input as a grid, building the initial active sets to include all the sea cucumbers of the
    /// relevant type
    fn from(s: &String) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut active_right = HashSet::new();
        let mut active_down = HashSet::new();
        let mut cells = Vec::new();
        for (y, line) in s.lines().enumerate() {
            width = width.max(line.len());
            height += 1;
            for (x, chr) in line.chars().enumerate() {
                match Cell::try_from(chr) {
                    Ok(RIGHT) => {
                        active_right.insert((x, y));
                        cells.push(RIGHT)
                    }
                    Ok(DOWN) => {
                        active_down.insert((x, y));
                        cells.push(DOWN)
                    }
                    _ => cells.push(EMPTY),
                };
            }
        }

        Grid {
            cells,
            width,
            height,
            active_right,
            active_down,
        }
    }
}

impl Grid {
    /// Get the current value of a given cell co-ordinate, or None if it is out of bounds for the grid
    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.cells.get(self.pos_of(x, y))
        }
    }

    /// Convert x, y co-ordinates to an index in the underlying list of cells.
    fn pos_of(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Swap the values of two cells - used when sea cucumbers move
    fn swap(&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) {
        let pos1 = self.pos_of(x1, y1);
        let pos2 = self.pos_of(x2, y2);
        self.cells.swap(pos1, pos2)
    }

    /// We are overly optimistic when building the active sets. Given the co-ordinates of a sea cucumber and it's
    /// direction of travel, check if it's next cell is actually available to move into.
    fn can_move(&self, x: usize, y: usize, direction: Cell) -> bool {
        match direction {
            RIGHT => self.get((x + 1) % self.width, y) == Some(&EMPTY),
            DOWN => self.get(x, (y + 1) % self.height) == Some(&EMPTY),
            _ => false,
        }
    }

    /// Do a full iteration of the grid in-place, moving RIGHTs that can move rightwards, then DOWNs that can move
    /// downwards. Calculate the new active set as all the cucumbers that moved, plus any that can move into the
    /// vacated space.
    fn iterate(&mut self) {
        let mut new_active_right = HashSet::new();

        let move_right: Vec<(usize, usize)> = self
            .active_right
            .iter()
            .filter(|(x, y)| self.can_move(*x, *y, RIGHT))
            .map(|&(x, y)| (x, y))
            .collect();

        for (x, y) in move_right.clone() {
            let next_x = (x + 1) % self.width;
            self.swap((x, y), (next_x, y));

            new_active_right.insert((next_x, y));

            let prev_x = if x == 0 { self.width - 1 } else { x - 1 };
            if self.get(prev_x, y) == Some(&RIGHT) {
                new_active_right.insert((prev_x, y));
            }

            let prev_y = if y == 0 { self.height - 1 } else { y - 1 };
            if self.get(x, prev_y) == Some(&DOWN) {
                self.active_down.insert((x, prev_y));
            }
        }

        self.active_right = new_active_right;

        let mut new_active_down = HashSet::new();
        let move_down: Vec<(usize, usize)> = self
            .active_down
            .iter()
            .filter(|(x, y)| self.can_move(*x, *y, DOWN))
            .map(|&(x, y)| (x, y))
            .collect();

        for (x, y) in move_down.clone() {
            let next_y = (y + 1) % self.height;
            self.swap((x, y), (x, next_y));

            new_active_down.insert((x, next_y));

            let prev_x = if x == 0 { self.width - 1 } else { x - 1 };
            if self.get(prev_x, y) == Some(&RIGHT) {
                self.active_right.insert((prev_x, y));
            }

            let prev_y = if y == 0 { self.height - 1 } else { y - 1 };
            if self.get(x, prev_y) == Some(&DOWN) {
                new_active_down.insert((x, prev_y));
            }
        }

        self.active_down = new_active_down;
    }

    fn iterate_until_static(&mut self) -> usize {
        let mut states = 0;
        while self.active_right.len() > 0 || self.active_down.len() > 0 {
            self.iterate();
            states += 1;
        }

        states
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.cells
            .iter()
            .enumerate()
            .fold(Result::Ok(()), |acc, (i, cell)| {
                acc.and_then(|()| {
                    write!(
                        f,
                        "{}{}",
                        cell,
                        if i % self.width == self.width - 1 {
                            "\n"
                        } else {
                            ""
                        }
                    )
                })
            })
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-25-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 25.
pub fn run() {
    let contents = fs::read_to_string("res/day-25-input").expect("Failed to read file");
    let mut grid = Grid::from(&contents);
    let count = grid.iterate_until_static();
    println!("The sea cucumbers stabilise in {} steps", count)
}

#[cfg(test)]
mod tests {
    use crate::day_25::Cell::{DOWN, EMPTY, RIGHT};
    use crate::day_25::Grid;
    use std::collections::HashSet;

    #[test]
    fn can_parse() {
        let grid = Grid::from(&"...>>>>>...".to_string());
        assert_eq!(grid.width, 11);
        assert_eq!(grid.height, 1);
        assert_eq!(
            grid.cells,
            Vec::from([
                EMPTY, EMPTY, EMPTY, RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, EMPTY, EMPTY, EMPTY,
            ])
        );

        assert_eq!(grid.active_right.len(), 5);
        assert_eq!(grid.active_down.len(), 0);

        let grid2 = Grid::from(
            &"..........
.>v....v..
.......>..
.........."
                .to_string(),
        );

        assert_eq!(grid2.width, 10);
        assert_eq!(grid2.height, 4);
        assert_eq!(grid2.get(1, 1), Some(&RIGHT));
        assert_eq!(grid2.get(2, 1), Some(&DOWN));
        assert_eq!(grid2.get(7, 1), Some(&DOWN));
        assert_eq!(grid2.get(7, 2), Some(&RIGHT));
        assert_eq!(grid2.cells.iter().filter(|&&c| c == EMPTY).count(), 36);
        assert_eq!(grid2.active_right, HashSet::from([(1, 1), (7, 2)]));
        assert_eq!(grid2.active_down, HashSet::from([(2, 1), (7, 1)]));
    }

    #[test]
    fn can_display() {
        let grid = Grid {
            cells: Vec::from([
                EMPTY, EMPTY, EMPTY, RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, EMPTY, EMPTY, EMPTY,
            ]),
            height: 1,
            width: 11,
            active_right: HashSet::new(),
            active_down: HashSet::new(),
        };

        assert_eq!(format!("{}", grid), "...>>>>>...\n".to_string());

        let grid2 = "..........
.>v....v..
.......>..
..........\n"
            .to_string();

        assert_eq!(format!("{}", Grid::from(&grid2)), grid2);
    }

    #[test]
    fn can_iterate() {
        let mut grid = Grid::from(&"...>>>>>...\n".to_string());

        grid.iterate();
        assert_eq!(format!("{}", grid), "...>>>>.>..\n");
        assert_eq!(grid.active_right, HashSet::from([(6, 0), (8, 0)]));
        assert_eq!(grid.active_down.len(), 0);

        grid.iterate();
        assert_eq!(format!("{}", grid), "...>>>.>.>.\n");
        assert_eq!(grid.active_right, HashSet::from([(5, 0), (7, 0), (9, 0)]));

        grid.iterate();
        grid.iterate();

        assert_eq!(format!("{}", grid), ">..>.>.>.>.\n");

        let mut grid2 = Grid::from(
            &"..........
.>v....v..
.......>..
.........."
                .to_string(),
        );

        grid2.iterate();

        assert_eq!(
            format!("{}", grid2),
            "..........
.>........
..v....v>.
..........\n"
        );

        let mut grid3 = Grid::from(
            &"...>...
.......
......>
v.....>
......>
.......
..vvv.."
                .to_string(),
        );

        grid3.iterate();

        assert_eq!(
            format!("{}", grid3),
            "..vv>..
.......
>......
v.....>
>......
.......
....v..\n"
        );

        grid3.iterate();

        assert_eq!(
            format!("{}", grid3),
            "....v>.
..vv...
.>.....
......>
v>.....
.......
.......\n"
        );

        grid3.iterate();

        assert_eq!(
            format!("{}", grid3),
            "......>
..v.v..
..>v...
>......
..>....
v......
.......\n"
        );

        grid3.iterate();

        assert_eq!(
            format!("{}", grid3),
            ">......
..v....
..>.v..
.>.v...
...>...
.......
v......\n"
        );
    }

    #[test]
    fn can_iterate_until_static() {
        let mut grid = Grid::from(
            &"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
                .to_string(),
        );

        let count = grid.iterate_until_static();
        assert_eq!(count, 58);
        assert_eq!(
            format!("{}", grid),
            "..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv.....>>
>vv......>
.>v.vv.v..\n"
        );
    }
}
