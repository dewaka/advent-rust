// https://adventofcode.com/2015/day/3
use std::io::{self, BufRead};
use std::collections::HashSet;

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

fn move_pos(p: Pos, d: Direction) -> Pos {
    match d {
        Direction::North => Pos { x: p.x, y: p.y + 1 },
        Direction::East => Pos { x: p.x + 1, y: p.y },
        Direction::South => Pos { x: p.x, y: p.y - 1 },
        Direction::West => Pos { x: p.x - 1, y: p.y },
    }
}

fn process_move(current: Pos, dir: Direction, pos: &mut HashSet<Pos>) -> Pos {
    let new_pos = move_pos(current, dir);
    pos.insert(new_pos);
    new_pos
}

fn process_moves(cmd: &String) -> i32 {
    let mut pos_set: HashSet<Pos> = HashSet::new();
    let mut current = Pos { x: 0, y: 0 };
    pos_set.insert(current);    // add initial position

    for c in cmd.chars() {
        match c {
            '^' => current = process_move(current, Direction::North, &mut pos_set),
            '>' => current = process_move(current, Direction::East, &mut pos_set),
            'v' => current = process_move(current, Direction::South, &mut pos_set),
            '<' => current = process_move(current, Direction::West, &mut pos_set),
            _ => println!("Invalid direction: {}", c),
        }
    }

    pos_set.len() as i32
}

pub fn problem() {
    println!("2015, day 3");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("Houses: {}", process_moves(&line.unwrap()));
    }
}

#[test]
fn test_examples() {
    assert_eq!(2, process_moves(&">".to_owned()));
    assert_eq!(4, process_moves(&"^>v<".to_owned()));
    assert_eq!(2, process_moves(&"^v^v^v^v^v".to_owned()));
}
