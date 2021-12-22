//! This is my solution for [Advent of Code - Day 22 - _Reactor Reboot_](https://adventofcode.com/2021/day/22)
//!
//!

use std::fs;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Cuboid {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

fn part_one_limit() -> Cuboid {
    Cuboid::new(-50, 50, -50, 50, -50, 50)
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Instruction {
    is_on: bool,
    cuboid: Cuboid,
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        if let Some((on_off, coords)) = line.split_once(" ") {
            let is_on = on_off == "on";
            let numbers: Vec<isize> = coords
                .split(&['=', '.', ','][..])
                .flat_map(|n| n.parse::<isize>().ok())
                .collect();
            Instruction {
                is_on,
                cuboid: Cuboid::new(
                    numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5],
                ),
            }
        } else {
            panic!("invalid cuboid {}", line)
        }
    }
}

impl Instruction {
    #[cfg(test)]
    fn new(
        is_on: bool,
        x_min: isize,
        x_max: isize,
        y_min: isize,
        y_max: isize,
        z_min: isize,
        z_max: isize,
    ) -> Instruction {
        Instruction {
            is_on,
            cuboid: Cuboid::new(x_min, x_max, y_min, y_max, z_min, z_max),
        }
    }
}

impl Cuboid {
    fn new(
        x_min: isize,
        x_max: isize,
        y_min: isize,
        y_max: isize,
        z_min: isize,
        z_max: isize,
    ) -> Cuboid {
        Cuboid {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }

    fn intersects(&self, other: &Cuboid) -> Option<Cuboid> {
        let x_min = self.x_min.max(other.x_min);
        let x_max = self.x_max.min(other.x_max);
        let y_min = self.y_min.max(other.y_min);
        let y_max = self.y_max.min(other.y_max);
        let z_min = self.z_min.max(other.z_min);
        let z_max = self.z_max.min(other.z_max);

        if x_min <= x_max && y_min <= y_max && z_min <= z_max {
            Some(Cuboid {
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
            })
        } else {
            None
        }
    }

    fn diff_and_split(&self, other: &Cuboid) -> Vec<Cuboid> {
        match self.intersects(other) {
            Some(diff) => {
                let mut splits = Vec::new();
                if diff.x_min > self.x_min {
                    splits.push(Cuboid::new(
                        self.x_min,
                        diff.x_min - 1,
                        self.y_min,
                        self.y_max,
                        self.z_min,
                        self.z_max,
                    ))
                }
                if diff.x_max < self.x_max {
                    splits.push(Cuboid::new(
                        diff.x_max + 1,
                        self.x_max,
                        self.y_min,
                        self.y_max,
                        self.z_min,
                        self.z_max,
                    ))
                }
                if diff.y_min > self.y_min {
                    splits.push(Cuboid::new(
                        diff.x_min,
                        diff.x_max,
                        self.y_min,
                        diff.y_min - 1,
                        self.z_min,
                        self.z_max,
                    ))
                }
                if diff.y_max < self.y_max {
                    splits.push(Cuboid::new(
                        diff.x_min,
                        diff.x_max,
                        diff.y_max + 1,
                        self.y_max,
                        self.z_min,
                        self.z_max,
                    ))
                }
                if diff.z_min > self.z_min {
                    splits.push(Cuboid::new(
                        diff.x_min,
                        diff.x_max,
                        diff.y_min,
                        diff.y_max,
                        self.z_min,
                        diff.z_min - 1,
                    ))
                }
                if diff.z_max < self.z_max {
                    splits.push(Cuboid::new(
                        diff.x_min,
                        diff.x_max,
                        diff.y_min,
                        diff.y_max,
                        diff.z_max + 1,
                        self.z_max,
                    ))
                }

                splits
            }
            None => Vec::from([self.clone()]),
        }
    }

    fn volume(&self) -> isize {
        (self.x_max - self.x_min + 1)
            * (self.y_max - self.y_min + 1)
            * (self.z_max - self.z_min + 1)
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-22-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 22.
pub fn run() {
    let contents = fs::read_to_string("res/day-22-input").expect("Failed to read file");
    let instructions = parse_input(&contents);
    let part_one_instructions = limit_instructions(&instructions, part_one_limit());
    println!(
        "There are {} cubes active in the initialisation procedure",
        volume_active(&part_one_instructions)
    );

    println!(
        "There are {} cubes active in the full reactor",
        volume_active(&instructions)
    );
}

fn parse_input(input: &String) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn merge_instruction(instruction: Instruction, cuboids: &Vec<Cuboid>) -> Vec<Cuboid> {
    let mut new_cuboids = Vec::new();

    cuboids
        .iter()
        .flat_map(|cuboid| cuboid.diff_and_split(&instruction.cuboid))
        .for_each(|cuboid| new_cuboids.push(cuboid));

    if instruction.is_on {
        new_cuboids.push(instruction.cuboid)
    }

    new_cuboids
}

fn volume_active(instructions: &Vec<Instruction>) -> isize {
    instructions
        .iter()
        .fold(Vec::new(), |acc, &inst| merge_instruction(inst, &acc))
        .iter()
        .map(|c| c.volume())
        .sum()
}

fn limit_instructions(instructions: &Vec<Instruction>, limit: Cuboid) -> Vec<Instruction> {
    instructions
        .iter()
        .flat_map(|inst| {
            limit.intersects(&inst.cuboid).map(|cuboid| Instruction {
                is_on: inst.is_on,
                cuboid,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day_22::{
        limit_instructions, merge_instruction, parse_input, part_one_limit, volume_active, Cuboid,
        Instruction,
    };

    fn sample_instructions() -> Vec<Instruction> {
        Vec::from([
            Instruction::new(true, 10, 12, 10, 12, 10, 12),
            Instruction::new(true, 11, 13, 11, 13, 11, 13),
            Instruction::new(false, 9, 11, 9, 11, 9, 11),
            Instruction::new(true, 10, 10, 10, 10, 10, 10),
        ])
    }

    #[test]
    fn can_parse() {
        let input = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"
            .to_string();

        let expected = sample_instructions();

        let actual = parse_input(&input);

        assert_eq!(actual.len(), expected.len());
        actual
            .iter()
            .zip(expected)
            .for_each(|(&act, exp)| assert_eq!(act, exp));
    }

    #[test]
    fn can_intersect() {
        let cuboids: Vec<Cuboid> = sample_instructions().iter().map(|i| i.cuboid).collect();

        assert_eq!(cuboids[0].intersects(&cuboids[3]), Some(cuboids[3]));
        assert_eq!(
            cuboids[0].intersects(&cuboids[1]),
            Some(Cuboid::new(11, 12, 11, 12, 11, 12))
        );
        assert_eq!(cuboids[1].intersects(&cuboids[3]), None);
    }

    #[test]
    fn can_diff_and_split() {
        let cuboids: Vec<Cuboid> = sample_instructions().iter().map(|i| i.cuboid).collect();

        let expected1 = Vec::from([
            Cuboid::new(11, 12, 10, 12, 10, 12),
            Cuboid::new(10, 10, 11, 12, 10, 12),
            Cuboid::new(10, 10, 10, 10, 11, 12),
        ]);
        assert_eq!(cuboids[0].diff_and_split(&cuboids[3]), expected1);

        let expected2 = Vec::from([cuboids[1]]);
        assert_eq!(cuboids[1].diff_and_split(&cuboids[3]), expected2);

        let expected3 = Vec::from([
            Cuboid::new(9, 9, 9, 11, 9, 11),
            Cuboid::new(11, 11, 9, 11, 9, 11),
            Cuboid::new(10, 10, 9, 9, 9, 11),
            Cuboid::new(10, 10, 11, 11, 9, 11),
            Cuboid::new(10, 10, 10, 10, 9, 9),
            Cuboid::new(10, 10, 10, 10, 11, 11),
        ]);
        assert_eq!(cuboids[2].diff_and_split(&cuboids[3]), expected3);
        assert_eq!(cuboids[0].diff_and_split(&cuboids[0]), Vec::new());
    }

    #[test]
    fn can_merge_instruction() {
        let instructions = sample_instructions();

        let pass1 = merge_instruction(instructions[0], &Vec::new());
        assert_eq!(pass1, Vec::from([instructions[0].cuboid]));

        let pass2 = merge_instruction(instructions[1], &pass1);
        let expected2 = Vec::from([
            Cuboid::new(10, 10, 10, 12, 10, 12),
            Cuboid::new(11, 12, 10, 10, 10, 12),
            Cuboid::new(11, 12, 11, 12, 10, 10),
            Cuboid::new(11, 13, 11, 13, 11, 13),
        ]);
        assert_eq!(pass2, expected2);

        assert_eq!(
            merge_instruction(instructions[2], &Vec::from([instructions[3].cuboid])),
            Vec::new()
        );
    }

    #[test]
    fn can_calc_volume() {
        let volumes: Vec<isize> = sample_instructions()
            .iter()
            .map(|i| i.cuboid.volume())
            .collect();
        assert_eq!(volumes, Vec::from([27, 27, 27, 1]));
        assert_eq!(Cuboid::new(10, 10, 10, 12, 10, 12).volume(), 9);
        assert_eq!(Cuboid::new(10, 10, 11, 12, 9, 12).volume(), 8);
    }

    #[test]
    fn can_sum_active_volumes() {
        assert_eq!(volume_active(&sample_instructions()), 39);

        let large_input = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507"
            .to_string();

        assert_eq!(volume_active(&parse_input(&large_input)), 2758514936282235);
    }

    #[test]
    fn can_limit() {
        assert_eq!(
            limit_instructions(&sample_instructions(), Cuboid::new(10, 10, 10, 10, 10, 10)),
            Vec::from([
                Instruction::new(true, 10, 10, 10, 10, 10, 10),
                Instruction::new(false, 10, 10, 10, 10, 10, 10),
                Instruction::new(true, 10, 10, 10, 10, 10, 10),
            ])
        )
    }

    #[test]
    fn can_sum_active_volumes_with_limit() {
        let input = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"
            .to_string();

        let instructions = limit_instructions(&parse_input(&input), part_one_limit());

        assert_eq!(volume_active(&instructions), 590784)
    }
}
