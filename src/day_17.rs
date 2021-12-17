//! This is my solution for [Advent of Code - Day 17 - _Trick Shot_](https://adventofcode.com/2021/day/17)
//!
//! Today involved quite a bit of maths, and some brute-force looping through possible results. The
//! aim was to calculate whether a probe would hit a target area given a starting trajectory that
//! slowed down in the x direction (drag) and increased in the -ve y direction due to gravity.
//!
//! The first part could just be solved with maths [`highest_point`]. The second part I just brute
//! force calculated all permutations within upper and lower bounds for x and y,
//! [`all_trajectories`]. Working out a lower bound for x was interesting, but it doesn't save much
//! time over just using 1.

use std::collections::HashSet;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-17-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 17.
pub fn run() {
    let contents = fs::read_to_string("res/day-17-input").expect("Failed to read file");
    let target = parse_target(&contents);

    println!("The highest point reached is {}.", highest_point(target));
    println!(
        "The count of valid trajectories is {}.",
        all_trajectories(target).len()
    );
}

type Target = ((isize, isize), (isize, isize));

/// This is mostly discarding the unwanted syntax that makes this readable to humans.
/// # Example from puzzle specification
/// ```rust
/// assert_eq!(
///     parse_target(&"target area: x=20..30, y=-10..-5\n".to_string()),
///     ((20, 30), (-10, -5))
/// )
/// ```
/// Note the trailing new line needed to match the input file.
fn parse_target(input: &String) -> Target {
    fn parse_range(range: &str) -> (isize, isize) {
        if let Some((a, b)) = range.split_once("..") {
            (a.parse().unwrap(), b.parse().unwrap())
        } else {
            panic!("Unexpected range: {}", range)
        }
    }

    if let Some((x, y)) = input
        .trim()
        .replace("target area: x=", "")
        .replace(" y=", "")
        .split_once(",")
    {
        (parse_range(x), parse_range(y))
    } else {
        panic!("unexpected input: {}", input)
    }
}

/// The delta on the y-axis of -1 is such that the y co-ordinates of the points on the downward
/// portion of the trajectory are the same as the upward portion, so there will be a co-ordinate
/// on the x-axis for each trajectory that starts upwards. Further the trajectory(ies) with the
/// highest peak will be travelling fastest at that point. As the target area is below the origin,
/// The fastest possible trajectory at that point will be the x-axis followed by the lowest altitude
/// of the target area (y_min). As the delta is -1 the height of the peak is a triangular number
/// (`n * (n + 1) / 2`), where n is the distance of that final step 0..y_min. This is the distance
/// between the peak and y_min. To get the height above the origin, we need to go back one step
/// (`(n-1) * ((n-1) + 1) / 2`), simplified and with -y_min  substituted for n: `(-y_min - 1) *
/// -y_min / 2`.
fn highest_point(target: Target) -> isize {
    let (_, (y_min, _)) = target;
    (-y_min - 1) * -y_min / 2
}

/// Determine if a given trajectory hits the target by recursively stepping through the co-ordinates
/// it covers.
fn is_hit(
    (pos_x, pos_y): (isize, isize),
    (dx, dy): (isize, isize),
    ((x1, x2), (y1, y2)): Target,
) -> bool {
    // If we've gone beyond the area, this was a miss
    if pos_x > x2 || pos_y < y1 {
        return false;
    }

    // if the co-ordinates are on or within the target area bounds, this was a miss
    if pos_x >= x1 && pos_x <= x2 && pos_y >= y1 && pos_y <= y2 {
        return true;
    }

    // otherwise apply and update the deltas and continue
    is_hit(
        (pos_x + dx, pos_y + dy),
        ((dx - 1).max(0), dy - 1),
        ((x1, x2), (y1, y2)),
    )
}

