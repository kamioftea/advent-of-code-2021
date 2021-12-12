/// A representation of a 2D grid of u8s. Originally implemented for [`crate::day_9`], another grid was needed for
/// [`crate::day_11`] and so common methods were extracted to this shared module
#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    /// Store the numbers in a 1D list...
    pub numbers: Vec<u8>,
    /// ...and use the width to determine the 1D offset as a 2D co-ordinate
    pub width: usize,
}

impl From<String> for Grid {
    /// Turn the characters into digits and concatenate, caching the width
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

impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            numbers: self.numbers.to_vec(),
            width: self.width,
        }
    }
}

/// Temporary struct representing an iterator over a grid
pub struct GridCoords<'a> {
    /// Reference to the grid being iterated
    grid: &'a Grid,
    /// The current position of the iterator
    pos: usize,
}

impl<'a> Iterator for GridCoords<'a> {
    type Item = ((usize, usize), u8);

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.grid.get_with_coords(self.pos);
        self.pos = self.pos + 1;

        curr
    }
}

impl Grid {
    /// Helper to abstract iterating over the whole grid
    pub fn iter(&self) -> GridCoords {
        GridCoords { grid: self, pos: 0 }
    }

    /// Return the value at the given co-ordinates
    pub fn get(&self, y: usize, x: usize) -> Option<u8> {
        if x >= self.width {
            return None;
        }
        self.numbers.get(x + y * self.width).map(|&t| t)
    }

    /// Update the value in a given cell
    pub fn set(&mut self, y: usize, x: usize, val: u8) -> bool {
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

    /// Used by [`GridCoords::next`] and other iterators over the grid , e.g. [`Grid::iterate_and_flash`] to turn the
    /// current iterator position into the x/y co-ordinates and the value in that cell.
    pub fn get_with_coords(&self, pos: usize) -> Option<((usize, usize), u8)> {
        let x = pos % self.width;
        let y = pos / self.width;

        self.numbers.get(pos).map(|&val| ((y, x), val))
    }

    /// Given a cell and a delta, return the new co-ordinates and the value at those co-ordinates if it is within the
    /// grid, None otherwise.
    pub fn get_relative(
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

    /// Dump the grid to stdout - useful for visualising the grid when debugging
    #[allow(dead_code)]
    pub fn print(&self) -> String {
        let (_, out) = self
            .iter()
            .fold((0usize, "".to_string()), |(prev_y, out), ((y, _), v)| {
                (
                    y,
                    format!(
                        "{}{}{}",
                        out,
                        if y != prev_y { "\n" } else { "" },
                        if v <= 9 {
                            v.to_string()
                        } else {
                            "#".to_string()
                        },
                    ),
                )
            });

        out.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_9::Grid;

    fn sample_input() -> String {
        "12345\n\
        23456\n\
        34567\n\
        45678\n\
        56789"
            .to_string()
    }

    #[test]
    fn can_print() {
        let input = sample_input();

        let mut grid = Grid::from(input.clone());

        assert_eq!(grid.print(), input);

        grid.set(4, 4, 10);

        assert_eq!(grid.print(), input.replace("9", "#"));
    }

    #[test]
    fn set_ignores_out_of_bounds() {
        let mut grid = Grid::from(sample_input());

        assert_eq!(grid.set(5, 0, 9), false);
        assert_eq!(grid.set(0, 5, 9), false);
        assert_eq!(grid.set(5, 5, 9), false);
        // unchanged
        assert_eq!(grid.print(), sample_input());
    }
}
