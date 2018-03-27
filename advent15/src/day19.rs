// https://adventofcode.com/2015/day/19
/*
--- Day 19: Medicine for Rudolph ---

Rudolph the Red-Nosed Reindeer is sick! His nose isn't shining very brightly,
and he needs medicine.

Red-Nosed Reindeer biology isn't similar to regular reindeer biology; Rudolph is
going to need custom-made medicine. Unfortunately, Red-Nosed Reindeer chemistry
isn't similar to regular reindeer chemistry, either.

The North Pole is equipped with a Red-Nosed Reindeer nuclear fusion/fission
plant, capable of constructing any Red-Nosed Reindeer molecule you need. It
works by starting with some input molecule and then doing a series of
replacements, one per step, until it has the right molecule.

However, the machine has to be calibrated before it can be used. Calibration
involves determining the number of molecules that can be generated in one step
from a given starting point.

For example, imagine a simpler machine that supports only the following
replacements:

H => HO
H => OH
O => HH

Given the replacements above and starting with HOH, the following molecules
could be generated:

HOOH (via H => HO on the first H).
HOHO (via H => HO on the second H).
OHOH (via H => OH on the first H).
HOOH (via H => OH on the second H).
HHHH (via O => HH).

So, in the example above, there are 4 distinct molecules (not five, because HOOH
appears twice) after one replacement from HOH. Santa's favorite molecule,
HOHOHO, can become 7 distinct molecules (over nine replacements: six from H, and
three from O).

The machine replaces without regard for the surrounding characters. For example,
given the string H2O, the transition H => OO would result in OO2O.

Your puzzle input describes all of the possible replacements and, at the bottom,
the medicine molecule for which you need to calibrate the machine. How many
distinct molecules can be created after all the different ways you can do one
replacement on the medicine molecule?
*/

/*
What is an efficient way to implement following algorithm.

unique_replacements hay rs = nub $ all_replacements hay rs

all_replacements hay rs = foldl (++) [] $ [ replacements hay n r | (n, r) <- rs ]

replacements hay needle repl = [(f ++ repl ++ b) | (f, b) <- split_match hay needle]

split_match hay needle = go hay needle []
    where
        go []  _  _                  = []
        go hay [] _                  = [(hay, [])]
        go hay@(h:ts) needle prev
            | starts_with hay needle = let remain = drop (length needle) hay
                                       in (prev, remain) : go ts needle (prev ++ [h])
            | otherwise              = go ts needle (prev ++ [h])

        starts_with _ []           = True
        starts_with [] _           = False
        starts_with (n:ns) (h:hs)  = (n == h) && starts_with ns hs
*/

extern crate regex;

use self::regex::Regex;
use std::io::{self, BufRead};

// type Replacement = (String, String);

fn split_matches(hay: &str, needle: &str) -> Vec<(String, String)> {
    let mut ms: Vec<(String, String)> = vec![];

    fn go(hay: &String, needle: &String, prev: &String, ms: &mut Vec<(String, String)>) {
        if hay.is_empty() && !needle.is_empty() {
            return;
        }
        if needle.is_empty() {
            ms.push((hay.to_owned(), "".to_owned()));
        } else {
            let hs: String = hay.chars().skip(1).collect();
            let h: char = hay.chars().nth(0).unwrap();
            let new_prev = format!("{}{}", prev, h);

            if hay.starts_with(needle) {
                let remain = hay.chars().skip(needle.len()).collect();
                ms.push((prev.to_owned(), remain));
                go(&hs, needle, &new_prev, ms);
            } else {
                go(&hs, needle, &new_prev, ms);
            }
        }
    }

    go(&hay.to_owned(), &needle.to_owned(), &"".to_owned(), &mut ms);

    ms
}

fn replacements(hay: &str, needle: &str, repl: &str, repls: &mut Vec<String>) {
    for (prefix, suffix) in split_matches(hay, needle) {
        repls.push(format!("{}{}{}", prefix, repl, suffix));
    }
}

fn unique_replacements(hay: &str, rep_vec: &Vec<(String, String)>) -> Vec<String> {
    let mut repls: Vec<String> = vec![];

    for &(ref needle, ref rep) in rep_vec {
        replacements(hay, needle, rep, &mut repls);
    }

    // sort the vector and dedup to remove duplicates
    repls.sort();
    repls.dedup();
    repls
}

// By default we expect a string replacement rule. If it does not look like a
// string replacement rule, then we take the given string as simply an input
// string to be run the rules on
fn parse_input(s: &String) -> Result<(String, String), String> {
    lazy_static! {
        static ref R_RULE: Regex = Regex::new(r"(\w+) => (\w+)").unwrap();
    }

    // Check if s matches a rule like <NEEDLE> => <REPL>
    if R_RULE.is_match(s) {
        for cap in R_RULE.captures_iter(s) {
            let needle = cap[1].to_owned();
            let repl = cap[2].to_owned();
            return Ok((needle, repl));
        }
    }

    Err(s.to_owned())
}

pub fn problem() {
    // test_replace();
    // return;

    let mut repls: Vec<(String, String)> = vec![];
    let mut input: String = String::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        // ignore empty lines
        if sline.trim().is_empty() {
            continue;
        }

        match parse_input(&sline.to_owned()) {
            Ok(p) => repls.push(p),
            Err(s) => {
                input = s.to_owned();
                break;
            }
        }
    }

    if input.is_empty() {
        println!("Error: empty input");
        return;
    }
    if repls.is_empty() {
        println!("Error: empty replacement rules");
        return;
    }

    println!("Input is: {}", input);
    println!("The are {} replacement rules", repls.len());

    let unique_reps = unique_replacements(&input, &repls);
    println!("There are {} unique replacements", unique_reps.len());
    println!("Unique replacements: {:?}", unique_reps);
}

#[test]
fn test_split_matches() {
    // Helper to run common tests
    fn test(h: &str, n: &str, expected_count: usize) {
        let matches = split_matches(h, n);

        assert_eq!(matches.len(), expected_count);

        for (f, b) in matches {
            let mut computed: String = f;
            computed.push_str(n);
            computed.push_str(&b);
            assert_eq!(h, computed);
        }
    }

    test("ababa", "aba", 2);
    test("", "aba", 0);
    test("aba", "", 1);
    test("", "", 1);
}

#[test]
fn test_unique_replacements() {
    let reps = vec![
        ("H".to_owned(), "HO".to_owned()),
        ("H".to_owned(), "OH".to_owned()),
        ("O".to_owned(), "HH".to_owned()),
    ];

    let ureps = unique_replacements("HOH", &reps);
    assert_eq!(4, ureps.len());
}
