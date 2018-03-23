// https://adventofcode.com/2015/day/14
/*
--- Day 14: Reindeer Olympics ---

This year is the Reindeer Olympics! Reindeer can fly at high speeds, but must
rest occasionally to recover their energy. Santa would like to know which of his
reindeer is fastest, and so he has them race.

Reindeer can only either be flying (always at their top speed) or resting (not
moving at all), and always spend whole seconds in either state.

For example, suppose you have the following Reindeer:

- Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
- Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.

After one second, Comet has gone 14 km, while Dancer has gone 16 km. After ten
seconds, Comet has gone 140 km, while Dancer has gone 160 km. On the eleventh
second, Comet begins resting (staying at 140 km), and Dancer continues on for a
total distance of 176 km. On the 12th second, both reindeer are resting. They
continue to rest until the 138th second, when Comet flies for another ten
seconds. On the 174th second, Dancer flies for another 11 seconds.

In this example, after the 1000th second, both reindeer are resting, and Comet
is in the lead at 1120 km (poor Dancer has only gotten 1056 km by that point).
So, in this situation, Comet would win (if the race ended at 1000 seconds).

Given the descriptions of each reindeer (in your puzzle input), after exactly
2503 seconds, what distance has the winning reindeer travelled?
*/

extern crate regex;

use std::io::{self, BufRead};
use self::regex::Regex;

#[derive(Debug, Clone)]
struct Reindeer {
    name: String,
    speed: i32,
    run_duration: i32,
    rest_duration: i32,
}

impl Reindeer {
    fn distance_travelled(&self, period: i32) -> i32 {
        let mut p = period;
        let mut distance = 0;

        loop {
            if p < 0 || p < self.run_duration {
                break;
            }

            // p >= self.run_duration so we can run
            distance += self.run_duration * self.speed;
            p -= self.run_duration + self.rest_duration;
        }
        distance
    }

    fn from_description(s: &String) -> Option<Reindeer> {
        lazy_static! {
            static ref R_DESC: Regex =
                Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
        }

        if R_DESC.is_match(s) {
            for cap in R_DESC.captures_iter(s) {
                let name = cap[1].to_owned();
                let str_speed = cap[2].to_owned();
                let str_run_duration = cap[3].to_owned();
                let str_rest_duration = cap[4].to_owned();

                match str_speed.parse::<i32>() {
                    Ok(speed) => match str_run_duration.parse::<i32>() {
                        Ok(run_duration) => match str_rest_duration.parse::<i32>() {
                            Ok(rest_duration) => {
                                let deer = Reindeer {
                                    name: name,
                                    speed: speed,
                                    run_duration: run_duration,
                                    rest_duration: rest_duration,
                                };
                                return Some(deer);
                            }
                            Err(_) => {
                                return None;
                            }
                        },
                        Err(_) => {
                            return None;
                        }
                    },
                    Err(_) => {
                        return None;
                    }
                }
            }
        }

        None
    }
}

fn find_fastest(deers: &Vec<Reindeer>, period: i32) -> Option<(Reindeer, i32)> {
    let mut fastest: Option<(Reindeer, i32)> = None;

    for d in deers.iter() {
        let cur = d.distance_travelled(period);

        match fastest {
            Some((_, dist)) => {
                // if current deer is faster, then set update the fastest
                if cur > dist {
                    fastest = Some((d.clone(), cur));
                }
            }
            None => {
                fastest = Some((d.clone(), cur));
            }
        }
    }

    fastest
}

pub fn problem() {
    let mut deers: Vec<Reindeer> = vec![];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        match Reindeer::from_description(&sline) {
            Some(deer) => {
                deers.push(deer);
            }
            None => {
                println!("Error: failed to parse Reindeer description: {}", sline);
            }
        }
    }

    println!("Deers count: {}", deers.len());

    match find_fastest(&deers, 2503) {
        Some(info) => println!("Fastest: {:?}", info),
        None => println!("Error: couldn't compute fastest Reindeer"),
    }
}

#[test]
fn test_parse_reindeer() {
    {
        let desc = "Dancer can fly 27 km/s for 5 seconds, but then must rest for 132 seconds.";

        let deer_op = Reindeer::from_description(&desc.to_owned());
        assert!(deer_op.is_some());

        let deer = deer_op.unwrap();
        assert_eq!(deer.name, "Dancer");
        assert_eq!(deer.speed, 27);
        assert_eq!(deer.run_duration, 5);
        assert_eq!(deer.rest_duration, 132);
    }
    {
        let desc = "Cupid can fly 22 km/s for 2 seconds, but then must rest for 41 seconds.";

        let deer_op = Reindeer::from_description(&desc.to_owned());
        assert!(deer_op.is_some());

        let deer = deer_op.unwrap();
        assert_eq!(deer.name, "Cupid");
        assert_eq!(deer.speed, 22);
        assert_eq!(deer.run_duration, 2);
        assert_eq!(deer.rest_duration, 41);
    }
}

#[test]
fn test_fastest_computation() {
    let comet = Reindeer {
        name: "Comet".to_owned(),
        speed: 14,
        run_duration: 10,
        rest_duration: 127,
    };

    let dancer = Reindeer {
        name: "Dancer".to_owned(),
        speed: 16,
        run_duration: 11,
        rest_duration: 162,
    };

    assert_eq!(comet.distance_travelled(1000), 1120);
    assert_eq!(dancer.distance_travelled(1000), 1056);

    let fastest = find_fastest(&vec![comet, dancer], 1000);
    assert!(fastest.is_some());
    let (d, _) = fastest.unwrap();
    assert_eq!(d.name, "Comet"); // Comet should be the fastest one
}
