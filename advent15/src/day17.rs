// https://adventofcode.com/2015/day/17
/*
--- Day 17: No Such Thing as Too Much ---

The elves bought too much eggnog again - 150 liters this time. To fit it all
into your refrigerator, you'll need to move it into smaller containers. You take
an inventory of the capacities of the available containers.

For example, suppose you have containers of size 20, 15, 10, 5, and 5 liters. If
you need to store 25 liters, there are four ways to do it:

15 and 10
20 and 5 (the first 5)
20 and 5 (the second 5)
15, 5, and 5

Filling all containers entirely, how many different combinations of containers
can exactly fit all 150 liters of eggnog?
*/
use std::io::{self, BufRead};

/*
What is the most Rusty way to following Haskell function?

divide_up n bs
  | n < 0     = 0
  | n == 0    = 1
  | otherwise = case bs of
                 [] -> 0
                 (h:ts) -> divide_up (n - h) ts + divide_up n ts
*/
fn count_ways(num: i32, containers: &[i32]) -> i32 {
    if num == 0 {
        return 1;
    }

    if num < 0 || containers.is_empty() {
        return 0;
    }

    let (head, tail) = containers.split_at(1);
    count_ways(num - head[0], tail) + count_ways(num, tail)
}

pub fn problem() {
    let mut containers: Vec<i32> = vec![];
    let amount = 150;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        if let Ok(c) = sline.parse::<i32>() {
            containers.push(c);
        }
    }

    println!("Containers: {:?}", containers);
    let ways = count_ways(amount, &containers);
    println!("There are {} ways to put {} into containers", ways, amount);
}

#[test]
fn test_count_ways_example() {
    let ways = count_ways(25, &vec![20, 15, 10, 5, 5]);
    assert_eq!(4, ways);
}
