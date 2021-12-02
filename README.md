# Advent of Code 2021

[Advent of Code Website](https://adventofcode.com/)

Scripts written to solve the 2021 addition of Advent of Code. Code is written in Rust. Despite writing last year's
puzzles in rust, I am still a Rust beginner so expect some horrible code. I will hopefully improve as the month
progresses.

[`main.rs`](https://github.com/kamioftea/advent-of-code-2021/blob/main/src/main.rs) - This is the entry point to the
script, and follows a pattern of asking for a day to run, then deferring to
`day_X.rs` for each days' solutions. Unit tests for each day written based on the examples given in the puzzle
descriptions are in a `tests` submodule in that day's file.

Alongside the puzzles I'm trying to learn how to use GitHub actions / pages to automate publishing the docs.

There is a [GitHub action](./.github/workflows/rust.yml) that runs on a pull request -> main to check everything is in
order. This:

- Builds the project
- Runs the tests
- Builds the docs

To enforce these checks the main branch has been protected, and pull requests to main require the action to complete
before they can be merged.

When the pull request is merged into main, a [second GitHub action](./.github/workflows/rust-docs.yml) is triggered.
This:

- Merges the main branch changes into the ghdocs branch
- Build the docs
- Deletes the old `/docs`, and copied the updated version in their place
- Commits and pushes any changes.

The [GitHub Pages Site](https://kamioftea.github.io/advent-of-code-2021/advent_of_code_2021/) for the repository is set
to be publised from the `/docs` folder of the ghpages branch, so this commit and push triggers a re-deployment of the 
pages site with the updated content automatically.

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
  
