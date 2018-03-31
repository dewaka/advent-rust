// https://adventofcode.com/2016/day/3
/*
--- Day 3: Squares With Three Sides ---

Now that you can think clearly, you move deeper into the labyrinth of hallways
and office furniture that makes up this part of Easter Bunny HQ. This must be a
graphic design department; the walls are covered in specifications for
triangles.

Or are they?

The design document gives the side lengths of each triangle it describes, but...
5 10 25? Some of these aren't triangles. You can't help but mark the impossible
ones.

In a valid triangle, the sum of any two sides must be larger than the remaining
side. For example, the "triangle" given above is impossible, because 5 + 10 is
not larger than 25.

In your puzzle input, how many of the listed triangles are possible?
*/
use std::io::{self, BufRead};

fn valid_triangle(x: i32, y: i32, z: i32) -> bool {
    (x + y > z) && (z + y > x) && (x + z > y)
}

fn parse_triangle_spec(s: &str) -> Option<(i32, i32, i32)> {
    let mut nums: Vec<i32> = vec![];

    for ns in s.split(" ") {
        if nums.len() > 3 {
            // we should only have 3 sides
            return None;
        }

        ns.trim();
        if ns.len() == 0 {
            continue;
        }

        if let Ok(num) = ns.parse::<i32>() {
            nums.push(num);
        } else {
            return None;
        }
    }

    if nums.len() == 3 {
        Some((nums[0], nums[1], nums[2]))
    } else {
        None
    }
}

pub fn problem() {
    let mut valid_count = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        match parse_triangle_spec(&sline) {
            Some((x, y, z)) => {
                if valid_triangle(x, y, z) {
                    valid_count += 1;
                }
            }
            None => {
                println!("Invalid triangle spec: {}", sline);
            }
        }
    }

    println!("There are {} valid triangles", valid_count);
}

#[test]
fn test_valid_triangle() {
    // Some invalid ones
    assert!(!valid_triangle(5, 10, 25));
    assert!(!valid_triangle(5, 10, 15));
    assert!(!valid_triangle(10, 5, 15));

    // Valid ones
    assert!(valid_triangle(5, 10, 12));
    assert!(valid_triangle(12, 5, 10));
    assert!(valid_triangle(10, 15, 22));
}

#[test]
fn test_parse_triangle() {
    assert_eq!(Some((1, 2, 3)), parse_triangle_spec("1 2 3"));
    assert_eq!(Some((13, 72, 34)), parse_triangle_spec("13 72 34"));
}
