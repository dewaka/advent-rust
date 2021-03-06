#[macro_use] extern crate lazy_static;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::env;

fn print_usage(prog: &String) {
    println!("usage: {} <day number>", prog);
}

pub fn main() {
    println!("Advent of Code, 2016");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_usage(&args[0]);
        return;
    }

    let day = &args[1];
    // Run given day's problem
    match day.as_ref() {
        "1" => day1::problem(),
        "2" => day2::problem(),
        "3" => day3::problem(),
        "4" => day4::problem(),
        "5" => day5::problem(),
        "6" => day6::problem(),
        "7" => day7::problem(),
        _ => print_usage(&args[0]),
    }
}
