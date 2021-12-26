//! This is my solution for [Advent of Code - Day 24 - _Arithmetic Logic Unit_](https://adventofcode.com/2021/day/24)
//!
//! Today was a challenge of interpreting an assembly language adjacent program and working out the underlying logic
//! about which inputs it accepts or rejects. Looking through the puzzle data it seemed like a lot of statements were
//! no-ops so I first wondered if the puzzle was just to factor out the redundant commands and brute0force it, but after
//! a bit of trying to write a parser that would reduce the program to just the key bits in calculating the final `z`
//! value. This was a bit of a dead end, but did clue me into the repeating nature of the input. The no-op
//! operations we're repeated later on, but in the later context they did do something. Also I had written [parse_input]
//! and [`Instruction::from`], and [`Param`] and [`OpType`] to have a structured representation of the commands.
//!
//! At this point I was working things out in a [rather disorganised spreadsheet](https://docs.google.com/spreadsheets/d/1EvNOOa-1rTDfxe4yj2pe-3x6HPskKhz1nLXUFkL9M64/edit)
//! Sheet 1 is me stepping through the logic. Copy of sheet 1 shows the program split into its repeating sections.
//!
//! ![Working out spreadsheet](../../assets/img/day-24.png "A screenshot of my working out spread sheet showing the
//! program split into 18 row sections laid out next to each other. Underneath there is working showing variables
//! being added/removed from the stack represented by `z`")
//!
//! The MONAD program is a loop that repeats the same 18 lines, with a few variable values. Here is a walk through of
//! my first 18 line section:
//! ```text
//!  1. inp w      |  Read the next input into `w`  
//!  2. mul x 0    |  Reset `x` to 0
//!  3. add x z    |  Copy z into x
//!  4. mod x 26   |  z is treated like a stack of base 26 numbers, this mod sets
//!                |  `x` to the top of the stack
//!  5. div z 1    |  The number here is always 1 or 26. `z / 1` is a no-op,
//!                |  `z / 26` pops the `prev` number from the top of the `z`
//!                |  stack.
//!  6. add x 13   |  The number here is variable per section, I'll call it `n`.
//!                |  This is adding `n` to `prev`.
//!  7. eql x w    |  `1` if `prev` + `n` == `inout` (`w` is the latest digit
//!                |  input.)
//!  8. eql x 0    |  This is inverting line 7 so `prev` + `n` != `input`
//!  9. mul y 0    |  Reset `y` to 0
//! 10. add y 25   |  Set `y` to 25
//! 11. mul y x    |  `x` is 1 if the modified previous stack value != `input`, 0
//!                |  otherwise. So `y` is now either `0` or `25` based on the
//!                |  same condition.
//! 12. add y 1    |  `y` is now either 1 or 26 based on the same condition.
//! 13. mul z y    |  If `y` is 1, then this is a no-op otherwise this is shifting
//!                |  0 onto the top of the `z` stack
//! 14. mul y 0    |  Reset `y` to 0 again
//! 15. add y w    |  Copy the input into `y`
//! 16. add y 15   |  The literal here varies, I'll call it `p`.
//! 17. mul y x    |  Same idea as line 11, if `prev` + `n` != `input` this is a
//!                |  no-op, if `prev` + `n` == `input` y is now 0.
//! 18. add z y    |  If `y` is still non-zero, update the 0 we pushed onto the  
//!                |  stack to be `input` + `p`.
//! ```
//!
//! So written as rust code this could be seen as:
//! ```rust
//! fn section(input: isize, z: &mut Vec<isize>, pop: bool, n: isize, p: isize) {
//!     let x = z.last().unwrap() + n;
//!
//!     if pop {z.pop()};
//!
//!     if input != x {
//!         z.push(input + p);
//!     };
//! }
//! ```
//! It is also worth noting that as `input` is in the range `1` - `9`. So for some of the inputs it is guaranteed
//! that they'll be pushed onto the stack regardless of input. I stepped through the whole 14 sections by hand and
//! worked out that for z to be 0 (i.e. the stack is empty) each time `prev + n` could equal the input, it had to
//! match as the guaranteed pop operations matched the count of the guaranteed pushes, so each time it was possible
//! not to push we had to take it, or numbers would be left on the stack. This gave me the criteria for the valid
//! numbers. I made some human errors stepping through the constraints so the number I worked out failed, but since
//! this is a coding problem I should eliminate that by coding the analysis instead. [`analyse_program`] does just
//! that. Working out the maximum valid model number (part one), and then part two (the minimum) was a minor
//! modification.
//!
//! Overall, whilst there is some satisfaction in having worked out what was going on, I was not a fan of today's
//! puzzle. The answer was in deduction, so examples that actually helped a solver would have given the game away.
//! This meant the usual plan of build some tests from the example as a guide doesn't apply and makes the whole
//! experience more frustrating when it doesn't work (as it didn't for me when stepping through by hand). The only
//! feedback is that your answer is wrong, but you also can't go looking for hints as to why, as that gives the whole
//! game away.

use std::fs;

use crate::day_24::Instruction::{Inp, Op};
use crate::day_24::OpType::{Add, Div, Eql, Mod, Mul};
use crate::day_24::Param::{Lit, W, X, Y, Z};

/// Represents a operation's parameter(s) as either one of the four memory addresses or a literal number
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Param {
    W,
    X,
    Y,
    Z,
    Lit(isize),
}

