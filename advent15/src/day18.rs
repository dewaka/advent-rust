// https://adventofcode.com/2015/day/18
/*
--- Day 18: Like a GIF For Your Yard ---

After the million lights incident, the fire code has gotten stricter: now, at most ten thousand
lights are allowed. You arrange them in a 100x100 grid.

Never one to let you down, Santa again mails you instructions on the ideal lighting configuration.
With so few lights, he says, you'll have to resort to animation.

Start by setting your lights to the included initial configuration (your puzzle input). A # means
"on", and a . means "off".

Then, animate your grid in steps, where each step decides the next configuration based on the
current one. Each light's next state (either on or off) depends on its current state and the
current states of the eight lights adjacent to it (including diagonals). Lights on the edge of the
grid might have fewer than eight neighbors; the missing ones always count as "off".

For example, in a simplified 6x6 grid, the light marked A has the neighbors numbered 1 through 8,
and the light marked B, which is on an edge, only has the neighbors marked 1 through 5:

1B5...
234...
......
..123.
..8A4.
..765.

The state a light should have next is based on its current state (on or off) plus the number of
neighbors that are on:

- A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
- A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
- All of the lights update simultaneously; they all consider the same current state before moving
  to the next.

Here's a few steps from an example configuration of another 6x6 grid:

Initial state:
.#.#.#
...##.
#....#
..#...
#.#..#
####..

After 1 step:
..##..
..##.#
...##.
......
#.....
#.##..

After 2 steps:
..###.
......
..###.
......
.#....
.#....

After 3 steps:
...#..
......
...#..
..##..
......
......

After 4 steps:
......
......
..##..
..##..
......
......
After 4 steps, this example has four lights on.

In your grid of 100x100 lights, given your initial configuration, how many lights are on after 100
steps?
*/

extern crate ndarray;

use self::ndarray::prelude::*;
// use self::ndarray::{Array, Array2, ArrayD, IxDyn, ShapeBuilder, arr2};
use std::io::{self, BufRead};

#[derive(Debug)]
struct Board {
    lights: Array2<u8>,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
        Board {
            lights: ndarray::Array::from_elem((rows, cols), 0),
        }
    }

    pub fn print(&self) {
        for row in self.lights.genrows() {
            for &x in row {
                if x > 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    #[allow(dead_code)]
    pub fn turn_on_corners(&mut self) {
        let n = self.lights.rows();
        let m = self.lights.cols();
        self.lights[[1, 1]] = 1;
        self.lights[[1, m - 2]] = 1;
        self.lights[[n - 2, 1]] = 1;
        self.lights[[n - 2, m - 2]] = 1;
    }

    pub fn iterate(&mut self, scratch: &mut Board) {
        let mut neigh = scratch.lights.view_mut();
        neigh.fill(0);

        neigh += &self.lights.slice(s![0..-2, 0..-2]);
        neigh += &self.lights.slice(s![0..-2, 1..-1]);
        neigh += &self.lights.slice(s![0..-2, 2..]);

        neigh += &self.lights.slice(s![1..-1, 0..-2]);
        neigh += &self.lights.slice(s![1..-1, 2..]);

        neigh += &self.lights.slice(s![2.., 0..-2]);
        neigh += &self.lights.slice(s![2.., 1..-1]);
        neigh += &self.lights.slice(s![2.., 2..]);

        // birth where n = 3 and lights[i] = 0
        // survive where n = 2 || n = 3 and lights[i] = 1
        let mut zv = self.lights.slice_mut(s![1..-1, 1..-1]);

        zv.zip_mut_with(&neigh, |y, &n| *y = ((n == 3) || (n == 2 && *y > 0)) as u8);
    }

    fn parse(n: usize, x: &[u8]) -> Option<Board> {
        // make a border of 0 cells
        let a = Array::from_iter(x.iter().filter_map(|&b| match b {
            b'#' => Some(1),
            b'.' => Some(0),
            _ => None,
        }));

        if let Ok(a) = a.into_shape((n, n)) {
            let mut map: Array2<u8> = Array2::from_elem((n + 2, n + 2), 0);
            map.slice_mut(s![1..-1, 1..-1]).assign(&a);

            Some(Board { lights: map })
        } else {
            None
        }
    }

    fn count_on(&self) -> usize {
        let alive = self.lights.iter().filter(|&&x| x > 0).count();
        alive
    }
}

fn compute_board_state(bsize: usize, init: &[u8], rounds: i32) -> Option<Board> {
    let board_op = Board::parse(bsize, init);
    if board_op.is_some() {
        let mut board = board_op.unwrap();
        let mut scratch = Board::new(bsize, bsize);

        println!("Initial board");
        board.print();
        for _ in 0..rounds {
            board.iterate(&mut scratch);
            // board.turn_on_corners();
        }

        println!("Final board after {} rounds", rounds);
        board.print();
        println!(
            "There are {} lights on after {} rounds",
            board.count_on(),
            rounds
        );

        Some(board)
    } else {
        println!("Error initialising board");
        None
    }
}

// For the problem, we expect to read 100x100 board from stdin
pub fn problem() {
    let mut init_board: Vec<u8> = vec![];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();
        init_board.extend_from_slice(sline.as_bytes());
    }

    compute_board_state(100, &init_board, 100);
}

#[test]
fn test_ndarray() {
    let board = compute_board_state(6, b".#.#.#...##.#....#..#...#.#..#####..", 4);
    assert!(board.is_some());
    // In the given example, there should be 4 bulbs on after 4 rounds
    assert_eq!(board.unwrap().count_on(), 4);
}
