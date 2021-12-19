//! This is my solution for [Advent of Code - Day 19 - _Beacon Scanner_](https://adventofcode.com/2021/day/19)
//!
//! Today was horrible. I find 3D geometry really hard as I can't visualise it very well, and that hinders me
//! reasoning about it. It was a very similar puzzle last year that caused me to stall, so I'm very glad I worked
//! through this one and got to a solution. It is not very efficient, it takes 1s-1.5s to run which is a lot given
//! all previous days run in ~300-400ms combined. But I don't think I have a decent enough idea how to improve it
//! that I'm just happy to have solved it and will take the speed hit.
//!
//! [`parse_scanners`] is fairly simple, it splits the input on the double line breaks between scanner inputs, and
//! for each then returns the list of relative beacon co-ordinates. [`try_merge`] does all the heavy lifting, it
//! takes the set of beacons fixed so far, and a scanner, and tries for each possible rotation to position the
//! beacons so that there is an overlap of twelve beacons. If it succeeds it merges the translated beacon permissions
//! into the set of fixed beacons, and returns the offset of the sensor from the first. [`merge_all`] takes the initial
//! list of scanner inputs, sets the first as the base scanner, fixing all those beacons. Then repeatedly scans the
//! remaining scanners until it finds one that merges with the current set (using [`try_merge`]). Once found, it
//! removes that scanner from the list, and stores its offset for solving part two.
//!
//! Part one is solved by just taking the length of the set of beacons returned by [`merge_all`]. For part two
//! [`largest_distance`] takes the set of all scanner offsets, iterates through the pair combinations, mapping each
//! pair to their manhatten distance, then takes the max of those.

use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

/// Type alias for a 3D co-ordinate, used for beacon and scanner offsets.
type Position = (isize, isize, isize);
/// Type alias for the data set of one scanner. A list of the relative positions of all beacons the scanner can detect.
type Scanner = Vec<Position>;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-19-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 19.
pub fn run() {
    let contents = fs::read_to_string("res/day-19-input").expect("Failed to read file");
    let scanners = parse_scanners(&contents);
    let (beacons, scanner_positions) = merge_all(&scanners);
    println!("There are {} distinct beacons.", beacons.len());
    println!(
        "The greatest manhattan distance between scanners is {}.",
        largest_distance(&scanner_positions)
    );
}

fn parse_scanners(input: &String) -> Vec<Scanner> {
    input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1) // header
                .map(|line| {
                    let coords: Vec<isize> = line
                        .split(",")
                        .map(|c| c.parse::<isize>().unwrap())
                        .collect();
                    (coords[0], coords[1], coords[2])
                })
                .collect()
        })
        .collect()
}

/// Expand a scanner into each of the 24 possible rotations. I started off trying to build the set of rotation
/// functions as a static vector of closures that could be cached using `lazy_static!` but I was wasting too much
/// time trying to satisfy the compiler so ended up with this mess as I inlined the 6 valid combinations for each ±x,
/// ±y permutation.
fn rotations(scanner: &Scanner) -> Vec<Scanner> {
    let signs = Vec::from([-1isize, 1isize]);
    signs
        .clone()
        .iter()
        .cartesian_product(signs)
        // For each of the 4 ±x,±y pairs, the z can only have one sign - the other sign mirrors the set.
        .flat_map(|(&sign_x, sign_y)| {
            let sign_z = if sign_x == sign_y { 1 } else { -1 };

            // It was easier to type them out using multiple carets than use matrices
            Vec::from([
                scanner
                    .iter()
                    .map(|(x, y, z)| (x * sign_x, y * sign_y, z * sign_z))
                    .collect(),
                scanner
                    .iter()
                    .map(|(x, y, z)| (x * sign_x, z * -sign_z, y * sign_y))
                    .collect(),
                scanner
                    .iter()
                    .map(|(x, y, z)| (y * sign_y, x * sign_x, z * -sign_z))
                    .collect(),
                scanner
                    .iter()
                    .map(|(x, y, z)| (y * sign_y, z * sign_z, x * sign_x))
                    .collect(),
                scanner
                    .iter()
                    .map(|(x, y, z)| (z * sign_z, x * sign_x, y * sign_y))
                    .collect(),
                scanner
                    .iter()
                    .map(|(x, y, z)| (z * -sign_z, y * sign_y, x * sign_x))
                    .collect(),
            ])
        })
        .collect()
}

