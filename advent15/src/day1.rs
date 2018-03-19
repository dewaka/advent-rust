// https://adventofcode.com/2015/day/1

use std::io::{self, BufRead};

fn compute_floor(s: &String) -> i32 {
    let mut floor = 0;
    for c in s.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }

    floor
}

fn print_answer(s: &String) {
    println!("Santa is in {} floor", compute_floor(s));
}

pub fn problem() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        print_answer(&line.unwrap());
    }
}
