//! This is my solution for [Advent of Code - Day 23 - _Amphipod_](https://adventofcode.com/2021/day/23)
//!
//! This one I really struggled with. Almost entirely due to me misreading the puzzle description, and missing the
//! restrictions on Amphipod movement that vastly restrict the graph of moves. It took eventually despairing, looking
//! up others' solutions, seeing a bunch of things that seemed weird, and going back to work out why. By this point
//! I'd rewritten my state representation as an integer of upto 23 3-bit sections from which I could unpack the state,
//! so that I could more efficiently store the states in a Binary Heap. I was able to shoe-horn in the restrictions to
//! my existing code, but that journey has left it pretty unreadable. I have left it as is, and will try to make it
//! clearer what is going on with some comments. It is worth noting that my initial, less restricted solution still
//! worked for part one, though it did take about 6 minutes to complete.
//!
//! [`Burrow`] is the previously mentioned state-as-integer, which contains the state, and the length for ease of
//! determining how many leading 0's are significant. [`Burrow::from`] is used for creating burrows from a string in
//! tests. [`Burrow::fmt`] turns the integer into a string of letters so debugging is possible. [`Burrow::get_at`] does
//! some bit-manipulation to get the Amphipod type, if any, at that position. [`Burrow::set_at`] uses more bit tricks to
//! mutate the state of one of the positions and [Burrow::swap] uses these to swap the state between two positions, used
//! to move Amphipods. [`State`] wraps a [`Burrow`] with a cost to enable using Dijkstra's algorithm to solve the puzzle
//! with a graph search. [`crate::day_15`] has a cleaner implementation of this. Note [`Burrow::cmp`] is manually
//! implemented to reverse the ordering, so that Rust's default [`BinaryHeap`], which is a max-heap, works as the
//! required min-heap instead
//!
//! [`parse_letter`] turns an `.`, `A`, `B`, `C`, or `D` into a number 0-4 to represent the possible state for each
//! cell. [`parse_input`] parses the ascii diagram of the burrow, mostly by ignoring every thing that isn't `A`, `B`,
//! `C`, or `D`. [`build_goal`] builds the burrow representing the expected final state of the burrow for a given depth
//! of side-tunnels. [`build_states`] returns a list of possible states, and the cost to move there from the given
//! state. This is where the worst of the mess is, as it relies on a lot of number manipulation tricks to turn the flat
//! 15/23 cell list of cells into something that represents the more complex burrow structure. [`find_shortest_path`] is
//! just implementing Dijkstra's Algorithm and is very similar to [`crate::day_15`]'s version, but with a different
//! adjacency/cost implementation. Finally [`expand_burrow`] handles turning the input for part one into the input for
//! part two.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Display, Formatter};
use std::fs;

/// The cost to move each type of Amphipod in order A-D
const COSTS: [usize; 4] = [1, 10, 100, 1000];

/// Represents a burrow as an integer that can be used as a list of 3-bit sections. 0-6 are the 7 cells in the hallway
/// where a Amphipod can stop, the cells adjacent to each side tunnel are not represented here, and instead handled by
/// [`build_states`] accounting for them when calculating costs. The remaining cells represent the side-tunnels, reading
/// like a book. You can walk down a tunnel by staring at indices 7, 8, 9 or 10, and increasing by 4 each step. It is
/// possible to represent a burrow of up to depth 8 in the u128 used.
///
/// The cells themselves use the numbers 0-4 to represent the types, 5 - 7 are unused:
/// - 0 - Empty
/// - 1 - Amber
/// - 2 - Bronze
/// - 3 - Copper
/// - 4 - Desert
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone)]
struct Burrow {
    /// The number of cells in the grid. 15 for depth 2 (part one) and 23 for depth 4 (part two)
    len: usize,
    /// Each cell maps to 3 bits in this integer
    positions: u128,
}

impl From<&String> for Burrow {
    /// Parses a string in the format `.......BCBDDCBADBACADCA` as a Burrow. Used mostly for testing. See [`parse_input]
    /// for parsing the actual puzzle input
    fn from(str: &String) -> Self {
        let (len, positions) = str
            .chars()
            .flat_map(parse_letter)
            .fold((0, 0), |(len, pos), num| (len + 1, (pos << 3) + num));
        Burrow { len, positions }
    }
}

