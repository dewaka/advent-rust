// https://adventofcode.com/2016/day/4
/*
--- Day 4: Security Through Obscurity ---

Finally, you come across an information kiosk with a list of rooms. Of course, the list is
encrypted and full of decoy data, but the instructions to decode the list are barely hidden nearby.
Better remove the decoy data first.

Each room consists of an encrypted name (lowercase letters separated by dashes) followed by a dash,
a sector ID, and a checksum in square brackets.

A room is real (not a decoy) if the checksum is the five most common letters in the encrypted name,
in order, with ties broken by alphabetization. For example:

- aaaaa-bbb-z-y-x-123[abxyz] is a real room because the most common letters are a (5), b (3), and 
  then a tie between x, y, and z, which are listed alphabetically.
- a-b-c-d-e-f-g-h-987[abcde] is a real room because although the letters are all tied (1 of each), 
  the first five are listed alphabetically.
- not-a-real-room-404[oarel] is a real room.
- totally-real-room-200[decoy] is not.

Of the real rooms from the list above, the sum of their sector IDs is 1514.

What is the sum of the sector IDs of the real rooms?
*/

extern crate regex;

use std::cmp::Ordering;
use self::regex::Regex;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Entry {
    data: String,
    sector_id: i32,
    hash: String,
}

impl Entry {
    fn valid(&self) -> bool {
        let calc_checksum = Entry::calculate_checksum(&self.data);
        calc_checksum == self.hash
    }

    fn calculate_checksum(s: &str) -> String {
        let mut freq_map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            // ignore dashes
            if c == '-' {
                continue;
            }

            *freq_map.entry(c).or_insert(0) += 1;
        }

        let mut ps: Vec<(char, i32)> = freq_map.iter().map(|(c, n)| (*c, *n)).collect();
        // We need to sort the vector by frequecy + alphabetization
        ps.sort_by(|&(c1, n1), &(c2, n2)| {
            let count_order = n2.cmp(&n1); // descending order for frequency
            if count_order == Ordering::Equal {
                c1.cmp(&c2) // ascending order for characters
            } else {
                count_order
            }
        });

        // Collect just first 5 characters into a String
        let hash = ps.iter().map(|&(c, _)| c).take(5).collect();
        hash
    }

    fn parse_entry(s: &str) -> Option<Entry> {
        lazy_static! {
            static ref R_ENTRY: Regex = Regex::new(r"([a-z,-]*)-(\d+)\[(\w+)\]").unwrap();
        }

        if R_ENTRY.is_match(s) {
            for cap in R_ENTRY.captures_iter(s) {
                if let Ok(sector_id) = cap[2].parse::<i32>() {
                    let entry = Entry {
                        data: cap[1].to_string(),
                        sector_id,
                        hash: cap[3].to_string(),
                    };
                    return Some(entry);
                }
            }
        }

        None
    }
}

pub fn problem() {
    let mut sector_id_sum = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        match Entry::parse_entry(&sline) {
            Some(entry) => {
                if entry.valid() {
                    // only sum sector ids of valid entries
                    sector_id_sum += entry.sector_id;
                }
            }
            None => {
                println!("Invalid entry line: {}", sline);
            }
        }
    }

    println!("Sector id sum: {}", sector_id_sum);
}

#[test]
fn test_parse_entry() {
    let check_valid = |s, expect_valid| {
        let entry = Entry::parse_entry(s);
        assert!(entry.is_some());
        assert_eq!(expect_valid, entry.unwrap().valid());
    };

    check_valid("aaaaa-bbb-z-y-x-123[abxyz]", true);
    check_valid("a-b-c-d-e-f-g-h-987[abcde]", true);
    check_valid("not-a-real-room-404[oarel]", true);
    check_valid("totally-real-room-200[decoy]", false);
}

#[test]
fn test_calculate_checksum() {
    assert_eq!("abxyz", Entry::calculate_checksum("aaaaa-bbb-z-y-x"));
    assert_eq!("abcde", Entry::calculate_checksum("a-b-c-d-e-f-g-h"));
    assert_eq!("ab", Entry::calculate_checksum("aa-bbb-aa"));
    assert_eq!("ba", Entry::calculate_checksum("aa-bb-bbb-aa"));
}
