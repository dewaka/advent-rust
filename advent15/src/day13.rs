// https://adventofcode.com/2015/day/13
/*
--- Day 13: Knights of the Dinner Table ---

In years past, the holiday feast with your family hasn't gone so well. Not
everyone gets along! This year, you resolve, will be different. You're going to
find the optimal seating arrangement and avoid all those awkward conversations.

You start by writing up a list of everyone invited and the amount their
happiness would increase or decrease if they were to find themselves sitting
next to each other person. You have a circular table that will be just big
enough to fit everyone comfortably, and so each person will have exactly two
neighbors.

For example, suppose you have only four attendees planned, and you calculate
their potential happiness as follows:

Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.

Then, if you seat Alice next to David, Alice would lose 2 happiness units
(because David talks so much), but David would gain 46 happiness units (because
Alice is such a good listener), for a total change of 44.

If you continue around the table, you could then seat Bob next to Alice (Bob
gains 83, Alice gains 54). Finally, seat Carol, who sits next to Bob (Carol
gains 60, Bob loses 7) and David (Carol gains 55, David gains 41). The
arrangement looks like this:

     +41 +46
+55   David    -2
Carol       Alice
+60    Bob    +54
     -7  +83

After trying every other seating arrangement in this hypothetical scenario, you
find that this one is the most optimal, with a total change in happiness of 330.

What is the total change in happiness for the optimal seating arrangement of the
actual guest list?
*/

extern crate permutohedron;
extern crate regex;

use std::io::{self, BufRead};
use std::collections::HashMap;
use self::regex::Regex;
use self::permutohedron::heap_recursive;

type Pairing = (String, String);
type HappinessInfo = (Pairing, i32);
type HappinessMap = HashMap<Pairing, i32>;

fn add_to_happiness_map(map: &mut HappinessMap, info: &HappinessInfo) {
    let &(ref pair, ref val) = info;
    map.insert(pair.to_owned(), *val);
}

#[allow(dead_code)]
fn print_map(map: &HappinessMap) {
    for (&(ref a, ref b), v) in map {
        if *v > 0 {
            println!(
                "{} would gain {} hapiness units by sitting next to {}.",
                a, v, b
            );
        } else {
            println!(
                "{} would lose {} hapiness units by sitting next to {}.",
                a, v, b
            );
        }
    }
}

fn get_happiness(map: &HappinessMap, p: &Pairing) -> i32 {
    match map.get(p) {
        Some(val) => *val,
        None => 0,
    }
}

fn minimum_total(map: &HappinessMap) -> Option<i32> {
    let mut people: Vec<String> = vec![];
    let mut phash: HashMap<String, String> = HashMap::new();

    for (&(ref p1, ref p2), _) in map {
        phash.insert(p1.to_owned(), p1.to_owned());
        phash.insert(p2.to_owned(), p2.to_owned());
    }

    for p in phash.keys() {
        people.push(p.to_owned());
    }

    let mut total: Option<i32> = None;

    heap_recursive(&mut people, |people_perm| {
        let new_total = calculate_happiness_total(map, &people_perm.to_vec());

        match total {
            Some(cur) => {
                if cur < new_total {
                    total = Some(new_total);
                }
            }
            None => {
                total = Some(new_total);
            }
        }
    });

    total
}

fn calculate_happiness_total(map: &HappinessMap, people: &Vec<String>) -> i32 {
    let mut total: i32 = 0;
    let mut left: String;
    let mut right: String;

    if people.len() <= 2 {
        return total;
    }

    for i in 0..people.len() {
        let person = &people[i];

        if i == 0 {
            left = people[people.len() - 1].to_owned();
            right = people[1].to_owned();
        } else if i + 1 == people.len() {
            left = people[i - 1].to_owned();
            right = people[0].to_owned();
        } else {
            left = people[i - 1].to_owned();
            right = people[i + 1].to_owned();
        }

        total += get_happiness(map, &(person.to_owned(), left));
        total += get_happiness(map, &(person.to_owned(), right));
    }

    total
}

