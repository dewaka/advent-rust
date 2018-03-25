# Advent of Code Problems in Rust

Solving problems from [Advent of Code](https://adventofcode.com/) to learn
Rust.

Solutions are written with the goals of learning Rust, and not focusing on
brevity or even outright performance.

Problems read test input from the standard input. They can be run with the
following convention, demonstrated for day 5, of 2015 problems.

From `advent15` directory,

```
cat ../resources/2015/day5 | cargo run 5
```

Furthermore, most problems contain simple unit tests to test against the solved
examples (if any) given in the problem description.

As an example, to run 2015, day 18th tests,

```
cargo test 18
```
