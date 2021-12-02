# Advent of Code 2021

[Advent of Code Website](https://adventofcode.com/)

Scripts written to solve the 2021 addition of Advent of Code. Code is written in Rust. Despite 
writing last year's puzzles in rust, I am still a Rust beginner so expect some horrible code. 
I will hopefully improve as the month progresses.

[`main.rs`](https://github.com/kamioftea/advent-of-code-2021/blob/main/src/main.rs) - This is the 
entry point to the script, and follows a pattern of asking for a day to run, then deferring to
`day_X.rs` for each days' solutions. Unit tests for each day written based on the examples given in
the puzzle descriptions are in a `tests` submodule in that day's file.

There is a GitHub action that:
- Builds the project
- Runs the tests
- Runs/verifies the docs
- Copies the docs to the [GitHub Pages for the repository](https://kamioftea.github.io/advent-of-code-2021/advent_of_code_2021/)

## Previous years:
- 2020 `36/50` Rust [Github](https://github.com/kamioftea/advent-of-code-2020/tree/master),
  [Puzzles](https://adventofcode.com/2020)
- 2018 `10/50` Rust [Github](https://github.com/kamioftea/advent-of-code-2018/tree/master),
  [Puzzles](https://adventofcode.com/2018)
- 2017 `50/50` Scala [Github](https://github.com/kamioftea/advent-of-code-2017/tree/master), 
  [Write Ups](https://blog.goblinoid.co.uk/tag/advent-of-code-2017/),
  [Puzzles](https://adventofcode.com/2017)
- 2016 `10/50` Scala [Github](https://github.com/kamioftea/advent-of-code-2017/tree/master), 
  [Write Ups](https://kamioftea.github.io/advent-of-code-2016/)
  [Puzzles](https://adventofcode.com/2017)
  