/// Explode the scanner into its 24 rotations, then for each, pair each up with every element in the fixed beacon set,
/// and work out the position delta needed to make them match up. If we can find 12 or more point pairs that share the
/// same delta, that delta gives a translation for the current rotation that has enough overlap to be confident that
/// is is a match. Take the first rotation (if any) that produces a match. If a match is found, apply that delta to the
/// current rotation of the scanner data, and merge those points with the existing fixed set. Then return the delta
/// as that is also the scanner position. [Itertools::cartesian_product], [`Itertools::counts`], and
/// [`Iterator::find_map`] respectively do the pairing of scanner points with the existing beacon set, grouping by
/// delta, and finding the first match (if any) both for the rotations, and delta groups.
fn try_merge(beacon_set: &mut HashSet<Position>, scanner: &Scanner) -> Option<Position> {
    let rots = rotations(&scanner);
    // Find a rotation with overlap
    let maybe_match = rots.iter().find_map(|scanner| {
        beacon_set
            .iter()
            .cartesian_product(scanner)
            .map(|((x1, y1, z1), (x2, y2, z2))| ((x1 - x2, y1 - y2, z1 - z2)))
            .counts()
            .iter()
            .find_map(|(&k, &v)| if v >= 12 { Some((scanner, k)) } else { None })
    });

    // Insert it into the existing beacon set
    if let Some((scanner, (dx, dy, dz))) = maybe_match {
        scanner
            .iter()
            .map(|(x, y, z)| (x + dx, y + dy, z + dz))
            .for_each(|(x, y, z)| {
                beacon_set.insert((x, y, z));
            });
        Some((dx, dy, dz))
    } else {
        None
    }
}

/// Use the first scanner as the base set, and repeatedly hunt for scanners that can be merged until the relative
/// positions of all of them has been determined, Return the set of beacons that results in, and the list of scanner
/// offsets. Note the order of the scanner list doesn't matter so the more efficient [`Vec::swap_remove`] can be used.
fn merge_all(scanners: &Vec<Scanner>) -> (HashSet<Position>, HashSet<Position>) {
    // Make a mutable copy so that scanners can be removed as they're matched
    let mut to_merge = scanners.clone();
    // Seed the set of beacons from the first scanner dataset
    let mut beacon_set: HashSet<Position> = to_merge.swap_remove(0).iter().map(|&a| a).collect();
    // The first scanner is the reference point, so is at the origin by definition.
    let mut scanner_pos: HashSet<Position> = HashSet::from([(0, 0, 0)]);
    // find_map again to search for any one scanner that can be combined with the current set.
    while let Some((i, pos)) = to_merge
        .iter()
        // track which scanner we're at to allow removing the correct one
        .enumerate()
        // try merge will mutate the set if it finds a match
        .find_map(|(i, scanner)| try_merge(&mut beacon_set, scanner).map(|pos| (i, pos)))
    {
        // remove the scanner from the pending list
        to_merge.swap_remove(i);
        // keep the offset for use in part two
        scanner_pos.insert(pos);
    }

    // return the datasets needed to calculate each part's result.
    (beacon_set, scanner_pos)
}

