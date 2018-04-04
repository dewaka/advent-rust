// https://adventofcode.com/2016/day/6
/*
--- Day 6: Signals and Noise ---

Something is jamming your communications with Santa. Fortunately, your signal is
only partially jammed, and protocol in situations like this is to switch to a
simple repetition code to get the message through.

In this model, the same message is sent repeatedly. You've recorded the
repeating message signal (your puzzle input), but the data seems quite corrupted
- almost too badly to recover. Almost.

All you need to do is figure out which character is most frequent for each
position. For example, suppose you had recorded the following messages:

eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar

The most common character in the first column is e; in the second, a; in the
third, s, and so on. Combining these characters returns the error-corrected
message, easter.

Given the recording in your puzzle input, what is the error-corrected version of the message being sent?
*/

use std::io::{self, BufRead};
use std::collections::HashMap;

type FreqMap = HashMap<char, i32>;

struct Message {
    frequencies: Vec<FreqMap>,
}

impl Message {
    fn add_to_map(fmap: &mut FreqMap, c: char) {
        // Update the count for given character c
        let c_count = fmap.entry(c).or_insert(0);
        *c_count += 1;
    }

    fn add_char(&mut self, pos: usize, c: char) {
        if pos >= self.frequencies.len() {
            self.frequencies.resize(pos + 1, FreqMap::new());
        }
        let fmap = &mut self.frequencies[pos];
        Message::add_to_map(fmap, c);
    }

    pub fn add_message(&mut self, msg: &str) {
        for (p, c) in msg.chars().enumerate() {
            self.add_char(p, c);
        }
    }

    fn get_most_frequent(fmap: &FreqMap) -> Option<char> {
        let mut fchar: Option<char> = None;
        let mut ftimes: Option<i32> = None;

        for (c, v) in fmap.iter() {
            match ftimes {
                Some(val) => {
                    if val < *v {
                        ftimes = Some(*v);
                        fchar = Some(*c);
                    }
                }
                None => {
                    ftimes = Some(*v);
                    fchar = Some(*c);
                }
            }
        }

        fchar
    }

    pub fn get_decoded(&self) -> String {
        let mut msg = String::new();

        for fmap in &self.frequencies {
            if let Some(c) = Message::get_most_frequent(&fmap) {
                msg.push(c);
            }
        }

        msg
    }

    pub fn new() -> Message {
        Message {
            frequencies: vec![],
        }
    }
}

pub fn problem() {
    let mut msg = Message::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();
        msg.add_message(&sline);
    }

    let decoded = msg.get_decoded();
    println!("Decoded message: {}", decoded);
}

#[test]
fn test_example() {
    let mut msg = Message::new();

    let messages = vec![
        "eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa", "rasrtv", "nssdts",
        "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear", "dvrsen", "enarar",
    ];

    for m in &messages {
        msg.add_message(m);
    }

    let decoded = msg.get_decoded();
    assert_eq!("easter", decoded);
}
