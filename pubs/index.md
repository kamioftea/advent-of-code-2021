---
header: Jeff's Advent of Code 2021
---
[Advent of Code](https://adventofcode.com/2021) Is a yearly challenge with one coding puzzle a day from 1st of December
until Christmas Day. The challenges are language agnostic, providing the input as a text file, and expecting a number or
a string as the result of each part.

This year I've chosen to use [Rust](https://www.rust-lang.org/), and I've used the built-in Rust Doc tool to build
write-ups of my solutions from doc blocks. I've then used GitHub Actions to automatically build the Rust Docs and
publish them to GitHub Pages. The individual days are all linked from
the [ advent_of_code_2021 crate docs](https://kamioftea.github.io/advent-of-code-2021/advent_of_code_2021/).

Rust doc was great for talking about each solution, but didn't provide a good way to customise the landing page and
introduction to the challenge. I've filled this gap with some static pages built by [11ty](https://www.11ty.dev/).

## My Solutions

<div class="solutions-list">
{% for solution in solutions %}
  <div class="solution">
    <p>{{solution.title}}</p>
    <p>
      [<!--suppress HtmlUnknownTarget --><a href="{{solution.url}}">Puzzle</a>]
      [<a href="https://kamioftea.github.io/advent-of-code-2021/advent_of_code_2021/day_{{solution.day}}/index.html">Write Up</a>]
    </p>
  </div>
{% endfor %}
</div>
