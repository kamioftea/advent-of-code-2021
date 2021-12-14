//! This is my solution for [Advent of Code - Day 14 - _Extended Polymerization_](https://adventofcode.com/2021/day/14)
//!
//!

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-14-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 14.
pub fn run() {
    let contents = fs::read_to_string("res/day-14-input").expect("Failed to read file");
    let (seed, mapping) = parse_input(&contents);

    let polymer = iterate(&seed, 10, &mapping);
    let (_, result) = summarise(&polymer);
    println!("The max - min counts after 10 cycles = {}.", result);

    let polymer2 = iterate(&polymer, 30, &mapping);
    let (_, result2) = summarise(&polymer2);
    println!("The max - min counts = {} after 40 cycles.", result2)
}

fn parse_input(
    input: &String,
) -> (
    HashMap<(char, char), usize>,
    HashMap<(char, char), Vec<(char, char)>>,
) {
    let mut lines = input.lines();
    let seed = into_pair_counts(&lines.next().expect("Empty input").chars().collect());
    // skip blank
    lines.next();
    let mapping: HashMap<(char, char), Vec<(char, char)>> = lines
        .flat_map(|line| line.split_once(" -> "))
        .flat_map(|(pair, insert)| {
            let mut pair_chars = pair.chars();
            pair_chars
                .next()
                .zip(pair_chars.next())
                .zip(insert.chars().next())
        })
        .map(|(pair, c)| (pair, vec![(pair.0, c), (c, pair.1)]))
        .collect();

    (seed, mapping)
}

fn intersperse(
    polymer: &HashMap<(char, char), usize>,
    mapping: &HashMap<(char, char), Vec<(char, char)>>,
) -> HashMap<(char, char), usize> {
    let mut new = HashMap::new();
    for (pair, count) in polymer {
        if let Some(pairs) = mapping.get(&pair) {
            pairs.iter().for_each(|p| {
                new.insert(*p, new.get(p).unwrap_or(&0) + count);
            })
        } else {
            new.insert(*pair, new.get(pair).unwrap_or(&0) + count);
        };
    }

    new
}

fn into_pair_counts(polymer: &Vec<char>) -> HashMap<(char, char), usize> {
    polymer
        .windows(2)
        .map(|window| (window[0], window[1]))
        .counts()
}

fn polymer_length(polymer: &HashMap<(char, char), usize>) -> usize {
    polymer.values().sum::<usize>() + 1
}

fn iterate(
    seed: &HashMap<(char, char), usize>,
    cycles: usize,
    mapping: &HashMap<(char, char), Vec<(char, char)>>,
) -> HashMap<(char, char), usize> {
    if cycles == 0 {
        return seed.clone();
    }

    iterate(&intersperse(seed, mapping), cycles - 1, mapping)
}

fn summarise(polymer: &HashMap<(char, char), usize>) -> (HashMap<char, usize>, usize) {
    let starts: HashMap<char, usize> = polymer
        .iter()
        .into_grouping_map_by(|((a, _), _)| a)
        .fold(0, |acc, _, (_, &val)| acc + val)
        .iter()
        .map(|(&&k, &v)| (k, v))
        .collect();
    let ends: HashMap<char, usize> = polymer
        .iter()
        .into_grouping_map_by(|((_, b), _)| b)
        .fold(0, |acc, _, (_, &val)| acc + val)
        .iter()
        .map(|(&&k, &v)| (k, v))
        .collect();

    let counts: HashMap<char, usize> = starts
        .iter()
        .map(|(&chr, &count)| (chr, *ends.get(&chr).unwrap_or(&0).max(&count)))
        .collect();

    let (&min, &max) = counts
        .values()
        .minmax()
        .into_option()
        .expect("Not enough chars");

    (counts, max - min)
}

#[cfg(test)]
mod tests {
    use crate::day_14::{
        intersperse, into_pair_counts, iterate, parse_input, polymer_length, summarise,
    };
    use std::collections::HashMap;

    fn sample_input() -> String {
        "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
            .to_string()
    }

    #[test]
    fn can_parse() {
        let (seed, mapping) = parse_input(&sample_input());
        assert_eq!(
            seed,
            HashMap::from([(('N', 'N'), 1), (('N', 'C'), 1), (('C', 'B'), 1),])
        );
        assert_eq!(
            mapping,
            HashMap::from([
                (('C', 'H'), vec![('C', 'B'), ('B', 'H')]),
                (('H', 'H'), vec![('H', 'N'), ('N', 'H')]),
                (('C', 'B'), vec![('C', 'H'), ('H', 'B')]),
                (('N', 'H'), vec![('N', 'C'), ('C', 'H')]),
                (('H', 'B'), vec![('H', 'C'), ('C', 'B')]),
                (('H', 'C'), vec![('H', 'B'), ('B', 'C')]),
                (('H', 'N'), vec![('H', 'C'), ('C', 'N')]),
                (('N', 'N'), vec![('N', 'C'), ('C', 'N')]),
                (('B', 'H'), vec![('B', 'H'), ('H', 'H')]),
                (('N', 'C'), vec![('N', 'B'), ('B', 'C')]),
                (('N', 'B'), vec![('N', 'B'), ('B', 'B')]),
                (('B', 'N'), vec![('B', 'B'), ('B', 'N')]),
                (('B', 'B'), vec![('B', 'N'), ('N', 'B')]),
                (('B', 'C'), vec![('B', 'B'), ('B', 'C')]),
                (('C', 'C'), vec![('C', 'N'), ('N', 'C')]),
                (('C', 'N'), vec![('C', 'C'), ('C', 'N')]),
            ])
        );
    }

    #[test]
    fn can_intersperse() {
        let (seed_counts, mapping) = parse_input(&sample_input());

        let pass_1 = intersperse(&seed_counts, &mapping);
        let pass_2 = intersperse(&pass_1, &mapping);
        let pass_3 = intersperse(&pass_2, &mapping);
        let pass_4 = intersperse(&pass_3, &mapping);

        let expected_1 = into_pair_counts(&"NCNBCHB".chars().collect());
        let expected_2 = into_pair_counts(&"NBCCNBBBCBHCB".chars().collect());
        let expected_3 = into_pair_counts(&"NBBBCNCCNBBNBNBBCHBHHBCHB".chars().collect());
        let expected_4 = into_pair_counts(
            &"NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
                .chars()
                .collect(),
        );

        assert_eq!(pass_1, expected_1);
        assert_eq!(pass_2, expected_2);
        assert_eq!(pass_3, expected_3);
        assert_eq!(pass_4, expected_4);
    }

    #[test]
    fn can_iterate() {
        let (seed, mapping) = parse_input(&sample_input());

        assert_eq!(polymer_length(&iterate(&seed, 5, &mapping)), 97);
        assert_eq!(polymer_length(&iterate(&seed, 10, &mapping)), 3073);
    }

    #[test]
    fn can_summarise() {
        let (seed, mapping) = parse_input(&sample_input());
        let polymer = iterate(&seed, 10, &mapping);
        let summary = summarise(&polymer);
        assert_eq!(
            summary,
            (
                HashMap::from([('B', 1749), ('C', 298), ('H', 161), ('N', 865)]),
                1588
            )
        );

        let polymer2 = iterate(&polymer, 30, &mapping);
        let (counts, result) = summarise(&polymer2);
        assert_eq!(counts.get(&'B'), Some(&2192039569602));
        assert_eq!(counts.get(&'H'), Some(&3849876073));
        assert_eq!(result, 2188189693529);
    }
}
