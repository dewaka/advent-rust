// https://adventofcode.com/2015/day/12
/*
--- Day 12: JSAbacusFramework.io ---

Santa's Accounting-Elves need help balancing the books after a recent order.
Unfortunately, their accounting software uses a peculiar storage format. That's
where you come in.

They have a JSON document which contains a variety of things: arrays ([1,2,3]),
objects ({"a":1, "b":2}), numbers, and strings. Your first job is to simply find
all of the numbers throughout the document and add them together.

For example:

- [1,2,3] and {"a":2,"b":4} both have a sum of 6.
- [[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
- {"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
- [] and {} both have a sum of 0.

You will not encounter any strings containing numbers.

What is the sum of all numbers in the document?
*/
extern crate serde_json;

use self::serde_json::{Error, Value};
use std::io::{self, BufRead};

fn sum_numbers(v: Value) -> i64 {
    match v {
        Value::Number(n) => {
            if n.is_i64() {
                n.as_i64().unwrap()
            } else if n.is_f64() {
                n.as_f64().unwrap() as i64
            } else if n.is_u64() {
                n.as_u64().unwrap() as i64
            } else {
                0
            }
        }
        Value::Array(vals) => {
            let mut sum: i64 = 0;
            for v in vals {
                sum += sum_numbers(v);
            }
            sum
        }
        Value::Object(map) => {
            let mut sum: i64 = 0;

            for (_, val) in map {
                sum += sum_numbers(val);
            }

            sum
        }
        _ => 0,
    }
}


fn calculate_sum(data: &String) -> Option<i64> {
    match serde_json::from_str(data) {
        Ok(val) => Some(sum_numbers(val)),
        Err(_) => None,
    }
}

pub fn problem() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        match calculate_sum(&sline) {
            Some(num) => println!("Sum is: {}", num),
            None => println!("Error - failed to parse json for: {}", sline),
        }
    }
}

#[test]
fn test_calculate() {
    // Simple array sum
    let data = r#"[1, 2, 3, 4, 5]"#;
    assert_eq!(calculate_sum(&data.to_owned()), Some(15));

    // Nested sum for map
    let data2 = r#"{
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                      "+44 1234567",
                      "+44 2345678"
                    ]
                  }"#;
    assert_eq!(calculate_sum(&data2.to_owned()), Some(43));
}
