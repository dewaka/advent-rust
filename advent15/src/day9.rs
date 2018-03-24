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

extern crate permutohedron;
extern crate regex;

use std::io::{self, BufRead};
use std::collections::HashMap;
use self::regex::Regex;
use self::permutohedron::heap_recursive;

type Distance = i32;
type CityPair = (String, String);
type DistanceMap = HashMap<CityPair, Distance>;

#[allow(dead_code)]
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

// Get a vector of unique cities from the distance map
fn get_cities(map: &DistanceMap) -> Vec<String> {
    let mut cities: HashMap<String, String> = HashMap::new();
    let mut cv: Vec<String> = vec![];

    for (&(ref c1, ref c2), _) in map.iter() {
        cities.insert(c1.to_owned(), c1.to_owned());
        cities.insert(c2.to_owned(), c2.to_owned());
    }

    for city in cities.keys() {
        cv.push(city.to_owned());
    }
    cv
}

fn calculate_shortest(map: &DistanceMap, cities: &Vec<String>) -> (Distance, Distance) {
    let mut shortest: Option<Distance> = None;
    let mut longest: Option<Distance> = None;

    let mut mcities: Vec<String> = cities.clone();
    let mut city_permutations = Vec::new();

    heap_recursive(&mut mcities, |city_perm| {
        city_permutations.push(city_perm.to_vec())
    });

    for city_perm in city_permutations {
        let mut sum_distance = 0;
        for city_pair in city_perm.iter().zip(city_perm[1..].iter()) {
            let (c1, c2) = city_pair;
            sum_distance += map.get(&(c1.to_owned(), c2.to_owned())).unwrap_or(&0);
        }

        if shortest == None {
            shortest = Some(sum_distance);
        } else if sum_distance < shortest.unwrap() {
            shortest = Some(sum_distance);
        }

        if longest == None {
            longest = Some(sum_distance);
        } else if sum_distance > longest.unwrap() {
            longest = Some(sum_distance);
        }
    }

    (shortest.unwrap_or(-1), longest.unwrap_or(-1))
}

pub fn problem() {
    println!("2015, day 9");

    // test_permutations();
    // return;

    let mut distance_map = DistanceMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        if !update_distance_map(&mut distance_map, &sline) {
            println!("Invalid distance spec: {}", sline);
        }
    }

    let cities = get_cities(&distance_map);
    let (shortest, longest) = calculate_shortest(&distance_map, &cities);
    println!("Shortest: {}, Longest: {}", shortest, longest);
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

#[allow(dead_code)]
// Playground method to test slices, zippers and permutations in Rust
fn test_permutations() {
    let mut data = vec![1, 2, 3, 4];
    let mut perms = Vec::new();

    heap_recursive(&mut data, |pm| perms.push(pm.to_vec()));

    println!("** Permutations ***");
    for pm in perms {
        println!("{:?}", pm);
    }

    fn use_slices() {
        let a = vec![1, 2, 3, 4, 5];

        // All elements
        println!("a[..] -> {:?}", &a[..]);

        // Rest
        println!("a[1..] -> {:?}", &a[1..]);

        // With a start and an end
        println!("a[1..4] -> {:?}", &a[1..4]);

        // With just a start
        println!("a[2..] -> {:?}", &a[2..]);

        // With just an end
        println!("a[..3] -> {:?}", &a[..3]);
    }

    use_slices();

    let test_data = vec![1, 2, 3, 4, 5, 6, 7];
    for p in test_data.iter().zip(test_data[1..].iter()) {
        println!("{:?}", p);
    }
}
