// https://adventofcode.com/2015/day/16
/*
--- Day 16: Aunt Sue ---

Your Aunt Sue has given you a wonderful gift, and you'd like to send her a thank
you card. However, there's a small problem: she signed it "From, Aunt Sue".

You have 500 Aunts named "Sue".

So, to avoid sending the card to the wrong person, you need to figure out which
Aunt Sue (which you conveniently number 1 to 500, for sanity) gave you the gift.
You open the present and, as luck would have it, good ol' Aunt Sue got you a My
First Crime Scene Analysis Machine! Just what you wanted. Or needed, as the case
may be.

The My First Crime Scene Analysis Machine (MFCSAM for short) can detect a few
specific compounds in a given sample, as well as how many distinct kinds of
those compounds there are. According to the instructions, these are what the
MFCSAM can detect:

- children, by human DNA age analysis.
- cats. It doesn't differentiate individual breeds.
- Several seemingly random breeds of dog: samoyeds, pomeranians, akitas, and vizslas.
- goldfish. No other kinds of fish.
- trees, all in one group.
- cars, presumably by exhaust or gasoline or something.
- perfumes, which is handy, since many of your Aunts Sue wear a few kinds.

In fact, many of your Aunts Sue have many of these. You put the wrapping from
the gift into the MFCSAM. It beeps inquisitively at you a few times and then
prints out a message on ticker tape:

children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1

You make a list of the things you can remember about each Aunt Sue. Things
missing from your list aren't zero - you simply don't remember the value.

What is the number of the Sue that got you the gift?
*/

extern crate regex;

use self::regex::Regex;
use std::io::{self, BufRead};
use std::collections::HashMap;

type Info = HashMap<String, i32>;

#[derive(Debug)]
struct Aunt {
    name: String,
    info: Info,
}

fn check_compatibility(aunt: &Aunt, check: &Info) -> bool {
    for (key, val) in &aunt.info {
        if let Some(check_val) = check.get(key) {
            if check_val != val {
                return false;
            }
        }
    }

    true
}

fn eliminate_non_matching(info: &Info, aunts: &mut Vec<Aunt>) {
    aunts.retain(|aunt| check_compatibility(&aunt, info));
}

fn parse_info(s: &String) -> Option<Info> {
    lazy_static! {
        static ref R_PROPERTY: Regex = Regex::new(r"(\w+): (\d+)").unwrap();
    }

    if R_PROPERTY.is_match(s) {
        let mut props_map = Info::new();

        for cap in R_PROPERTY.captures_iter(s) {
            let prop_name = cap[1].to_owned();
            let prop_value = cap[2].to_owned();

            if let Ok(val) = prop_value.parse::<i32>() {
                props_map.insert(prop_name, val);
            }
        }

        return Some(props_map);
    }

    None
}

fn parse_aunt(s: &String) -> Option<Aunt> {
    if let Some(idx) = s.find(':') {
        let name: String = s.chars().take(idx).collect();
        let rest: String = s.chars().skip(idx + 1).collect();

        if let Some(props_map) = parse_info(&rest) {
            let aunt = Aunt {
                name: name,
                info: props_map,
            };

            return Some(aunt);
        }
    } else {
        println!("Error: No match");
    }

    None
}

pub fn problem() {
    let mut aunt_info: Vec<Aunt> = vec![];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        if let Some(aunt) = parse_aunt(&sline) {
            aunt_info.push(aunt);
        } else {
            println!("Invalid aunt info: {}", sline);
        }
    }

    let match_info =
        parse_info(&"children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1".to_owned()).unwrap();

    eliminate_non_matching(&match_info, &mut aunt_info);
    println!("*** Matching Aunts ***");
    for a in aunt_info {
        println!("{:?}", a);
    }
}

#[test]
fn test_parse_aunt() {
    let aunt_op = parse_aunt(&"Sue 8: perfumes: 7, children: 2, cats: 1".to_owned());
    assert!(aunt_op.is_some());
    let aunt = aunt_op.unwrap();
    assert_eq!(aunt.name, "Sue 8");
}

#[test]
fn test_parse_info() {
    let match_info =
        parse_info(&"children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1".to_owned());

    assert!(match_info.is_some());
    // There should be 8 properties
    assert_eq!(match_info.unwrap().len(), 10);
}