impl Display for Burrow {
    /// Output a representation of the burrow in `.......BCBDDCBADBACADCA` format
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut chars: String = "".to_string();
        for i in 0..self.len {
            chars = chars
                + (match self.get_at(i) {
                    0 => ".",
                    1 => "A",
                    2 => "B",
                    3 => "C",
                    4 => "D",
                    _ => "?",
                })
        }

        write!(f, "{}", chars)
    }
}

impl Debug for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Burrow {
    /// Return the number representing the state at a given cell offset.
    fn get_at(&self, pos: usize) -> u128 {
        if pos >= self.len {
            panic!("Burrow overflow")
        }
        // shift later cells off the end, then & with `111` to mask earlier cell values
        (self.positions >> ((self.len - pos - 1) * 3)) & 7
    }

    /// update the positions integer with the `value` in `pos`.
    ///
    /// Set cell offset 1 in a 4-cell burrow to 4, original value = 4321 (`100 011 010 001`):
    /// ```text
    /// let offset = 6; // i.e. the first bit after the 3 bits of the cell.'
    /// let bits = (1 << self.len /*4*/ * 3)       // 1 000 000 000 000
    ///            -1;                             //   111 111 111 111
    ///
    /// let hole = 7u128 << offset;                //   000 111 000 000
    ///
    /// let mask = hole ^ bits;                    //   111 000 111 111
    ///
    /// let zeroed = self.positions & mask;        //   100 011 010 001
    ///                                            // & 111 000 111 111
    ///                                            //   ---------------
    ///                                            //   100 000 010 001
    ///
    /// self.positions = zeroed | (val << offset); //   100 000 010 001
    ///                                            // | 000 100 000 000
    ///                                            //   ---------------
    ///                                            //   100 100 010 001  
    ///                                            // = 4421
    /// ```
    fn set_at(&mut self, pos: usize, val: u128) {
        // the offset of the cell to update
        let offset = ((self.len - pos - 1) * 3) as u128;
        // fill a bitmap matching the length of the positions with `1`s
        let bits = (1 << self.len * 3) - 1;
        // A mask that just has the three bits of the cell set
        let hole = 7u128 << offset;
        // Invert that so all bits except the cell we're updating are set
        let mask = hole ^ bits;
        // & the current value with the mask to set all the bits in the cell to 0, leaving others as is.
        let zeroed = self.positions & mask;
        // finally or with the desired new value sifted into the cell's position .
        self.positions = zeroed | (val << offset);
    }

    /// Return a new burrow with the values at a and b swapped
    fn swap(&self, a: usize, b: usize) -> Burrow {
        let mut burrow = self.clone();
        burrow.set_at(a, self.get_at(b));
        burrow.set_at(b, self.get_at(a));
        burrow
    }
}

/// Wrapper for a Burrow state with the cost to reach that state. Implements [`Ord`] in reverse order so that we can use
/// Rust's built in max-[`BinaryHeap`] as a min-heap.
#[derive(Eq, PartialEq, Debug)]
struct State {
    /// The cost to reach this burrow state
    cost: usize,
    /// The burrow state
    burrow: Burrow,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.burrow.cmp(&other.burrow))
    }
}