/// Calculate an upper and lower bound for x and y co-ordinates, then brute-force iterate through
/// each permutation. There is probably a more efficient solution in working the sets of steps each
/// relevant x and y magnitude will be in the target area and intersecting those, but the
/// brute-force method runs in 1-2ms so is good enough.
///
/// The lower bound for y, and the upper bound for x is the trajectory that hits the bottom right of
/// the target area after it's first step. As noted in [`highest_point`] the highest positive y
/// trajectory is -_target_y_min - 1. The lowest x needs to be large enough that it reaches
/// target_x_min before decaying to a delta of 0.
///
/// so:
/// ```text
/// x (x + 1)                               | max x distance for starting x is
/// ---------   >= target_x_min             | triangular number for x
///     2                                   |
///                                         |
/// x (x + 1)   >= target_x_min * 2         | multiply both sides by 2
///                                         |
/// x² + x      >= target_x_min * 2         | re-arrange
///                                         |
/// x² + 2x + 1 >  target_x_min * 2         | x is >= 0, so adding x + 1  
///                                         | is still > RHS
///                                         |
/// (x + 1)²    >  target_x_min * 2         | re-arrange, (x + 1)(x + 1) ===
///                                         | x² + 2x + 1
///                                         |
///                 /----------------       |
/// x + 1       > \/ target_x_min * 2       | square root both sides
///                                         |
///                 /----------------       |
/// x           > \/ target_x_min * 2  - 1  | -1 both sides
/// ```
fn all_trajectories(target: Target) -> HashSet<(isize, isize)> {
    let mut out = HashSet::new();

    let ((x1, x2), (y1, _)) = target;
    //
    let x_min = ((x1 as f64 * 2.0).sqrt().ceil() - 1.0) as isize;
    let x_max = x2;
    let y_min = y1;
    let y_max = -y1 - 1;

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            if is_hit((0, 0), (x, y), target) {
                out.insert((x, y));
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::day_17::{all_trajectories, highest_point, is_hit, parse_target};
    use std::collections::HashSet;

    #[test]
    fn can_parse() {
        assert_eq!(
            parse_target(&"target area: x=20..30, y=-10..-5\n".to_string()),
            ((20, 30), (-10, -5))
        )
    }

    #[test]
    fn can_calc_highest() {
        let target = ((20, 30), (-10, -5));
        assert_eq!(highest_point(target), 45)
    }

    #[test]
    fn can_calc_hit() {
        let target = ((20, 30), (-10, -5));
        assert_eq!(is_hit((0, 0), (23, -10), target), true);
        assert_eq!(is_hit((0, 0), (23, -11), target), false);
    }

    #[test]
    fn can_calc_all_hits() {
        let target = ((20, 30), (-10, -5));
        let actual = all_trajectories(target);
        let expected = HashSet::from([
            (23, -10),
            (25, -9),
            (27, -5),
            (29, -6),
            (22, -6),
            (21, -7),
            (9, 0),
            (27, -7),
            (24, -5),
            (25, -7),
            (26, -6),
            (25, -5),
            (6, 8),
            (11, -2),
            (20, -5),
            (29, -10),
            (6, 3),
            (28, -7),
            (8, 0),
            (30, -6),
            (29, -8),
            (20, -10),
            (6, 7),
            (6, 4),
            (6, 1),
            (14, -4),
            (21, -6),
            (26, -10),
            (7, -1),
            (7, 7),
            (8, -1),
            (21, -9),
            (6, 2),
            (20, -7),
            (30, -10),
            (14, -3),
            (20, -8),
            (13, -2),
            (7, 3),
            (28, -8),
            (29, -9),
            (15, -3),
            (22, -5),
            (26, -8),
            (25, -8),
            (25, -6),
            (15, -4),
            (9, -2),
            (15, -2),
            (12, -2),
            (28, -9),
            (12, -3),
            (24, -6),
            (23, -7),
            (25, -10),
            (7, 8),
            (11, -3),
            (26, -7),
            (7, 1),
            (23, -9),
            (6, 0),
            (22, -10),
            (27, -6),
            (8, 1),
            (22, -8),
            (13, -4),
            (7, 6),
            (28, -6),
            (11, -4),
            (12, -4),
            (26, -9),
            (7, 4),
            (24, -10),
            (23, -8),
            (30, -8),
            (7, 0),
            (9, -1),
            (10, -1),
            (26, -5),
            (22, -9),
            (6, 5),
            (7, 5),
            (23, -6),
            (28, -10),
            (10, -2),
            (11, -1),
            (20, -9),
            (14, -2),
            (29, -7),
            (13, -3),
            (23, -5),
            (24, -8),
            (27, -9),
            (30, -7),
            (28, -5),
            (21, -10),
            (7, 9),
            (6, 6),
            (21, -5),
            (27, -10),
            (7, 2),
            (30, -9),
            (21, -8),
            (22, -7),
            (24, -9),
            (20, -6),
            (6, 9),
            (29, -5),
            (8, -2),
            (27, -8),
            (30, -5),
            (24, -7),
        ]);

        let diff: HashSet<&(isize, isize)> = if expected.len() > actual.len() {
            expected.difference(&actual).collect()
        } else {
            actual.difference(&expected).collect()
        };

        assert_eq!(diff, HashSet::new())
    }
}
