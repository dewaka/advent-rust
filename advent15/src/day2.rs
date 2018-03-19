// https://adventofcode.com/2015/day/2
use std::cmp;
use std::io::{self, BufRead};

struct Dimensions {
    width: i32,
    height: i32,
    length: i32,
}

fn surface_area(d: &Dimensions) -> i32 {
    2 * d.length * d.width + 2 * d.width * d.height + 2 * d.height * d.length
}

fn slack(d: &Dimensions) -> i32 {
    let top = d.length * d.width;
    let front = d.width * d.height;
    let side = d.length * d.height;

    // slack is the minimum surface area of above areas
    cmp::min(top, cmp::min(side, front))
}

fn required_paper(d: &Dimensions) -> i32 {
    surface_area(d) + slack(d)
}

fn parse_dimension(ds: &String) -> Result<Dimensions, String> {
    let comps: Vec<&str> = ds.split("x").collect();
    if comps.len() != 3 {
        return Err("invalid dimensions - 3 components required".to_owned());
    }

    match comps[0].parse::<i32>() {
        Ok(w) => match comps[1].parse::<i32>() {
            Ok(l) => match comps[2].parse::<i32>() {
                Ok(h) => Ok(Dimensions {
                    width: w,
                    height: h,
                    length: l,
                }),
                Err(_) => Err("parsing height failed".to_owned()),
            },
            Err(_) => Err("parsing length failed".to_owned()),
        },
        Err(_) => Err("parsing width failed".to_owned()),
    }
}

fn compute_aggregate(ds: &Vec<String>) -> i32 {
    let mut sum = 0;

    for line in ds {
        match parse_dimension(line) {
            Ok(d) => sum += required_paper(&d),
            Err(msg) => println!("error '{}' parsing {}", msg, line),
        }
    }

    sum
}

pub fn problem() {
    let stdin = io::stdin();
    let mut ds: Vec<String> = vec![];

    for ln in stdin.lock().lines() {
        ds.push(ln.unwrap());
    }

    let agg = compute_aggregate(&ds);
    println!("Total required: {}", agg);
}

#[test]
fn test_examples() {
    let d1 = Dimensions {
        width: 2,
        height: 3,
        length: 4,
    };

    let d2 = Dimensions {
        width: 1,
        height: 1,
        length: 10,
    };

    assert_eq!(58, required_paper(&d1));
    assert_eq!(43, required_paper(&d2));
}

#[test]
fn test_aggregate() {
    let d1 = Dimensions {
        width: 2,
        height: 3,
        length: 4,
    };

    let d2 = Dimensions {
        width: 1,
        height: 1,
        length: 10,
    };

    let expected = required_paper(&d1) + required_paper(&d2);

    assert_eq!(
        expected,
        compute_aggregate(&vec!["2x3x4".to_owned(), "1x1x10".to_owned()])
    )
}
