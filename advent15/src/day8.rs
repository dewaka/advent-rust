// https://adventofcode.com/2015/day/8
/*
--- Day 8: Matchsticks ---

Space on the sleigh is limited this year, and so Santa will be bringing his list
as a digital copy. He needs to know how much space it will take up when stored.

It is common in many programming languages to provide a way to escape special
characters in strings. For example, C, JavaScript, Perl, Python, and even PHP
handle special characters in very similar ways.

However, it is important to realize the difference between the number of
characters in the code representation of the string literal and the number of
characters in the in-memory string itself.

For example:

- "" is 2 characters of code (the two double quotes), but the string contains zero
  characters.
- "abc" is 5 characters of code, but 3 characters in the string data.
- "aaa\"aaa" is 10 characters of code, but the string itself contains six "a"
  characters and a single, escaped quote character, for a total of 7 characters in
  the string data.
- "\x27" is 6 characters of code, but the string itself contains just one - an
  apostrophe ('), escaped using hexadecimal notation.

Santa's list is a file that contains many double-quoted string literals, one on
each line. The only escape sequences used are \\ (which represents a single
backslash), \" (which represents a lone double-quote character), and \x plus two
hexadecimal characters (which represents a single character with that ASCII
code).

Disregarding the whitespace in the file, what is the number of characters of
code for string literals minus the number of characters in memory for the values
of the strings in total for the entire file?

For example, given the four strings above, the total number of characters of
string code (2 + 5 + 10 + 6 = 23) minus the total number of characters in memory
for string values (0 + 3 + 7 + 1 = 11) is 23 - 11 = 12.
*/

use std::io::{self, BufRead};

fn count_difference(s: &String) -> i32 {
    let bs = s.to_owned().into_bytes();
    bs.len() as i32 - count_in_memory(&bs) as i32
}

fn count_in_memory_str(s: &String) -> usize {
    count_in_memory(&s.to_owned().into_bytes())
}

fn count_in_memory(s: &Vec<u8>) -> usize {
    let mut count: usize = 0;
    let n = s.len();
    let mut i = 0;

    while i < n {
        if s[i] == '\\' as u8 {
            // if form is \xAB where AB are octal numbers
            if i + 1 < n && s[i + 1] == 'x' as u8 {
                // advance 3 characters
                i += 3;
            } else {
                // otherwise it should be either \" or \\. so, advance just one
                i += 1;
            }

            count += 1;
        } else if s[i] != '"' as u8 {
            count += 1;
        }

        i += 1;
    }
    count
}

fn test1() {
    fn print_len(s: &String) {
        let n = count_in_memory(&s.to_owned().into_bytes());
        println!("{} -> {}", s, n);
    }

    print_len(&"\"\\x27\"".to_owned());
    print_len(&"\"\"".to_owned());
    print_len(&"\"abc\"".to_owned());
    print_len(&"\"aaa\\\"aaa\"".to_owned());
}

pub fn problem() {
    println!("2015, day 8");

    let mut sum = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        sum += count_difference(&sline);
    }

    println!("Total difference: {}", sum);
}

#[test]
fn test_example() {
    assert_eq!(count_in_memory_str(&"\"\\x27\"".to_owned()), 1);
    assert_eq!(count_in_memory_str(&"\"\"".to_owned()), 0);
    assert_eq!(count_in_memory_str(&"\"abc\"".to_owned()), 3);
    assert_eq!(count_in_memory_str(&"\"aaa\\\"aaa\"".to_owned()), 7);
}
