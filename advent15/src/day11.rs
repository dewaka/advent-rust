// https://adventofcode.com/2015/day/11
/*
--- Day 11: Corporate Policy ---

Santa's previous password expired, and he needs help choosing a new one.

To help him remember his new password after the old one expires, Santa has
devised a method of coming up with a password based on the previous one.
Corporate policy dictates that passwords must be exactly eight lowercase letters
(for security reasons), so he finds his new password by incrementing his old
password string repeatedly until it is valid.

Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on.
Increase the rightmost letter one step; if it was z, it wraps around to a, and
repeat with the next letter to the left until one doesn't wrap around.

Unfortunately for Santa, a new Security-Elf recently started, and he has imposed
some additional password requirements:

- Passwords must include one increasing straight of at least three letters, like
  abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't
  count.
- Passwords may not contain the letters i, o, or l, as these letters can be
  mistaken for other characters and are therefore confusing.
- Passwords must contain at least two different, non-overlapping pairs of letters,
  like aa, bb, or zz.

For example:

hijklmmn meets the first requirement (because it contains the straight hij) but fails the second requirement requirement (because it contains i and l).
abbceffg meets the third requirement (because it repeats bb and ff) but fails the first requirement.
abbcegjk fails the third requirement, because it only has one double letter (bb).
The next password after abcdefgh is abcdffaa.
The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords that start with ghi..., since i is not allowed.

Given Santa's current password (your puzzle input), what should his next password be?

Your puzzle input is hepxcrrq.
*/
use std::io::{self, BufRead};

#[derive(Debug)]
struct Password {
    pass: Vec<u8>,
}

impl Password {
    fn is_valid_pass_char(b: u8) -> bool {
        let c = b as char;

        if c < 'a' || c > 'z' {
            return false;
        }

        match c {
            'i' | 'o' | 'l' => false,
            _ => true,
        }
    }

    fn from_str(s: &String) -> Option<Password> {
        let mut data: Vec<u8> = vec![];

        for c in s.as_bytes() {
            if Password::is_valid_pass_char(*c) {
                data.push(*c);
            } else {
                return None;
            }
        }

        Some(Password { pass: data })
    }

    fn has_increasing_seq(&self) -> bool {
        let mut i = 0;
        while i < self.pass.len() - 3 {
            // Check if we can find three consecutive letters
            let mut c = self.pass[i];
            let mut found = true;
            for n in 1..3 {
                if c + 1 != self.pass[i + n] {
                    found = false;
                    break;
                }
                c = self.pass[i + n];
            }

            if found {
                return true;
            }

            i += 1;
        }

        false
    }

    fn has_valid_chars(&self) -> bool {
        for c in &self.pass {
            if !Password::is_valid_pass_char(*c) {
                return false;
            }
        }

        true
    }

    fn count_non_overlapping_pairs(&self) -> i32 {
        let mut i = 0;
        while i < self.pass.len() - 3 {
            // Check if we can find three consecutive letters
            let mut c = self.pass[i];
            let mut found = true;
            for n in 1..3 {
                if c + 1 != self.pass[i + n] {
                    found = false;
                    break;
                }
                c = self.pass[i + n];
            }

            if found {
                unimplemented!();
            }

            i += 1;
        }

        unimplemented!();
    }

    fn has_required_pairs(&self) -> bool {
        self.count_non_overlapping_pairs() >= 2
    }

    pub fn is_valid(&self) -> bool {
        self.has_increasing_seq() && self.has_valid_chars() && self.has_required_pairs()
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.pass.clone()).unwrap()
    }
}

fn test_pass() {
    let p = Password::from_str(&"xyz".to_owned()).unwrap();
    println!("{}", p.to_string());

    if p.is_valid() {
        println!("{} is valid", p.to_string());
    } else {
        println!("{} is invalid", p.to_string());
    }
}

pub fn problem() {
    test_pass();
    return;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        println!("{}", sline);
    }
}

#[test]
fn test_valid_passwords() {
    let p1 = Password::from_str(&"hijklmmn".to_owned()).unwrap();
    assert!(!p1.is_valid());
}
