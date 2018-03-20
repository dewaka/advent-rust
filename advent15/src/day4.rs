// https://adventofcode.com/2015/day/4

extern crate md5;

use std;
use std::io::{self, BufRead};

fn calculate_match(s: &String) -> u64 {
    for num in 0..std::u64::MAX {
        let src: String = format!("{}{}", s, num);
        let digest = md5::compute(&src);
        let hash: String = format!("{:x}", digest);

        if hash.starts_with("00000") {
            return num;
        }
    }

    return 0;
}

pub fn problem() {
    println!("2015, day 4");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();
        println!("{} -> {}", &sline, calculate_match(&sline));
    }
}

#[test]
fn test_examples() {
    assert_eq!(609043, calculate_match(&"abcdef".to_owned()));
    assert_eq!(1048970, calculate_match(&"pqrstuv".to_owned()));
}
