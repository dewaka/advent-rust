// https://adventofcode.com/2015/day/6

/*
--- Day 6: Probably a Fire Hazard ---

Because your neighbors keep defeating you in the holiday house decorating
contest year after year, you've decided to deploy one million lights in a
1000x1000 grid.

Furthermore, because you've been especially nice this year, Santa has mailed you
instructions on how to display the ideal lighting configuration.

Lights in your grid are numbered from 0 to 999 in each direction; the lights at
each corner are at 0,0, 0,999, 999,999, and 999,0. The instructions include
whether to turn on, turn off, or toggle various inclusive ranges given as
coordinate pairs. Each coordinate pair represents opposite corners of a
rectangle, inclusive; a coordinate pair like 0,0 through 2,2 therefore refers to
9 lights in a 3x3 square. The lights all start turned off.

To defeat your neighbors this year, all you have to do is set up your lights by
doing the instructions Santa sent you in order.

For example:
- turn on 0,0 through 999,999 would turn on (or leave on) every light.
- toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning
  off the ones that were on, and turning on the ones that were off.
- turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.

After following the instructions, how many lights are lit?
*/

extern crate regex;

use self::regex::Regex;
use std::io::{self, BufRead};

struct Board {
    lights: [[u8; 1000]; 1000],
}

#[derive(Debug, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
enum Command {
    TurnOn { from: Pos, to: Pos },
    TurnOff { from: Pos, to: Pos },
    Toggle { from: Pos, to: Pos },
}

fn toggle(board: &mut Board, from: &Pos, to: &Pos) {
    for x in from.x..to.x + 1 {
        for y in from.y..to.y + 1 {
            if board.lights[x as usize][y as usize] == 0 {
                board.lights[x as usize][y as usize] = 1;
            } else {
                board.lights[x as usize][y as usize] = 0;
            }
        }
    }
}

fn turn_on(board: &mut Board, from: &Pos, to: &Pos) {
    for x in from.x..to.x + 1 {
        for y in from.y..to.y + 1 {
            board.lights[x as usize][y as usize] = 1;
        }
    }
}

fn turn_off(board: &mut Board, from: &Pos, to: &Pos) {
    for x in from.x..to.x + 1 {
        for y in from.y..to.y + 1 {
            board.lights[x as usize][y as usize] = 0;
        }
    }
}

fn update_board(board: &mut Board, cmd: &Command) {
    match cmd {
        &Command::TurnOn { ref from, ref to } => turn_on(board, &from, &to),
        &Command::TurnOff { ref from, ref to } => turn_off(board, &from, &to),
        &Command::Toggle { ref from, ref to } => toggle(board, &from, &to),
    }
}

fn count_on(board: &Board) -> i32 {
    let mut count = 0;
    for row in board.lights.iter() {
        for bulb in row.iter() {
            if *bulb == 1 {
                count += 1;
            }
        }
    }

    count
}

/*
toggle 461,550 through 564,900
turn off 370,39 through 425,839
turn on 599,989 through 806,993
*/
fn parse_command(cmd: &String) -> Option<Command> {
    lazy_static! {
        static ref TOGGLE_REG: Regex = Regex::new(r"toggle (\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})").unwrap();
        static ref TURNON_REG: Regex =
            Regex::new(r"turn on (\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})").unwrap();
        static ref TURNOFF_REG: Regex =
            Regex::new(r"turn off (\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})").unwrap();
    }

    if TOGGLE_REG.is_match(cmd) {
        for cap in TOGGLE_REG.captures_iter(cmd) {
            let from = Pos {
                x: cap[1].parse::<i32>().unwrap(),
                y: cap[2].parse::<i32>().unwrap(),
            };

            let to = Pos {
                x: cap[3].parse::<i32>().unwrap(),
                y: cap[4].parse::<i32>().unwrap(),
            };

            return Some(Command::Toggle { to: to, from: from });
        }
    } else if TURNON_REG.is_match(cmd) {
        for cap in TURNON_REG.captures_iter(cmd) {
            let from = Pos {
                x: cap[1].parse::<i32>().unwrap(),
                y: cap[2].parse::<i32>().unwrap(),
            };

            let to = Pos {
                x: cap[3].parse::<i32>().unwrap(),
                y: cap[4].parse::<i32>().unwrap(),
            };

            return Some(Command::TurnOn { to: to, from: from });
        }
    } else if TURNOFF_REG.is_match(cmd) {
        for cap in TURNOFF_REG.captures_iter(cmd) {
            let from = Pos {
                x: cap[1].parse::<i32>().unwrap(),
                y: cap[2].parse::<i32>().unwrap(),
            };

            let to = Pos {
                x: cap[3].parse::<i32>().unwrap(),
                y: cap[4].parse::<i32>().unwrap(),
            };

            return Some(Command::TurnOff { to: to, from: from });
        }
    }

    None
}

pub fn problem() {
    println!("2015, day 6");

    let mut board = Board {
        lights: [[0; 1000]; 1000],
    };
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        match parse_command(&sline) {
            None => println!("Failed to parse: {}", sline),
            Some(c) => update_board(&mut board, &c),
        }
    }

    println!("There are {} bulbs on", count_on(&board));
}

#[test]
fn test_parse_toggle() {
    assert_eq!(
        parse_command(&"toggle 461,550 through 564,900".to_owned()),
        Some(Command::Toggle {
            from: Pos { x: 461, y: 550 },
            to: Pos { x: 564, y: 900 },
        })
    );

    assert_eq!(
        parse_command(&"toggle 0,0 through 1,34".to_owned()),
        Some(Command::Toggle {
            from: Pos { x: 0, y: 0 },
            to: Pos { x: 1, y: 34 },
        })
    );
}

#[test]
fn test_parse_turnon() {
    assert_eq!(
        parse_command(&"turn on 599,989 through 806,993".to_owned()),
        Some(Command::TurnOn {
            from: Pos { x: 599, y: 989 },
            to: Pos { x: 806, y: 993 },
        })
    );

    assert_eq!(
        parse_command(&"turn on 0,0 through 23,34".to_owned()),
        Some(Command::TurnOn {
            from: Pos { x: 0, y: 0 },
            to: Pos { x: 23, y: 34 },
        })
    );
}

#[test]
fn test_parse_turnoff() {
    assert_eq!(
        parse_command(&"turn off 370,39 through 425,839".to_owned()),
        Some(Command::TurnOff {
            from: Pos { x: 370, y: 39 },
            to: Pos { x: 425, y: 839 },
        })
    );

    assert_eq!(
        parse_command(&"turn off 370,39 through 425,839".to_owned()),
        Some(Command::TurnOff {
            from: Pos { x: 370, y: 39 },
            to: Pos { x: 425, y: 839 },
        })
    );
}

// #[test]
// fn test_examples() {
//     assert!(is_nice(&"ugknbfddgicrmopn".to_owned()));
// }
