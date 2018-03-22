// https://adventofcode.com/2015/day/10
/*
--- Day 10: Elves Look, Elves Say ---

Today, the Elves are playing a game called look-and-say. They take turns making
sequences by reading aloud the previous sequence and using that reading as the
next sequence. For example, 211 is read as "one two, two ones", which becomes
1221 (1 2, 2 1s).

Look-and-say sequences are generated iteratively, using the previous value as
input for the next step. For each step, take the previous value, and replace
each run of digits (like 111) with the number of digits (3) followed by the
digit itself (1).

For example:

- 1 becomes 11 (1 copy of digit 1).
- 11 becomes 21 (2 copies of digit 1).
- 21 becomes 1211 (one 2 followed by one 1).
- 1211 becomes 111221 (one 1, one 2, and two 1s).
- 111221 becomes 312211 (three 1s, two 2s, and one 1).

Starting with the digits in your puzzle input, apply this process 40 times. What
is the length of the result?

Your puzzle input is 1113122113
*/
use std::io::{self, BufRead};

type LookAndSaySeries = Vec<i32>;

struct LookAndSay {
    current: LookAndSaySeries,
}

impl Iterator for LookAndSay {
    type Item = LookAndSaySeries;

    fn next(&mut self) -> Option<LookAndSaySeries> {
        let current = self.current.clone();
        let next = look_and_say(&current);
        self.current = next;
        Some(current)
    }
}

fn look_and_say(cur: &LookAndSaySeries) -> LookAndSaySeries {
    let mut next: LookAndSaySeries = vec![];

    let mut i = 0;
    while i < cur.len() {
        let num = cur[i];
        let mut count = 1;
        i += 1;

        while i < cur.len() {
            if cur[i] == num {
                count += 1;
            } else {
                break;
            }
            i += 1;
        }

        next.push(count);
        next.push(num);
    }

    next
}

fn look_and_say_next(series: &LookAndSaySeries, steps: i32) -> LookAndSaySeries {
    let mut current = series.clone();
    for _ in 0..steps {
        let next = look_and_say(&current);
        current = next;
    }
    current
}

fn parse_series(s: &String) -> Option<LookAndSaySeries> {
    let mut series: LookAndSaySeries = vec![];
    for c in s.as_bytes() {
        if *c >= '0' as u8 && *c <= '9' as u8 {
            series.push(*c as i32 - '0' as i32);
        } else {
            return None;
        }
    }
    Some(series.clone())
}

fn test_look_and_say() {
    match parse_series(&"1".to_owned()) {
        Some(s) => {
            let srs = look_and_say_next(&s, 40);
            println!("Length: {}", srs.len());
        }
        None => {
            println!("Failed to parse series");
        }
    }

    let series = LookAndSay { current: vec![1] };

    let mut count = 1;
    for sr in series {
        println!("{}: {:?}", count, sr);
        if count >= 10 {
            break;
        }
        count += 1;
    }
}

pub fn problem() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        match parse_series(&sline) {
            Some(s) => {
                let ls = look_and_say_next(&s, 40);
                println!("Length: {}", ls.len());
            }
            None => {
                println!("Failed to parse series: {}", sline);
            }
        }
    }
}

#[test]
fn test_series_parsing() {
    assert_eq!(parse_series(&"1".to_owned()).unwrap(), vec![1]);
    assert_eq!(parse_series(&"11".to_owned()).unwrap(), vec![1, 1]);
    assert_eq!(
        parse_series(&"11213".to_owned()).unwrap(),
        vec![1, 1, 2, 1, 3]
    );
}

#[test]
fn test_examples() {
    assert_eq!(look_and_say(&vec![1]), vec![1, 1]);
    assert_eq!(look_and_say(&vec![1, 1]), vec![2, 1]);
    assert_eq!(look_and_say(&vec![2, 1]), vec![1, 2, 1, 1]);
    assert_eq!(look_and_say(&vec![1, 2, 1, 1]), vec![1, 1, 1, 2, 2, 1]);
}
