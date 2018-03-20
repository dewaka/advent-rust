// https://adventofcode.com/2015/day/5

/*
--- Day 5: Doesn't He Have Intern-Elves For This? ---
Santa needs help figuring out which strings in his text file are naughty or nice.

A nice string is one with all of the following properties:
- It contains at least three vowels (aeiou only), like aei, xazegov, or
  aeiouaeiouaeiou.
- It contains at least one letter that appears twice in a row, like xx, abcdde
  (dd), or aabbccdd (aa, bb, cc, or dd).
- It does not contain the strings ab, cd, pq, or xy, even if they are part of
  one of the other requirements.

For example:
- ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...),
  a double letter (...dd...), and none of the disallowed substrings.
- aaa is nice because it has at least three vowels and a double letter, even
  though the letters used by different rules overlap.
- jchzalrnumimnmhp is naughty because it has no double letter.
- haegwjzuvuyypxyu is naughty because it contains the string xy.
- dvszwmarrgswjxmb is naughty because it contains only one vowel.

How many strings are nice?
*/

use std::io::{self, BufRead};

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn contains_invalid(c1: char, c2: char) -> bool {
    let check = format!("{}{}", c1, c2);
    match check.as_ref() {
        "ab" | "cd" | "pq" | "xy" => true,
        _ => false,
    }
}

fn is_nice(s: &String) -> bool {
    let mut twice = false;
    let mut vowels = 0;
    let mut prev: Option<char> = None;

    for c in s.chars() {
        match prev {
            None => prev = Some(c),
            Some(p) => {
                if contains_invalid(p, c) {
                    return false;
                }
                if p == c {
                    twice = true;
                }
                prev = Some(c);
            }
        }

        if is_vowel(c) {
            vowels += 1;
        }
    }

    twice && (vowels >= 3)
}

pub fn problem() {
    println!("2015, day 5");

    let mut count = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();
        if is_nice(&sline) {
            count += 1;
        }
    }

    println!("There are {} nice strings", count);
}

#[test]
fn test_examples() {
    assert!(is_nice(&"ugknbfddgicrmopn".to_owned()));
    assert!(is_nice(&"aaa".to_owned()));
    assert!(!is_nice(&"jchzalrnumimnmhp".to_owned()));
    assert!(!is_nice(&"haegwjzuvuyypxyu".to_owned()));
    assert!(!is_nice(&"dvszwmarrgswjxmb".to_owned()));
}
