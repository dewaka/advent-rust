// https://adventofcode.com/2015/day/20
/*
--- Day 20: Infinite Elves and Infinite Houses ---

To keep the Elves busy, Santa has them deliver some presents by hand,
door-to-door. He sends them down a street with infinite houses numbered
sequentially: 1, 2, 3, 4, 5, and so on.

Each Elf is assigned a number, too, and delivers presents to houses based on
that number:

- The first Elf (number 1) delivers presents to every house: 1, 2, 3, 4, 5, ....
- The second Elf (number 2) delivers presents to every second house: 2, 4, 6, 8, 10, ....
- Elf number 3 delivers presents to every third house: 3, 6, 9, 12, 15, ....

There are infinitely many Elves, numbered starting with 1. Each Elf delivers
presents equal to ten times his or her number at each house.

So, the first nine houses on the street end up like this:

House 1 got 10 presents.
House 2 got 30 presents.
House 3 got 40 presents.
House 4 got 70 presents.
House 5 got 60 presents.
House 6 got 120 presents.
House 7 got 80 presents.
House 8 got 150 presents.
House 9 got 130 presents.

The first house gets 10 presents: it is visited only by Elf 1, which delivers 1
* 10 = 10 presents. The fourth house gets 70 presents, because it is visited by
Elves 1, 2, and 4, for a total of 10 + 20 + 40 = 70 presents.

What is the lowest house number of the house to get at least as many presents as
the number in your puzzle input?

Your puzzle input is 29000000.
*/

use std::io::{self, BufRead};
use std::collections::HashSet;

fn divisors(house: i64) -> Vec<i64> {
    let mut divs: HashSet<i64> = HashSet::new();

    let bound: i64 = (house as f64).sqrt() as i64 + 1;

    for d in 1..bound {
        if house % d == 0 {
            divs.insert(d);
            divs.insert(house / d);
        }
    }

    divs.into_iter().collect()
}

fn presents_to_house(house: i64) -> i64 {
    divisors(house).iter().fold(0, |sum, val| sum + val * 10)
}

fn min_house_to_get(count: i64) -> (i64, i64) {
    for h in 1.. {
        let presents = presents_to_house(h);
        if presents >= count {
            return (h, presents);
        }
    }

    // We shouldn't be here!
    (0, 0)
}

pub fn problem() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        match sline.parse::<i64>() {
            Ok(count) => {
                let (house, actual_count) = min_house_to_get(count);
                println!(
                    "Minimum house to get at least {} is {} and it gets {} presents",
                    count, house, actual_count
                );
            }
            Err(_) => {
                println!("Invalid number: {}", sline);
            }
        }
    }
}

#[test]
fn test_given_elves_delivery() {
    assert_eq!(10, presents_to_house(1));
    assert_eq!(30, presents_to_house(2));
    assert_eq!(40, presents_to_house(3));
    assert_eq!(70, presents_to_house(4));
    assert_eq!(60, presents_to_house(5));
    assert_eq!(120, presents_to_house(6));
    assert_eq!(80, presents_to_house(7));
}