/// Take the set of scanner offsets returned by [`merge_all`], explode into all combinations of pairs with
/// [`Itertools::tuple_combinations`], map those to the manhattan distance, and take the maximum.
fn largest_distance(scanner_positions: &HashSet<Position>) -> usize {
    scanner_positions
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(&(x1, y1, z1), &(x2, y2, z2))| {
            ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) as usize
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day_19::{
        largest_distance, merge_all, parse_scanners, rotations, try_merge, Position, Scanner,
    };

    fn sample_input() -> String {
        "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"
            .to_string()
    }

    #[test]
    fn can_parse() {
        let input = "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 1 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0
"
        .to_string();

        assert_eq!(
            parse_scanners(&input),
            Vec::from([
                Vec::from([
                    (-1, -1, 1),
                    (-2, -2, 2),
                    (-3, -3, 3),
                    (-2, -3, 1),
                    (5, 6, -4),
                    (8, 0, 7),
                ]),
                Vec::from([
                    (1, -1, 1),
                    (2, -2, 2),
                    (3, -3, 3),
                    (2, -1, 3),
                    (-5, 4, -6),
                    (-8, -7, 0),
                ])
            ])
        )
    }

    #[test]
    fn can_merge() {
        let scanners = parse_scanners(&sample_input());
        let mut beacon_set = scanners.get(0).unwrap().iter().map(|&a| a).collect();

        let to_merge_first = scanners.get(1).unwrap();
        assert_eq!(
            try_merge(&mut beacon_set, &to_merge_first),
            Some((68, -1246, -43))
        );

        let to_merge_second = scanners.get(4).unwrap();
        assert_eq!(
            try_merge(&mut beacon_set, &to_merge_second),
            Some((-20, -1133, 1061))
        );

        let to_merge_third = scanners.get(2).unwrap();
        assert_eq!(
            try_merge(&mut beacon_set, &to_merge_third),
            Some((1105, -1205, 1229))
        );

        let to_merge_fourth = scanners.get(3).unwrap();
        assert_eq!(
            try_merge(&mut beacon_set, &to_merge_fourth),
            Some((-92, -2380, -20))
        );
    }

    #[test]
    fn can_rotate() {
        let scanner: Scanner = Vec::from([(1, 2, 3)]);
        let rotations: HashSet<Position> = rotations(&scanner)
            .iter()
            .flat_map(|a| a.get(0).map(|&a| a))
            .collect();
        let expected: HashSet<Position> = HashSet::from([
            (1, 2, 3),
            (2, -1, 3),
            (-1, -2, 3),
            (-2, 1, 3),
            (3, 2, -1),
            (2, -3, -1),
            (-3, -2, -1),
            (-2, 3, -1),
            (3, -1, -2),
            (-1, -3, -2),
            (-3, 1, -2),
            (1, 3, -2),
            (3, -2, 1),
            (-2, -3, 1),
            (-3, 2, 1),
            (2, 3, 1),
            (3, 1, 2),
            (1, -3, 2),
            (-3, -1, 2),
            (-1, 3, 2),
            (-1, 2, -3),
            (2, 1, -3),
            (1, -2, -3),
            (-2, -1, -3),
        ]);

        assert_eq!(rotations, expected);
    }

    #[test]
    fn can_merge_all() {
        let scanners = parse_scanners(&sample_input());
        let (beacons, _) = merge_all(&scanners);
        assert_eq!(beacons.len(), 79);
        assert_eq!(
            beacons,
            HashSet::from([
                (-892, 524, 684),
                (-876, 649, 763),
                (-838, 591, 734),
                (-789, 900, -551),
                (-739, -1745, 668),
                (-706, -3180, -659),
                (-697, -3072, -689),
                (-689, 845, -530),
                (-687, -1600, 576),
                (-661, -816, -575),
                (-654, -3158, -753),
                (-635, -1737, 486),
                (-631, -672, 1502),
                (-624, -1620, 1868),
                (-620, -3212, 371),
                (-618, -824, -621),
                (-612, -1695, 1788),
                (-601, -1648, -643),
                (-584, 868, -557),
                (-537, -823, -458),
                (-532, -1715, 1894),
                (-518, -1681, -600),
                (-499, -1607, -770),
                (-485, -357, 347),
                (-470, -3283, 303),
                (-456, -621, 1527),
                (-447, -329, 318),
                (-430, -3130, 366),
                (-413, -627, 1469),
                (-345, -311, 381),
                (-36, -1284, 1171),
                (-27, -1108, -65),
                (7, -33, -71),
                (12, -2351, -103),
                (26, -1119, 1091),
                (346, -2985, 342),
                (366, -3059, 397),
                (377, -2827, 367),
                (390, -675, -793),
                (396, -1931, -563),
                (404, -588, -901),
                (408, -1815, 803),
                (423, -701, 434),
                (432, -2009, 850),
                (443, 580, 662),
                (455, 729, 728),
                (456, -540, 1869),
                (459, -707, 401),
                (465, -695, 1988),
                (474, 580, 667),
                (496, -1584, 1900),
                (497, -1838, -617),
                (527, -524, 1933),
                (528, -643, 409),
                (534, -1912, 768),
                (544, -627, -890),
                (553, 345, -567),
                (564, 392, -477),
                (568, -2007, -577),
                (605, -1665, 1952),
                (612, -1593, 1893),
                (630, 319, -379),
                (686, -3108, -505),
                (776, -3184, -501),
                (846, -3110, -434),
                (1135, -1161, 1235),
                (1243, -1093, 1063),
                (1660, -552, 429),
                (1693, -557, 386),
                (1735, -437, 1738),
                (1749, -1800, 1813),
                (1772, -405, 1572),
                (1776, -675, 371),
                (1779, -442, 1789),
                (1780, -1548, 337),
                (1786, -1538, 337),
                (1847, -1591, 415),
                (1889, -1729, 1762),
                (1994, -1805, 1792),
            ])
        );
    }

    #[test]
    fn can_find_largest_distance() {
        let scanners = parse_scanners(&sample_input());
        let (_, scanner_positions) = merge_all(&scanners);
        assert_eq!(largest_distance(&scanner_positions), 3621);
    }
}