impl State {
    fn new(cost: usize, burrow: Burrow) -> Self {
        State { cost, burrow }
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-23-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 23.
pub fn run() {
    let contents = fs::read_to_string("res/day-23-input").expect("Failed to read file");
    let burrow = parse_input(&contents);
    let result = find_shortest_path(&burrow).unwrap();
    println!("Lowest energy for small burrow is {}", result);

    let expanded_burrow = expand_burrow(&burrow);
    let expanded_result = find_shortest_path(&expanded_burrow).unwrap();
    println!("Lowest energy for expanded burrow is {}", expanded_result);
}

/// Turn a letter in the ascii-art into the number we use to represent it internally
///
/// - 0 - Empty
/// - 1 - Amber
/// - 2 - Bronze
/// - 3 - Copper
/// - 4 - Desert
fn parse_letter(letter: char) -> Option<u128> {
    match letter {
        '.' => Some(0u128),
        'A' => Some(1u128),
        'B' => Some(2u128),
        'C' => Some(3u128),
        'D' => Some(4u128),
        _ => None,
    }
}

/// Parse the ascii-art diagram into the internal representation.
fn parse_input(input: &String) -> Burrow {
    let (len, positions) = input
        .lines()
        // the upper wall of `#` and the hallway ca be assumed to be empty
        .skip(2)
        // using flat_map means all non-relevant characters are filtered out ([`parse_letter`] returns None)
        .flat_map(|line| line.chars().flat_map(parse_letter))
        // start with a burrow of 7 cells, all `000` (the hallway) and shift each Amphipod from the right
        .fold((7, 0), |(len, pos), num| (len + 1, (pos << 3) + num));

    Burrow { len, positions }
}

/// Return a burrow that represents the target state for a given depth.
fn build_goal(depth: usize) -> Burrow {
    // hallway + four tunnels each of length `depth`
    let len = depth * 4 + 7;
    // ech row should be `1234` in order. Shift each cell on in turn
    let row = (1 << 9) + (2 << 6) + (3 << 3) + 4;
    // for each depth shift another full row onto the end
    let positions = (0..depth).fold(0, |acc, _| (acc << 12) + row);

    Burrow { len, positions }
}

/// This handles building the possible next states respecting the limits on Amphipod movement returning a list of the
/// possible states and the cost for each.
/// - For each hallway cell:
///     - If there is an Amphipod there walk towards its desired tunnel, aborting if there is a non-empty cell in the
///       way. Track the distance, `0 -> 1` or `6 -> 5` are 1 distance, all others are 2 to account for the
///       unrepresented cells the Amphipod can't stop in.
///     - Then walk down the  the tunnel, until a non-empty cell, or the bottom. Note the position of and distance
///       to the final empty cell. Continue to increment the distance, the first step is worth an extra 1 as the
///       Amphipod first steps into the cell adjacent to the tunnel that is not represented.
///     - Continue through any remaining cells, if any have an Amphipod that wants to be in a different tunnel, abort.
///     - If the move is valid, use [`Burrow::swap`] to copy the burrow with that move applied, and calculate the cost.
///       Add these to the output `Vec`.
/// - For each tunnel:
///     - Walk down it until you reach a non-empty cell.
///     - Starting at the cell left of the top of this tunnel, i.e. the first one the Amphipod can stop at, check if
///       the cell is empty, and, if so  use [`Burrow::swap`] to copy the burrow with that move applied, and calculate
///       the cost. Add these to the output `Vec`.
///     - Keep stepping leftwards until a non-empty cell, or the end of the hallway (`0`) is reached.
///     - Repeat for the cell to the right, stepping rightwards.
fn build_states(burrow: &Burrow) -> Vec<(usize, Burrow)> {
    let mut out = Vec::new();

    // start with the hallway, check each cell in turn
    'outer: for i in 0..7 {
        let curr = burrow.get_at(i);
        // if empty, nothing to move
        if curr == 0 {
            continue;
        }
        // Look up the cost based on the type (the costs array is 0 indexed, but Amber starts at 1
        let cost = COSTS[curr as usize - 1];
        // Does this Amphipod need to head left or right to reach its desired tunnel
        let delta: isize = if i <= curr as usize { 1 } else { -1 };
        // Aiming for the cell just to the left, or right of the tunnel entrance, depending on direction, as the
        // entrance itself can't be stopped at so isn't represented.
        let target = if i <= curr as usize { curr } else { curr + 1 };
        // track where we are horizontally
        let mut h_pos = i as usize;
        // Start at 1 to include the entrance to the tunnel in the distance
        let mut dist = 1;
        // walk towards the target - the middle steps cost more to cover passing the tunnel entrances
        while h_pos != target as usize {
            if [0, 6].contains(&h_pos) {
                dist += 1
            } else {
                dist += 2
            };
            h_pos = (h_pos as isize + delta) as usize;
            // check the path is clear, if not continue to the next cell
            if burrow.get_at(h_pos) != 0 {
                continue 'outer;
            }
        }
        // Now start moving down the tunnel, Because the type we have matches the tunnel we can use that to calculate
        // the offset of the first cell in that tunnel.
        let mut v_pos = curr as usize + 6;
        // We need to walk the whole tunnel to validate it but remember which was the final empty cell
        let mut final_pos = v_pos;
        while v_pos < burrow.len {
            if burrow.get_at(v_pos) == 0 {
                final_pos = v_pos;
                dist += 1
            }
            // All mismatched Amphipods need to leave before the correct ones will enter
            else if burrow.get_at(v_pos) != curr {
                continue 'outer;
            }
            // There are four tunnels so stepping in increments of 4 moves down this tunnel
            v_pos += 4;
        }
        // Invalid tunnels continue to the next cell explicitly. If this is reached it's a valid move - add it to the
        // output
        out.push((cost * dist, burrow.swap(i, final_pos)));
    }

    // Now check the four tunnels to see if an Amphipod can move out
    for i in 0..4 {
        // Skip the hallway and offset to the current tunnel
        let mut pos = 7 + i;
        // Two steps to tunnel entrance where the Amphipod can't stop, and the first cell it can stop at
        let mut dist = 2;
        // walk down the tunnel until we reach the bottom
        while pos < burrow.len {
            let curr = burrow.get_at(pos);
            // until a non-empty cell is found
            if burrow.get_at(pos) != 0 {
                // Look up the cost based on the type (the costs array is 0 indexed, but Amber starts at 1
                let cost = COSTS[curr as usize - 1];
                // first cell to the left of this tunnel's entrance
                let mut left_pos = i + 1;
                let mut left_dist = 0;
                // while the current cell is empty walk leftwards
                while burrow.get_at(left_pos) == 0 {
                    // add the new state and cost to the output
                    out.push((cost * (dist + left_dist), burrow.swap(pos, left_pos)));
                    // need to explicitly abort at the hallway end so as not to go to -1 which is invalid for `usize`
                    if left_pos == 0 {
                        break;
                    }
                    // it's just a jump to the left (1 cell)
                    left_pos -= 1;
                    // middle cells require crossing a tunnel entrance as well
                    left_dist += if left_pos == 0 { 1 } else { 2 };
                }
                // now do the same, but on the right
                let mut right_pos = i + 2;
                let mut right_dist = 0;
                // as the boundary is positive here we can do the check for hallway end in the loop condition
                while right_pos <= 6 && burrow.get_at(right_pos) == 0 {
                    // add the new state and cost to the output
                    out.push((cost * (dist + right_dist), burrow.swap(pos, right_pos)));
                    // ... and then a step to the right
                    right_pos += 1;
                    right_dist += if right_pos == 6 { 1 } else { 2 };
                }
                // having found and possibly moved an Amphipod, continue to the next tunnel
                break;
            }
            // There are four tunnels so stepping in increments of 4 moves down this tunnel, also track the extra
            // distance needed to leave the tunnel
            pos += 4;
            dist += 1
        }
    }

    out
}

/// Use Dijkstra's algorithm to represent the puzzle as a graph of states, and find the shortest path (i.e. lowest
/// total move energy) for the Amphipods to all reach their desired tunnel.
fn find_shortest_path(start: &Burrow) -> Option<usize> {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut dist: HashMap<u128, usize> = HashMap::new();

    let depth = (start.len - 7) / 4;
    let goal = build_goal(depth);

    dist.insert(start.positions, 0);
    heap.push(State::new(0, start.clone()));

    while let Some(State { cost, burrow }) = heap.pop() {
        if burrow == goal {
            return Some(cost);
        }

        if cost > *dist.get(&burrow.positions).unwrap_or(&usize::MAX) {
            continue;
        }

        for (energy, next_burrow) in build_states(&burrow) {
            let next_cost = cost + energy;
            let curr_cost = dist.get(&next_burrow.positions).unwrap_or(&usize::MAX);
            if next_cost < *curr_cost {
                heap.push(State::new(next_cost, next_burrow.clone()));
                dist.insert(next_burrow.positions, next_cost);
            }
        }
    }

    // if we exhaust the adjacent states without reaching a goal, there isn't a solution
    None
}

/// Add in the two extra lines that were hidden behind the fold for part two.
fn expand_burrow(burrow: &Burrow) -> Burrow {
    let mut as_str = format!("{}", burrow);
    as_str.insert_str(11, "DCBADBAC");
    Burrow::from(&as_str)
}

#[cfg(test)]
mod tests {
    use crate::day_23::{
        build_goal, build_states, expand_burrow, find_shortest_path, parse_input, Burrow,
    };
    use std::collections::HashSet;

