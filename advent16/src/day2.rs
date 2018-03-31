// https://adventofcode.com/2016/day/2
/*
--- Day 2: Bathroom Security ---

You arrive at Easter Bunny Headquarters under cover of darkness. However, you
left in such a rush that you forgot to use the bathroom! Fancy office buildings
like this one usually have keypad locks on their bathrooms, so you search the
front desk for the code.

"In order to improve security," the document you find says, "bathroom codes will
no longer be written down. Instead, please memorize and follow the procedure
below to access the bathrooms."

The document goes on to explain that each button to be pressed can be found by
starting on the previous button and moving to adjacent buttons on the keypad: U
moves up, D moves down, L moves left, and R moves right. Each line of
instructions corresponds to one button, starting at the previous button (or, for
the first line, the "5" button); press whatever button you're on at the end of
each line. If a move doesn't lead to a button, ignore it.

You can't hold it much longer, so you decide to figure out the code as you walk
to the bathroom. You picture a keypad like this:

1 2 3
4 5 6
7 8 9
Suppose your instructions are:

ULL
RRDDD
LURDL
UUUUD

- You start at "5" and move up (to "2"), left (to "1"), and left (you can't, and
  stay on "1"), so the first button is 1.
- Starting from the previous button ("1"), you move right twice (to "3") and then
  down three times (stopping at "9" after two moves and ignoring the third),
  ending up with 9.
- Continuing from "9", you move left, up, right, down, and left, ending with 8.
- Finally, you move up four times (stopping at "2"), then down once, ending with 5.
  So, in this example, the bathroom code is 1985.

Your puzzle input is the instructions from the document you found at the front
desk. What is the bathroom code?
*/
use std::io::{self, BufRead};

const BOARD_SIZE: i32 = 3;

type Pos = (i32, i32);

fn move_pos(p: Pos, d: char) -> Pos {
    let bounded = |x, y| if 0 <= x && x < BOARD_SIZE { x } else { y };

    let (x, y) = p;
    match d {
        'R' => (bounded(x + 1, x), y),
        'L' => (bounded(x - 1, x), y),
        'U' => (x, bounded(y - 1, y)),
        'D' => (x, bounded(y + 1, y)),
        _ => p,
    }
}

// We map top most left pos (0, 0) to 1, and bottom most right (2, 2) -> 9
fn pos_to_digit(p: Pos) -> i32 {
    let (x, y) = p;
    (x + 1) + BOARD_SIZE * y
}

fn process_instr(mut p: Pos, instr: &str) -> (Pos, i32) {
    for c in instr.chars() {
        p = move_pos(p, c);
    }
    (p, pos_to_digit(p))
}

#[allow(dead_code)]
fn process_code(mut p: Pos, instrs: &[&str]) -> String {
    let mut code: String = String::new();

    for instr in instrs {
        let (p_new, d) = process_instr(p, instr);
        p = p_new;
        code.push_str(&format!("{}", d));
    }

    code
}

pub fn problem() {
    let mut pos: Pos = (1, 1); // we start at 5th digit

    print!("Code: ");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        let (new_pos, digit) = process_instr(pos, &sline);
        pos = new_pos;
        print!("{}", digit);
    }

    println!("");
}

#[test]
fn test_pos_to_digit() {
    assert_eq!(1, pos_to_digit((0, 0)));
    assert_eq!(2, pos_to_digit((1, 0)));
    assert_eq!(3, pos_to_digit((2, 0)));
    assert_eq!(4, pos_to_digit((0, 1)));
    assert_eq!(5, pos_to_digit((1, 1)));
    assert_eq!(6, pos_to_digit((2, 1)));
    assert_eq!(7, pos_to_digit((0, 2)));
    assert_eq!(8, pos_to_digit((1, 2)));
    assert_eq!(9, pos_to_digit((2, 2)));
}

#[test]
fn test_move_pos() {
    assert_eq!((0, 0), move_pos((0, 0), 'L'));
    assert_eq!((0, 1), move_pos((0, 0), 'D'));
    assert_eq!((1, 0), move_pos((0, 0), 'R'));
    assert_eq!((0, 0), move_pos((0, 0), 'U'));

    assert_eq!((1, 0), move_pos((1, 1), 'U'));
    assert_eq!((1, 2), move_pos((1, 1), 'D'));

    assert_eq!((2, 2), move_pos((2, 2), 'D'));
    assert_eq!((1, 2), move_pos((2, 2), 'L'));
}

#[test]
fn test_example() {
    let p = (1, 1);
    let instrs = vec!["ULL", "RRDDD", "LURDL", "UUUUD"];

    let code = process_code(p, &instrs);
    assert_eq!("1985", code);
}
