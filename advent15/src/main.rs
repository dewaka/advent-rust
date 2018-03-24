#[macro_use] extern crate lazy_static;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

use std::env;

fn print_usage(prog: &String) {
    println!("usage: {} <day number>", prog);
}

pub fn main() {
    println!("Advent of Code, 2015");

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
        "8" => day8::problem(),
        "9" => day9::problem(),
        "10" => day10::problem(),
        "11" => day11::problem(),
        "12" => day12::problem(),
        "13" => day13::problem(),
        "14" => day14::problem(),
        "15" => day15::problem(),
        "16" => day16::problem(),
        "17" => day17::problem(),
        _ => print_usage(&args[0]),
    }
}