    fn sample_start() -> Burrow {
        Burrow::from(&".......BCBDADCA".to_string())
    }

    #[test]
    fn can_parse() {
        let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"
            .to_string();

        let burrow = parse_input(&input);
        assert_eq!(burrow, sample_start());
        assert_eq!(format!("{}", burrow), ".......BCBDADCA".to_string())
    }

    #[test]
    fn can_swap() {
        let burrow = sample_start();
        let swapped = burrow.swap(0, 14);
        assert_eq!(format!("{}", swapped), "A......BCBDADC.".to_string())
    }

    #[test]
    fn can_build_goal() {
        assert_eq!(build_goal(2), Burrow::from(&".......ABCDABCD".to_string()));
        assert_eq!(
            build_goal(4),
            Burrow::from(&".......ABCDABCDABCDABCD".to_string())
        );
    }

    #[test]
    fn can_calc_next_state() {
        let actual = build_states(&sample_start());
        let expected = HashSet::from([
            (30, Burrow::from(&"B.......CBDADCA".to_string())),
            (20, Burrow::from(&".B......CBDADCA".to_string())),
            (20, Burrow::from(&"..B.....CBDADCA".to_string())),
            (40, Burrow::from(&"...B....CBDADCA".to_string())),
            (60, Burrow::from(&"....B...CBDADCA".to_string())),
            (80, Burrow::from(&".....B..CBDADCA".to_string())),
            (90, Burrow::from(&"......B.CBDADCA".to_string())),
            (500, Burrow::from(&"C......B.BDADCA".to_string())),
            (400, Burrow::from(&".C.....B.BDADCA".to_string())),
            (200, Burrow::from(&"..C....B.BDADCA".to_string())),
            (200, Burrow::from(&"...C...B.BDADCA".to_string())),
            (400, Burrow::from(&"....C..B.BDADCA".to_string())),
            (600, Burrow::from(&".....C.B.BDADCA".to_string())),
            (700, Burrow::from(&"......CB.BDADCA".to_string())),
            (70, Burrow::from(&"B......BC.DADCA".to_string())),
            (60, Burrow::from(&".B.....BC.DADCA".to_string())),
            (40, Burrow::from(&"..B....BC.DADCA".to_string())),
            (20, Burrow::from(&"...B...BC.DADCA".to_string())),
            (20, Burrow::from(&"....B..BC.DADCA".to_string())),
            (40, Burrow::from(&".....B.BC.DADCA".to_string())),
            (50, Burrow::from(&"......BBC.DADCA".to_string())),
            (9000, Burrow::from(&"D......BCB.ADCA".to_string())),
            (8000, Burrow::from(&".D.....BCB.ADCA".to_string())),
            (6000, Burrow::from(&"..D....BCB.ADCA".to_string())),
            (4000, Burrow::from(&"...D...BCB.ADCA".to_string())),
            (2000, Burrow::from(&"....D..BCB.ADCA".to_string())),
            (2000, Burrow::from(&".....D.BCB.ADCA".to_string())),
            (3000, Burrow::from(&"......DBCB.ADCA".to_string())),
        ]);

        for entry in &actual {
            assert!(expected.contains(entry))
        }
        assert_eq!(actual.len(), expected.len());

        let actual2 = build_states(&Burrow::from(&"....D.............B...C".to_string()));
        let expected2 = HashSet::from([
            (40, Burrow::from(&"....DB................C".to_string())),
            (50, Burrow::from(&"....D.B...............C".to_string())),
        ]);
        for entry in &actual2 {
            assert!(expected2.contains(entry))
        }
        assert_eq!(actual2.len(), expected2.len());
    }

    #[test]
    fn can_calc_shortest_path() {
        assert_eq!(
            find_shortest_path(&Burrow::from(&".A......BCDABCD".to_string())),
            Some(2)
        );
        assert_eq!(
            find_shortest_path(&Burrow::from(&".B.....A.CDABCD".to_string())),
            Some(40)
        );
        assert_eq!(
            find_shortest_path(&Burrow::from(&".C.....AB.DABCD".to_string())),
            Some(600)
        );
        assert_eq!(
            find_shortest_path(&Burrow::from(&".......BACDABCD".to_string())),
            Some(46)
        );
        assert_eq!(find_shortest_path(&sample_start()), Some(12521));

        assert_eq!(
            find_shortest_path(&expand_burrow(&sample_start())),
            Some(44169)
        );
    }

    #[test]
    fn can_expand_burrow() {
        assert_eq!(
            format!("{}", expand_burrow(&sample_start())),
            ".......BCBDDCBADBACADCA"
        )
    }
}