impl From<&str> for Param {
    /// If the string parses as a number treat it as a literal, otherwise match it to a memory address or
    /// panic if it's not valid.
    fn from(s: &str) -> Self {
        if let Ok(num) = s.parse() {
            Lit(num)
        } else {
            match s {
                "w" => W,
                "x" => X,
                "y" => Y,
                "z" => Z,
                _ => panic!("invalid param {}", s),
            }
        }
    }
}

/// Whilst there are six instructions the `Inp` is different enough from the others that it is easier to split it out.
/// This then encodes the type of the remaining five op codes.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum OpType {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

/// Encode each line as either a read from input, or an operation
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Instruction {
    Inp(Param),
    Op(OpType, Param, Param),
}

impl From<&str> for Instruction {
    /// Parses a line of the input
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "inp" => Inp(Param::from(parts[1])),
            "add" => Op(Add, Param::from(parts[1]), Param::from(parts[2])),
            "mul" => Op(Mul, Param::from(parts[1]), Param::from(parts[2])),
            "div" => Op(Div, Param::from(parts[1]), Param::from(parts[2])),
            "mod" => Op(Mod, Param::from(parts[1]), Param::from(parts[2])),
            "eql" => Op(Eql, Param::from(parts[1]), Param::from(parts[2])),
            _ => panic!("invalid op: {}", s),
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-24-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 24.
pub fn run() {
    let contents = fs::read_to_string("res/day-24-input").expect("Failed to read file");
    let program: Vec<Instruction> = parse_input(&contents);
    let (min, max) = analyse_program(program);
    println!("The maximum model number is {}.", max);
    println!("The minimum model number is {}.", min);
}

/// Parse each line of the puzzle input program return with [`Instruction::from`], return the program as a list fo
/// instructions.
fn parse_input(input: &String) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

/// First split the program into its 18-line sections. For each extract the three variables. Tracking what `input +
/// p` values are on the stack, and where it is possible to avoid pushing to the stack, storing that as a condition.
/// Then iterate through these conditions working out where parts of the input are constrained by them and updating the
/// minimum and maximum numbers as appropriate. Finally return this minimum (part two) and maximum (part one).
fn analyse_program(program: Vec<Instruction>) -> (isize, isize) {
    // track the guaranteed push and pop operations
    let mut stack: Vec<(usize, isize)> = Vec::new();
    // track the conditions that prevent pushing to the stack
    let mut conditions: Vec<(usize, usize, isize)> = Vec::new();
    let chunks: Vec<Vec<Instruction>> = program.chunks(18).map(|chunk| chunk.to_vec()).collect();

    chunks.iter().enumerate().for_each(|(i, chunk)| {
        // peek at the top of the stack and account for it being empty for the first chunk.
        let &(prev_key, prev_p) = stack.last().unwrap_or(&(0, 0));

        // Line 5 (chunk lines are 0 indexed) is either 1 or 26. If it's 26 this causes a pop from the stack.
        if let Op(Div, Z, Lit(div)) = chunk[4] {
            if div == 26 {
                stack.pop();
            }
        }

        // Line 6 encodes `n`
        if let Op(Add, X, Lit(n)) = chunk[5] {
            // The previous input plus the previous `p` plus the current `n` must equal the current input to prevent
            // pushing to the stack. Since prev input must be at least 1, if n + prev_p > 8 then input must be > 9,
            // which is not possible. There is also a lower bound but that doesn't occur in the puzzle program.
            if n + prev_p <= 8 {
                // We have to prevent all unnecessary pushes, so record the condition that will prevent this push
                conditions.push((i, prev_key, n + prev_p));
            } else if let Op(Add, Y, Lit(p)) = chunk[15] {
                // Otherwise record that this `input + p` must be pushed to the top of the stack
                stack.push((i, p));
            }
        }
    });

    // Without conditions the min is 11111111111111 and the max is 99999999999999. Use these as starting values...
    let mut min = [9; 14];
    let mut max = [1; 14];
    // then loop through the conditions applying their constraints, which are in the form `input_a` == `input_b` + `v`
    for (a, b, v) in conditions {
        // b - something == a so b can be as high as possible (9) a can be as low as possible (1) and a can only go
        // up to `9 - mod(v)`, and b can only go down to `9 + mod(v)`
        if v < 0 {
            max[a] = 9 + v;
            max[b] = 9;

            min[b] = 1 - v;
            min[a] = 1;
        }
        // otherwise v is positive and it works the other way round.
        else {
            max[b] = 9 - v;
            max[a] = 9;

            min[a] = 1 + v;
            min[b] = 1;
        }
    }

    // convert the calculated arrays of digits into numbers and return the min/max pair.
    return (
        min.iter().fold(0, |acc, &v| (acc * 10) + v),
        max.iter().fold(0, |acc, &v| (acc * 10) + v),
    );
}

#[cfg(test)]
mod tests {
    use crate::day_24::parse_input;
    use crate::day_24::Instruction::{Inp, Op};
    use crate::day_24::OpType::{Eql, Mul};
    use crate::day_24::Param::{Lit, X, Z};

    #[test]
    fn can_parse() {
        assert_eq!(
            parse_input(&"inp x\nmul x -1".to_string()),
            Vec::from([Inp(X), Op(Mul, X, Lit(-1))])
        );
        assert_eq!(
            parse_input(&"inp z\ninp x\nmul z 3\neql z x".to_string()),
            Vec::from([Inp(Z), Inp(X), Op(Mul, Z, Lit(3)), Op(Eql, Z, X)])
        )
    }
}
