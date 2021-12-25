//! This is my solution for [Advent of Code - Day 25 - _Title_](https://adventofcode.com/2021/day/25)
//!
//!

use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use Cell::{DOWN, EMPTY, RIGHT};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Cell {
    EMPTY,
    DOWN,
    RIGHT, //
}

impl TryFrom<char> for Cell {
    type Error = ();

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

#[derive(Eq, PartialEq, Debug)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    active_right: HashSet<(usize, usize)>,
    active_down: HashSet<(usize, usize)>,
}

impl From<&String> for Grid {
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
    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.cells.get(self.pos_of(x, y))
        }
    }

    fn pos_of(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn swap(&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) {
        let pos1 = self.pos_of(x1, y1);
        let pos2 = self.pos_of(x2, y2);
        self.cells.swap(pos1, pos2)
    }

    fn can_move(&self, x: usize, y: usize, direction: Cell) -> bool {
        match direction {
            RIGHT => self.get((x + 1) % self.width, y) == Some(&EMPTY),
            DOWN => self.get(x, (y + 1) % self.height) == Some(&EMPTY),
            _ => false,
        }
    }

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
