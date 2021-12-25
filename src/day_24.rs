//! This is my solution for [Advent of Code - Day 24 - _Title_](https://adventofcode.com/2021/day/24)
//!
//!

use crate::day_24::Function::{Input, Literal, Operation};
use crate::day_24::Instruction::{Inp, Op};
use crate::day_24::OpType::{Add, Div, Eql, Mod, Mul};
use crate::day_24::Param::{Lit, W, X, Y, Z};
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Param {
    W,
    X,
    Y,
    Z,
    Lit(isize),
}

impl From<&str> for Param {
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

impl Param {
    fn as_function(&self, register: &Register) -> Function {
        match self {
            Lit(num) => Literal(*num),
            W => register.w.clone(),
            X => register.x.clone(),
            Y => register.y.clone(),
            Z => register.z.clone(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum OpType {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl OpType {
    fn apply(&self, a: isize, b: isize) -> isize {
        match self {
            Add => a + b,
            Mul => a * b,
            Div => a / b,
            Mod => a % b,
            Eql => (a == b) as isize,
        }
    }
    fn symbol(&self) -> &str {
        match self {
            Add => "+",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Eql => "==",
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Instruction {
    Inp(Param),
    Op(OpType, Param, Param),
}

impl From<&str> for Instruction {
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

#[derive(Eq, PartialEq, Debug, Clone)]
enum Function {
    Literal(isize),
    Input(usize),
    Operation(OpType, Box<Function>, Box<Function>),
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal(num) => write!(f, "{}", num),
            Input(idx) => write!(f, "i[{}]", idx),
            Operation(op_type, a, b) => write!(
                f,
                "({} {} {})",
                format!("{}", a),
                op_type.symbol(),
                format!("{}", b)
            ),
        }
    }
}

impl Function {
    fn is_input(&self) -> bool {
        match self {
            Input(_) => true,
            _ => false,
        }
    }

    fn can_equal_input(&self) -> bool {
        match self {
            Literal(n) => *n < 9 && *n > 1,
            Operation(Add, _, l) if l.literal().is_some() => l.literal().unwrap() < 8,
            Operation(Add, l, _) if l.literal().is_some() => l.literal().unwrap() < 8,
            _ => true,
        }
    }

    fn literal(&self) -> Option<isize> {
        return match self {
            Literal(n) => Some(*n),
            _ => None,
        };
    }

    fn operation(op_type: OpType, fn_a: Function, fn_b: Function) -> Function {
        Operation(op_type, Box::new(fn_a), Box::new(fn_b))
    }

    fn result(&self, inputs: &Vec<isize>) -> Option<isize> {
        match self {
            Literal(num) => Some(*num),
            Input(n) => inputs.get(*n).map(|&a| a),
            Operation(op_type, a, b) => a
                .result(inputs)
                .zip(b.result(inputs))
                .map(|(a, b)| op_type.apply(a, b)),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Register {
    input_pos: usize,
    w: Function,
    x: Function,
    y: Function,
    z: Function,
}

impl Register {
    fn new() -> Register {
        Register {
            input_pos: 0,
            w: Literal(0),
            x: Literal(0),
            y: Literal(0),
            z: Literal(0),
        }
    }

    fn with(&self, p: Param, fun: &Function, inc: bool) -> Register {
        Register {
            input_pos: self.input_pos + inc as usize,
            w: if p == W { fun.clone() } else { self.w.clone() },
            x: if p == X { fun.clone() } else { self.x.clone() },
            y: if p == Y { fun.clone() } else { self.y.clone() },
            z: if p == Z { fun.clone() } else { self.z.clone() },
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
    println!("The minimum model number is {}.", min);
    println!("The maximum model number is {}.", max);
}

fn parse_input(input: &String) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn analyse_program(program: Vec<Instruction>) -> (isize, isize) {
    let mut stack: Vec<(usize, isize)> = Vec::new();
    let mut conditions: Vec<(usize, usize, isize)> = Vec::new();
    let chunks: Vec<Vec<Instruction>> = program.chunks(18).map(|chunk| chunk.to_vec()).collect();

    chunks.iter().enumerate().for_each(|(i, chunk)| {
        let &(prev_key, prev_add) = stack.get(stack.len() - 1).unwrap_or(&(0, 0));

        println!("{:?}, {:?}, {:?}", chunk[4], chunk[5], chunk[15]);

        if let Op(Div, Z, Lit(v)) = chunk[4] {
            println!("Chunk {} check pop {}", i, v);
            if v == 26 {
                println!("Chunk {} pop", i);
                stack.pop();
            }
        }

        if let Op(Add, X, Lit(x_add)) = chunk[5] {
            println!("{} + {} + {}: {}", prev_key, prev_add, x_add, i);

            if x_add + prev_add <= 8 {
                conditions.push((prev_key, i, x_add + prev_add));
            } else if let Op(Add, Y, Lit(y_add)) = chunk[15] {
                stack.push((i, y_add));
            }
        }
    });

    println!("{:?}", stack);
    println!("{:?}", conditions);

    let mut min = [9; 14];
    let mut max = [1; 14];
    for (a, b, v) in conditions {
        if v < 0 {
            max[b] = 9 + v;
            max[a] = 9;

            min[a] = 1 - v;
            min[b] = 1;
        } else {
            max[a] = 9 - v;
            max[b] = 9;

            min[b] = 1 + v;
            min[a] = 1;
        }
    }

    return (
        min.iter().fold(0, |acc, &v| (acc * 10) + v),
        max.iter().fold(0, |acc, &v| (acc * 10) + v),
    );
}

#[cfg(test)]
mod tests {
    use crate::day_24::Function::{Input, Literal};
    use crate::day_24::Instruction::{Inp, Op};
    use crate::day_24::OpType::{Add, Eql, Mod, Mul};
    use crate::day_24::Param::{Lit, X, Z};
    use crate::day_24::{parse_input, Function};

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
