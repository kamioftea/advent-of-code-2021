//! This is my solution for [Advent of Code - Day 20 - _Trench Map_](https://adventofcode.com/2021/day/20)
//!
//! Today was iterating a grid, with the twist being that the puzzle input exhibited behaviour that
//! the sample input didn't. This feels unfair, maybe it's just to test debugging skills, but those
//! will be challenged throughout anyway. I don't like that it seemed to be purposefully setting
//! people up to fail.
//!
//! Luckily I noticed the trick. The first bit of the puzzle bitmap being set means that all the
//! infinite co-ordinates beyond the edge of the image data, which are initially set to `0` will all
//! flip to `1`, on each odd iteration. They will then flip back to `0` on even iterations as the
//! final bit of the bitmap is set to `0`. Having spotted this I decided to implement the grid as
//! a set of co-ordinates that are set within the known central area, the current bounds of the
//! known area, and the default value of every bit beyond that. [`Image`] and the methods
//! implemented for it make up the bulk of today's solution.
//!
//! [`parse_input`] takes the first line and transforms it into the bitmap to lookup new pixel
//! values, then passes the rest of the lines to [`Image::from`] to parse the rest into the seed
//! image. I originally had this inlined in [`parse_input`] but extracted it and included variables
//! for the origin as a way to parse the examples of iterated images in the specification when
//! writing tests.
//!
//! [`Image::map_pixel`] handles iterating a single pixel by looking up its surrounds, building them
//! into the bitmap index, and returning the relevant bit. [`Image::iterate`] handles a single
//! iteration of the image. This grows the image area by 1 (only pixels adjacent to the existing
//! image data will be affected by it), map all the pixels in the new area, and set the new default
//! value for pixels outside the area. Finally [`Image::iterate_n`] iterates the image the required
//! number of times, two for part one, fifty for part two.

use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::str::Lines;

/// Represents an image as the set of pixels that are on, the bounds of the current image data, and
/// the default value for pixels outside this area.
#[derive(Eq, PartialEq, Debug, Clone)]
struct Image {
    /// set of active pixels within (min_x, min_y) .. (max_x, max_y)
    pixels: HashSet<(isize, isize)>,
    /// lower bound of the image data x co-ordinate values
    min_x: isize,
    /// upper bound of the image data x co-ordinate values
    max_x: isize,
    /// lower bound of the image data y co-ordinate values
    min_y: isize,
    /// upper bound of the image data y co-ordinate values
    max_y: isize,
    /// The value of all pixels outside the image data bounds
    default_pixel: bool,
}

