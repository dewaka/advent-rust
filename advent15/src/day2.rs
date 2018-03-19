// https://adventofcode.com/2015/day/2

use std::cmp;

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

fn print_solution(d: &Dimensions) {
    println!(
        "{}x{}x{} requires {} square feet of paper",
        d.width,
        d.height,
        d.length,
        required_paper(d)
    );
}

pub fn problem() {
    println!("2015, day 2");

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

    print_solution(&d1);
    print_solution(&d2);
}
