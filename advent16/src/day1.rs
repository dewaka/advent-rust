// https://adventofcode.com/2016/day/1
/*
--- Day 1: No Time for a Taxicab ---

You're airdropped near Easter Bunny Headquarters in a city somewhere. "Near", unfortunately, is as
close as you can get - the instructions on the Easter Bunny Recruiting Document the Elves
intercepted start here, and nobody had time to work them out further.

The Document indicates that you should start at the given coordinates (where you just landed) and
face North. Then, follow the provided sequence: either turn left (L) or right (R) 90 degrees, then
walk forward the given number of blocks, ending at a new intersection.

There's no time to follow such ridiculous instructions on foot, though, so you take a moment and
work out the destination. Given that you can only walk on the street grid of the city, how far is
the shortest path to the destination?

For example:

- Following R2, L3 leaves you 2 blocks East and 3 blocks North, or 5 blocks away.
- R2, R2, R2 leaves you 2 blocks due South of your starting position, which is 2 blocks away.
- R5, L5, R5, R3 leaves you 12 blocks away.

How many blocks away is Easter Bunny HQ?
*/
use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq)]
enum Side {
    R(i32),
    L(i32),
}

const START_POS: Pos = Pos {
    x: 0,
    y: 0,
    direction: Direction::North,
};

fn distance(pos: &Pos) -> i32 {
    let abs_x = (pos.x - START_POS.x).abs();
    let abs_y = (pos.y - START_POS.y).abs();
    abs_x + abs_y
}

fn update_pos(mut pos: Pos, side: Side) -> Pos {
    match pos.direction {
        Direction::North => match side {
            Side::R(n) => {
                pos.direction = Direction::East;
                pos.x += n;
                pos
            }
            Side::L(n) => {
                pos.direction = Direction::West;
                pos.x -= n;
                pos
            }
        },
        Direction::East => match side {
            Side::R(n) => {
                pos.direction = Direction::South;
                pos.y -= n;
                pos
            }
            Side::L(n) => {
                pos.direction = Direction::North;
                pos.y += n;
                pos
            }
        },
        Direction::South => match side {
            Side::R(n) => {
                pos.direction = Direction::West;
                pos.x -= n;
                pos
            }
            Side::L(n) => {
                pos.direction = Direction::East;
                pos.x += n;
                pos
            }
        },
        Direction::West => match side {
            Side::R(n) => {
                pos.direction = Direction::North;
                pos.y += n;
                pos
            }
            Side::L(n) => {
                pos.direction = Direction::South;
                pos.y -= n;
                pos
            }
        },
    }
}

fn update_positions(pos: Pos, sides: &[Side]) -> Pos {
    sides.iter().fold(pos, |p, s| update_pos(p, s.to_owned()))
}

fn parse_side(s: &str) -> Option<Side> {
    if s.starts_with("R") {
        let num_part: String = s.chars().skip(1).collect();
        if let Ok(num) = num_part.parse::<i32>() {
            return Some(Side::R(num));
        }
    } else if s.starts_with("L") {
        let num_part: String = s.chars().skip(1).collect();
        if let Ok(num) = num_part.parse::<i32>() {
            return Some(Side::L(num));
        }
    }

    None
}

fn parse_sides(s: &str, sides: &mut Vec<Side>) {
    for x in s.split(",") {
        if let Some(d) = parse_side(x.trim()) {
            sides.push(d);
        }
    }
}

pub fn problem() {
    let mut sides: Vec<Side> = vec![];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        parse_sides(&sline, &mut sides);
    }

    println!("There {} sides", sides.len());

    let mut pos = START_POS;
    pos = update_positions(pos, &sides);
    println!("Final pos: {:?}", pos);
    println!("Final distance: {}", distance(&pos));
}

#[test]
fn test_parse_side() {
    assert_eq!(Some(Side::R(3)), parse_side("R3"));
    assert_eq!(Some(Side::L(8)), parse_side("L8"));
    assert_eq!(None, parse_side("LL"));
    assert_eq!(None, parse_side("P"));
    assert_eq!(None, parse_side(""));
    assert_eq!(None, parse_side("8"));
}

#[test]
fn test_examples() {
    {
        let mut pos = START_POS;
        pos = update_positions(pos, &vec![Side::R(2), Side::L(3)]);
        assert_eq!(5, distance(&pos));
    }

    {
        let mut pos = START_POS;
        pos = update_positions(pos, &vec![Side::R(2), Side::R(2), Side::R(2)]);
        assert_eq!(2, distance(&pos));
    }

    {
        let mut pos = START_POS;
        pos = update_positions(pos, &vec![Side::R(5), Side::L(5), Side::R(5), Side::R(3)]);
        assert_eq!(12, distance(&pos));
    }

    {
        let mut pos = START_POS;
        pos = update_positions(pos, &vec![Side::R(5), Side::R(5), Side::R(5), Side::R(5)]);
        println!("Pos: {:?}", pos);
        assert_eq!(0, distance(&pos));
    }
}