impl<'a> From<(&mut Lines<'a>, isize, isize)> for Image {
    /// Takes lines as [`parse_input`] needs to take the first two lines before this is called
    fn from((lines, min_x, min_y): (&mut Lines<'a>, isize, isize)) -> Self {
        let mut pixels = HashSet::new();
        // track the bounds of the image data
        let mut max_x = min_x;
        // decrease by one as it will be incremented at the start of each loop, including the
        // first.
        let mut max_y = min_y - 1;

        // insert all the co-ordinates where we see a #
        while let Some(line) = lines.next() {
            // can't enumerate once already taken a value from the iterator
            max_y += 1;
            line.chars().enumerate().for_each(|(raw_x, chr)| {
                let x = raw_x as isize + min_x;
                if chr == '#' {
                    pixels.insert((x, max_y));
                }

                if max_x < x {
                    max_x = x;
                };
            });
        }

        Image {
            pixels,
            min_x,
            max_x,
            min_y,
            max_y,
            default_pixel: false,
        }
    }
}

impl Image {
    /// Builds a new image by expanding the area by 1 pixel in all directions, and mapping those
    /// based in the image data / default pixel value, finally calculating the new value for the
    /// default.
    fn iterate(&self, bitmap: &Vec<bool>) -> Image {
        // The area affected by non-default pixels grows by 1 each iteration
        let min_x = self.min_x - 1;
        let min_y = self.min_y - 1;
        let max_x = self.max_x + 1;
        let max_y = self.max_y + 1;

        let mut pixels = HashSet::new();

        // iterate through all x,y pairs in the new image area, mapping each one
        (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .for_each(|(x, y)| {
                if self.map_pixel(x, y, bitmap) {
                    pixels.insert((x, y));
                };
            });

        // All pixels outside the new image area were surrounded entirely by other default pixels
        // in the existing image. If it was previously unset, all bits in the index are unset, so
        // index is 0, otherwise all bits are set the and index is 111111111 i.e. 511.
        let &default_pixel = if self.default_pixel {
            bitmap.get(511).unwrap()
        } else {
            bitmap.get(0).unwrap()
        };

        Image {
            pixels,
            min_x,
            max_x,
            min_y,
            max_y,
            default_pixel,
        }
    }

    /// Get if a specific pixel will be active in the next iteration
    fn map_pixel(&self, x: isize, y: isize, bitmap: &Vec<bool>) -> bool {
        let mut index: usize = 0;
        // for each pixel in the 3x3 grid surrounding it - the order matters here
        (y - 1..=y + 1)
            .cartesian_product(x - 1..=x + 1)
            .for_each(|(y1, x1)| {
                let pixel =
                    // Check that the pixel we're reading from is inside the current bounds
                    if x1 < self.min_x || x1 > self.max_x || y1 < self.min_y || y1 > self.max_y {
                        self.default_pixel
                    } else {
                        self.pixels.contains(&(x1, y1))
                    };

                // build by shifting the pixels on from the right
                index = (index << 1) + (pixel as usize);
            });

        // lookup the corresponding pixel in the bitmap
        *bitmap.get(index).unwrap()
    }

    /// Repeatedly iterate the current image n times
    fn iterate_n(&self, bitmap: &Vec<bool>, n: usize) -> Image {
        (0..n).fold(self.clone(), |acc, _| acc.iterate(&bitmap))
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-20-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 20.
pub fn run() {
    let contents = fs::read_to_string("res/day-20-input").expect("Failed to read file");
    let (bitmap, image) = parse_input(&contents);

    let iterated_2 = image.iterate_n(&bitmap, 2);
    println!(
        "After 2 iterations there are {} active pixels.",
        iterated_2.pixels.len()
    );

    let iterated_50 = iterated_2.iterate_n(&bitmap, 48);
    println!(
        "After 50 iterations there are {} active pixels.",
        iterated_50.pixels.len()
    )
}

/// Extract the first line as the bitmap lookup, then delegate parsing the seed image to
/// [`Image::from`]
fn parse_input(input: &String) -> (Vec<bool>, Image) {
    let mut lines = input.lines();
    let bitmap: Vec<bool> = lines
        .next()
        .expect("Empty input")
        .chars()
        .map(|c| c == '#')
        .collect();
    lines.next(); // skip blank line

    (bitmap, Image::from((&mut lines, 0, 0)))
}

#[cfg(test)]
mod tests {
    use crate::day_20::{parse_input, Image};
    use std::collections::HashSet;

    fn sample_input() -> String {
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"
            .to_string()
    }

    #[test]
    fn can_parse() {
        let (bitmap, image) = parse_input(&sample_input());

        assert_eq!(bitmap.len(), 512);
        assert_eq!(bitmap[0], false);
        assert_eq!(bitmap[1], false);
        assert_eq!(bitmap[2], true);
        assert_eq!(bitmap[3], false);
        assert_eq!(bitmap[4], true);
        assert_eq!(bitmap[5], false);
        assert_eq!(bitmap[6], false);
        assert_eq!(bitmap[7], true);
        assert_eq!(bitmap[8], true);
        assert_eq!(bitmap[9], true);
        assert_eq!(bitmap[10], true);
        assert_eq!(bitmap[20], true);
        assert_eq!(bitmap[30], true);
        assert_eq!(bitmap[40], true);
        assert_eq!(bitmap[50], true);
        assert_eq!(bitmap[60], false);
        assert_eq!(bitmap[70], false);
        assert_eq!(bitmap[511], true);

        assert_eq!(image.min_x, 0);
        assert_eq!(image.min_y, 0);
        assert_eq!(image.max_x, 4);
        assert_eq!(image.max_y, 4);

        let expected = HashSet::from([
            (0, 0),
            (3, 0),
            (0, 1),
            (0, 2),
            (1, 2),
            (4, 2),
            (2, 3),
            (2, 4),
            (3, 4),
            (4, 4),
        ]);

        let missing: HashSet<&(isize, isize)> = expected.difference(&image.pixels).collect();
        let additional: HashSet<&(isize, isize)> = image.pixels.difference(&expected).collect();

        assert_eq!(missing, HashSet::new());
        assert_eq!(additional, HashSet::new());
    }

    #[test]
    fn can_map_pixel() {
        let (bitmap, image) = parse_input(&sample_input());

        assert_eq!(image.map_pixel(2, 2, &bitmap), true);
        assert_eq!(image.map_pixel(-1, -1, &bitmap), false);
        assert_eq!(image.map_pixel(0, -1, &bitmap), true);
        assert_eq!(image.map_pixel(-1, 0, &bitmap), true);
    }

    #[test]
    fn can_iterate() {
        let (bitmap, image) = parse_input(&sample_input());

        let mut expected_lines = ".##.##.
#..#.#.
##.#..#
####..#
.#..##.
..##..#
...#.#."
            .lines();

        let mut expected = Image::from((&mut expected_lines, -1, -1));
        // isn't needed for the test input, but here for completeness
        expected.default_pixel = *bitmap.get(0).unwrap();

        assert_eq!(image.iterate(&bitmap), expected);

        assert_eq!(expected.iterate(&bitmap).pixels.len(), 35);
    }

    #[test]
    fn can_iterate_n() {
        let (bitmap, image) = parse_input(&sample_input());

        assert_eq!(image.iterate_n(&bitmap, 2).pixels.len(), 35);
        assert_eq!(image.iterate_n(&bitmap, 50).pixels.len(), 3351);
        assert_eq!(
            image
                .iterate_n(&bitmap, 2)
                .iterate_n(&bitmap, 48)
                .pixels
                .len(),
            3351
        );
    }
}
