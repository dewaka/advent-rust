// https://adventofcode.com/2016/day/7
/*
--- Day 7: Internet Protocol Version 7 ---

While snooping around the local network of EBHQ, you compile a list of IP
addresses (they're IPv7, of course; IPv6 is much too limited). You'd like to
figure out which IPs support TLS (transport-layer snooping).

An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or ABBA. An
ABBA is any four-character sequence which consists of a pair of two different
characters followed by the reverse of that pair, such as xyyx or abba. However,
the IP also must not have an ABBA within any hypernet sequences, which are
contained by square brackets.

For example:

- abba[mnop]qrst supports TLS (abba outside square brackets).
- abcd[bddb]xyyx does not support TLS (bddb is within square brackets, even
  though xyyx is outside square brackets).
- aaaa[qwer]tyui does not support TLS (aaaa is invalid; the interior characters
  must be different).
- ioxxoj[asdfgh]zxcvbn supports TLS (oxxo is outside square brackets, even though
  it's within a larger string).

How many IPs in your puzzle input support TLS?
*/

use std::io::{self, BufRead};

#[derive(Debug)]
struct IP {
    ant_seq: Vec<String>,      // ABBA annotations
    hypernet_seq: Vec<String>, // Hypernets
}

impl IP {
    fn has_valid_annotation(s: &str) -> bool {
        if s.len() < 4 {
            return false;
        }

        let cs: Vec<char> = s.chars().collect();
        for i in 0..cs.len() - 3 {
            let x: char = cs[i];
            let y: char = cs[i + 1];
            let y1: char = cs[i + 2];
            let x1: char = cs[i + 3];

            if (x != y) && (x == x1 && y == y1) {
                return true;
            }
        }

        false
    }

    pub fn is_valid(&self) -> bool {
        // There should be at least one valid annotation in ant_seq section and none in hypernet sections
        let has_valid_ant = self.ant_seq.iter().any(|ant| IP::has_valid_annotation(ant));
        let ant_in_hyper = self.hypernet_seq
            .iter()
            .any(|hyp| IP::has_valid_annotation(hyp));

        has_valid_ant && !ant_in_hyper
    }

    pub fn from_components(ants: Vec<String>, hypers: Vec<String>) -> IP {
        IP {
            ant_seq: ants,
            hypernet_seq: hypers,
        }
    }

    pub fn from_string(ipstr: &str) -> Option<IP> {
        let mut ants: Vec<String> = vec![];
        let mut hypers: Vec<String> = vec![];

        let mut current_ant = String::new();
        let mut current_hyper = String::new();
        let mut hyper_section = false;

        for c in ipstr.chars() {
            match c {
                '[' => {
                    // start of a hypernet section
                    hyper_section = true;

                    // Add the current ant section if it is not empty
                    if !current_ant.is_empty() {
                        ants.push(current_ant.to_owned());
                        current_ant.clear(); // clear for a new ant
                    }
                }
                ']' => {
                    // end of a hypernet section
                    hyper_section = false;

                    // Add the current hyper section if it is not empty
                    if !current_hyper.is_empty() {
                        hypers.push(current_hyper.to_owned());
                        current_hyper.clear(); // clear for a new hyper
                    }
                }
                _ => {
                    // normal character
                    if hyper_section {
                        current_hyper.push(c);
                    } else {
                        current_ant.push(c);
                    }
                }
            }
        }

        // We can have an ant section which we have not pushed in yet
        if !current_ant.is_empty() {
            ants.push(current_ant.to_owned());
        }

        if ants.len() > 0 && hypers.len() > 0 {
            Some(IP::from_components(ants, hypers))
        } else {
            None
        }
    }
}

pub fn problem() {
    let mut valid_count = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        if let Some(ip) = IP::from_string(&sline) {
            if ip.is_valid() {
                valid_count += 1;
            }
        } else {
            println!("Warning: {} is not a valid IP", sline);
        }
    }

    println!("There are {} valid IPs", valid_count);
}

#[test]
fn test_ip_from_string() {
    let ip_opt = IP::from_string("abba[mnop]qrst");
    assert!(ip_opt.is_some());

    let ip = ip_opt.unwrap();
    assert_eq!(2, ip.ant_seq.len());
    assert_eq!(1, ip.hypernet_seq.len());
}

#[test]
fn test_valid_ip() {
    // These are valid IPs
    assert!(IP::from_string("abba[mnop]qrst").unwrap().is_valid());
    assert!(IP::from_string("ioxxoj[asdfgh]zxcvbn").unwrap().is_valid());

    // This is not a valid IP
    assert!(!IP::from_string("abcd[bddb]xyyx").unwrap().is_valid());
    assert!(!IP::from_string("aaaa[qwer]tyui").unwrap().is_valid());
}

#[test]
fn test_valid_annotation() {
    // Some valid annotations
    assert!(IP::has_valid_annotation("abba"));
    assert!(IP::has_valid_annotation("ioxxoj"));
    assert!(IP::has_valid_annotation("this is long but valid ABBA OK"));

    // Some invalid annotations
    assert!(!IP::has_valid_annotation(""));
    assert!(!IP::has_valid_annotation("aaaa"));
    assert!(!IP::has_valid_annotation("XXYY"));
    assert!(!IP::has_valid_annotation("XxyY"));
    assert!(!IP::has_valid_annotation("aaaa"));
}
