//! This is my solution for [Advent of Code - Day 14 - _Extended Polymerization_](https://adventofcode.com/2021/day/14)
//!
//! By the end of today I was thinking it was a lot like [`crate::day_6`], but I missed how
//! exponential it was when first reading, so implemented the naive versions for part one, but that
//! did not complete before it ran out of memory for part two. I ended up noting that each pair
//! would become two new pairs each step (or stay as the same pair if there was no insertion mapping
//! for that pair). This meant I could just track the counts of each pair in a given iteration, and
//! work out the next step by walking through the current pair counts and adding its count to each
//! of the pairs it maps to in the counts map for the next iteration.
//!
//! The types are pretty convoluted today, enough so that I aliased the Polymer as map of pairs ->
//! counts, and pair mapping of pair -> pairs when iterated into [`Polymer`] and [`PairMap`]
//! respectively. That said, whilst the types were complex, they were a great guide for what
//! transformations I needed to do at each step, and once I'd satisfied the type checker the
//! solutions just worked.
//!
//! There's quite a lot of moving parts today, mostly involved in building the internal
//! representation from the input, and converting the result into the required output.
//! [`parse_input`] takes the file, transforms the seed polymer into the required map of pair counts
//! using [`into_pair_counts`], and then takes the remaining lines and creates the PairMap. i.e. a
//! mapping of `AB -> C` becomes `('A', 'B') => [('A', 'C'), ('C', 'B')]` which means when there is
//! a pair `AB` the next iteration will instead have two pairs, `AC` and `CB`. [`intersperse`]
//! handles performing a single insertion cycle, and [`iterate`] recursively calls [`intersperse`]
//! for the required number of cycles. Finally [`summarise`] works out the counts of each of the
//! characters. With the current implementation we need to take the counts of both parts of each
//! pair to account for the first and last characters. This in itself involves some complex type
//! munging, so has been extracted to [`into_count_by`]. If I was building this again I'd consider
//! making a struct to hold the polymer, including caching the final character from the seed. This
//! would allow just counting the first character in each pair and adding 1 to the count that
//! matches the final character. As it is, this works and is quick enough that it's not worth the
//! effort.

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

type Polymer = HashMap<(char, char), usize>;
type PairMap = HashMap<(char, char), Vec<(char, char)>>;

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
    let length = polymer_length(&polymer2);
    println!(
        "The max - min counts after 40 cycles = {}, total length {}.",
        result2, length
    )
}

/// Split a list of characters into the counts of all the consecutive pairs that exist. The hard
/// work is delegated to library functions [`slice::windows`] to give an iterator of the pairs
/// and [`Itertools::counts`] to reduce that to the required map.
fn into_pair_counts(polymer_chars: &Vec<char>) -> Polymer {
    polymer_chars
        .windows(2)
        .map(|window| (window[0], window[1]))
        .counts()
}

/// The types required to make today's solution work are pretty complex, so there is quite a lot of
/// work here to take a relatively simple input format into the complex format that makes the logic
/// efficient. A bunch of the tests need to convert intermediate polymer string representations into
/// the map of pair counts used internally, so this is delegated to [`into_pair_counts`].
fn parse_input(input: &String) -> (Polymer, PairMap) {
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

/// The name is a legacy from the naive solution where this was mapping each pair to the new pairs
/// and building the full polymer in order which failed as the final polymer had ~21 trillion
/// characters. This takes all the existing pairs and adds their counts to the pair(s) they map to
/// when the mapped character is inserted.
///
/// Consider the seed `BABABA`, in the internal representation this becomes `BA => 3, AB => 2`.
/// With the mapping `AB -> A` and `BA -> A` this would become `BAAABAAABAA` or `BA => 3, AA => 5,
/// AB => 2`. Because each pair mapping is independent, we can view the mapping as `AB => AA, AB`,
/// `BA => BA, AA`. So applying [`intersperse`] to the `BA => 3, AB => 2` polymer:
/// * `BA => 3` becomes `BA => 3, AA => 3`.
/// * The new map is empty so both are inserted with a count of 3.
/// * `AB => 2` becomes `AA => 2, AB => 2`.
/// * `AA` already exists with a count of 3, so these two are added to give `AA =>5`.
/// * `AB` isn't in the map, so it is inserted with a count of 2.
/// * This gives the expected `BA => 3, AA => 5, AB => 2` Polymer.
fn intersperse(polymer: &Polymer, mapping: &PairMap) -> Polymer {
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

// Utility for counting the length of the polymer. Since they overlap, the two chars per pair and
// two pairs per char cancel out, but we need to add one to cover that the first and last character
// are each only in one pair.
fn polymer_length(polymer: &Polymer) -> usize {
    polymer.values().sum::<usize>() + 1
}

/// Recursively apply [`intersperse`] the required number of times
fn iterate(seed: &Polymer, cycles: usize, mapping: &PairMap) -> Polymer {
    if cycles == 0 {
        return seed.clone();
    }

    iterate(&intersperse(seed, mapping), cycles - 1, mapping)
}

/// Reduce the pair mapping into a count of characters. This needs to be called twice once for each
/// element in the pair, to account for the first and last character that are each only in one pair.
/// The mapping parameter is to capture this difference, and maps a pair count entry from the
/// Polymer into the character this invocation cares about
fn into_count_by(
    polymer: &Polymer,
    mapping: for<'a> fn(&'a (&(char, char), &usize)) -> char,
) -> HashMap<char, usize> {
    polymer
        .iter()
        // group by the mapping - the values are now `Vec<((char, char), usize)>
        .into_grouping_map_by(mapping)
        // sum just the counts
        .fold(0, |acc, _, (_, &val)| acc + val)
        // map the resulting HashMap to fix the references
        .iter()
        .map(|(&k, &v)| (k, v))
        .collect()
}

/// This is responsible for converting the internal representation of a polymer into the data needed
/// to provide the puzzle solution. It also returns the intermediary hashmap so that this can be
/// verified in tests against the example provided in the specification.
fn summarise(polymer: &Polymer) -> (HashMap<char, usize>, usize) {
    // Get the counts bases on the first ...
    let starts: HashMap<char, usize> = into_count_by(polymer, |((a, _), _)| *a);
    // ... and second character in the pair
    let ends: HashMap<char, usize> = into_count_by(polymer, |((_, b), _)| *b);

    // For each character take the maximum count from these two maps. The count for the starting
    // character is one higher as it only appears in the start of the one pair it's in, and vice
    // versa for the final character.
    let counts: HashMap<char, usize> = starts
        .iter()
        .map(|(&chr, &count)| (chr, *ends.get(&chr).unwrap_or(&0).max(&count)))
        .collect();

    // For obtaining the min and max character counts the character doesn't matter so can just use
    // [`Itertools::minmax`] directly on the values, without the more complex mapping
    // needed in [`into_count_by`].
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