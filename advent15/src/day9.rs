// https://adventofcode.com/2015/day/9
/*
--- Day 9: All in a Single Night ---
Every year, Santa manages to deliver all of his presents in a single night.

This year, however, he has some new locations to visit; his elves have provided
him the distances between every pair of locations. He can start and end at any
two (different) locations he wants, but he must visit each location exactly
once. What is the shortest distance he can travel to achieve this?

For example, given the following distances:

London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
The possible routes are therefore:

Dublin -> London -> Belfast = 982
London -> Dublin -> Belfast = 605
London -> Belfast -> Dublin = 659
Dublin -> Belfast -> London = 659
Belfast -> Dublin -> London = 605
Belfast -> London -> Dublin = 982

The shortest of these is London -> Dublin -> Belfast = 605, and so the answer is
605 in this example.

What is the distance of the shortest route?
*/

extern crate regex;

use std::io::{self, BufRead};
use std::collections::HashMap;
use self::regex::Regex;

type Distance = i32;
type CityPair = (String, String);
type DistanceMap = HashMap<CityPair, Distance>;

fn distance(map: &DistanceMap, from: &String, to: &String) -> Option<Distance> {
    match map.get(&(from.to_owned(), to.to_owned())) {
        Some(d) => Some(*d),
        None => None,
    }
}

fn parse_distance_spec(s: &String) -> Option<(CityPair, Distance)> {
    lazy_static! {
        static ref R_DISTANCE_SPEC: Regex = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    }

    if R_DISTANCE_SPEC.is_match(s) {
        for cap in R_DISTANCE_SPEC.captures_iter(s) {
            let city1: String = cap[1].to_owned();
            let city2: String = cap[2].to_owned();

            match cap[3].parse::<Distance>() {
                Ok(d) => return Some(((city1, city2), d)),
                Err(_) => return None,
            }
        }
    }

    None
}

fn update_distance_map(map: &mut DistanceMap, s: &String) -> bool {
    match parse_distance_spec(s) {
        Some(((c1, c2), d)) => {
            map.insert((c1.to_owned(), c2.to_owned()), d);
            map.insert((c2.to_owned(), c1.to_owned()), d);
            true
        }
        None => false,
    }
}

fn test_permutations() {
    let v = vec![1, 2, 3, 4];
    for (i, el1) in v.iter().enumerate() {
        for el2 in v.slice_from(i + 1).iter() {
            println!("{}, {}", el1, el2);
        }
    }
}

pub fn problem() {
    println!("2015, day 9");

    test_permutations();
    return;

    let mut distance_map = DistanceMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        if !update_distance_map(&mut distance_map, &sline) {
            println!("Invalid distance spec: {}", sline);
        }
    }

    for (k, v) in distance_map.iter() {
        println!("{:?} -> {}", k, v);
    }
}

#[test]
fn test_parse_distance_spec() {
    match parse_distance_spec(&"London to Dublin = 464".to_owned()) {
        Some(((c1, c2), d)) => {
            assert_eq!("London", c1);
            assert_eq!("Dublin", c2);
            assert_eq!(464, d);
        }
        None => assert!(false),
    }

    match parse_distance_spec(&"London to Belfast = 518".to_owned()) {
        Some(((c1, c2), d)) => {
            assert_eq!("London", c1);
            assert_eq!("Belfast", c2);
            assert_eq!(518, d);
        }
        None => assert!(false),
    }

    match parse_distance_spec(&"Dublin to Belfast = 141".to_owned()) {
        Some(((c1, c2), d)) => {
            assert_eq!("Dublin", c1);
            assert_eq!("Belfast", c2);
            assert_eq!(141, d);
        }
        None => assert!(false),
    }
}
