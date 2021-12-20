//! This is my solution for [Advent of Code - Day 20 - _Trench Map_](https://adventofcode.com/2021/day/20)
//!
//!

use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::str::Lines;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Image {
    pixels: HashSet<(isize, isize)>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    default_pixel: bool,
}

impl<'a> From<(&mut Lines<'a>, isize, isize)> for Image {
    fn from((lines, min_x, min_y): (&mut Lines<'a>, isize, isize)) -> Self {
        let mut pixels = HashSet::new();
        let mut max_x = min_x;
        let mut max_y = min_y - 1;

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
    fn iterate(&self, bitmap: &Vec<bool>) -> Image {
        // The area affected by non-default pixels grows by 1 each iteration
        let min_x = self.min_x - 1;
        let min_y = self.min_y - 1;
        let max_x = self.max_x + 1;
        let max_y = self.max_y + 1;

        let mut pixels = HashSet::new();

        (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .for_each(|(x, y)| {
                if self.map_pixel(x, y, bitmap) {
                    pixels.insert((x, y));
                };
            });

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

    fn map_pixel(&self, x: isize, y: isize, bitmap: &Vec<bool>) -> bool {
        let mut index: usize = 0;
        (y - 1..=y + 1)
            .cartesian_product(x - 1..=x + 1)
            .for_each(|(y1, x1)| {
                let pixel =
                    if x1 < self.min_x || x1 > self.max_x || y1 < self.min_y || y1 > self.max_y {
                        self.default_pixel
                    } else {
                        self.pixels.contains(&(x1, y1))
                    };

                index = (index << 1) + (pixel as usize);
            });

        *bitmap.get(index).unwrap()
    }

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

        let expected = Image::from((&mut expected_lines, -1, -1));

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