fn parse_happiness_desc(s: &String) -> Option<HappinessInfo> {
    lazy_static! {
        static ref R_GAIN: Regex =
            Regex::new(r"(\w+) would gain (\d+) happiness units by sitting next to (\w+).").unwrap();

        static ref R_LOSE: Regex =
            Regex::new(r"(\w+) would lose (\d+) happiness units by sitting next to (\w+).").unwrap();
    }

    if R_GAIN.is_match(s) {
        for cap in R_GAIN.captures_iter(s) {
            let person1: String = cap[1].to_owned();
            let value: String = cap[2].to_owned();
            let person2: String = cap[3].to_owned();

            match value.parse::<i32>() {
                Ok(num) => {
                    let info: HappinessInfo = ((person1, person2), num);
                    return Some(info);
                }
                Err(_) => return None,
            }
        }
    }

    if R_LOSE.is_match(s) {
        for cap in R_LOSE.captures_iter(s) {
            let person1: String = cap[1].to_owned();
            let value: String = cap[2].to_owned();
            let person2: String = cap[3].to_owned();

            match value.parse::<i32>() {
                Ok(num) => {
                    // Negative value for happiness
                    let info: HappinessInfo = ((person1, person2), -num);
                    return Some(info);
                }
                Err(_) => return None,
            }
        }
    }

    None
}

pub fn problem() {
    let mut map = HappinessMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        match parse_happiness_desc(&sline) {
            Some(desc) => {
                add_to_happiness_map(&mut map, &desc);
            }
            None => {
                println!("Failed to parse: {}", sline);
            }
        }
    }

    println!("Optimal total: {}", minimum_total(&map).unwrap());
}

#[test]
fn test_parsing_happiness_desc() {
    {
        // happiness gain parsing
        let info = parse_happiness_desc(
            &"Alice would gain 54 happiness units by sitting next to Bob.".to_owned(),
        );

        assert!(info.is_some());
    }

    {
        // happiness lose parsing
        let info = parse_happiness_desc(
            &"Alice would lose 79 happiness units by sitting next to Carol.".to_owned(),
        );

        assert!(info.is_some());
    }
}

#[test]
fn test_example_arrangement() {
    let mut map = HappinessMap::new();

    {
        let alice_bob = (("Alice".to_owned(), "Bob".to_owned()), 54);
        let alice_carol = (("Alice".to_owned(), "Carol".to_owned()), -79);
        let alice_david = (("Alice".to_owned(), "David".to_owned()), -2);
        add_to_happiness_map(&mut map, &alice_bob);
        add_to_happiness_map(&mut map, &alice_carol);
        add_to_happiness_map(&mut map, &alice_david);
    }

    {
        let bob_alice = (("Bob".to_owned(), "Alice".to_owned()), 83);
        let bob_carol = (("Bob".to_owned(), "Carol".to_owned()), -7);
        let bob_david = (("Bob".to_owned(), "David".to_owned()), -63);
        add_to_happiness_map(&mut map, &bob_alice);
        add_to_happiness_map(&mut map, &bob_carol);
        add_to_happiness_map(&mut map, &bob_david);
    }

    {
        let carol_alice = (("Carol".to_owned(), "Alice".to_owned()), -62);
        let carol_bob = (("Carol".to_owned(), "Bob".to_owned()), 60);
        let carol_david = (("Carol".to_owned(), "David".to_owned()), 55);
        add_to_happiness_map(&mut map, &carol_alice);
        add_to_happiness_map(&mut map, &carol_bob);
        add_to_happiness_map(&mut map, &carol_david);
    }

    {
        let david_alice = (("David".to_owned(), "Alice".to_owned()), 46);
        let david_bob = (("David".to_owned(), "Bob".to_owned()), -7);
        let david_carol = (("David".to_owned(), "Carol".to_owned()), 41);
        add_to_happiness_map(&mut map, &david_alice);
        add_to_happiness_map(&mut map, &david_bob);
        add_to_happiness_map(&mut map, &david_carol);
    }

    assert_eq!(
        330,
        calculate_happiness_total(
            &map,
            &vec![
                "David".to_owned(),
                "Alice".to_owned(),
                "Bob".to_owned(),
                "Carol".to_owned(),
            ]
        )
    );
}
