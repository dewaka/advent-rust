// https://adventofcode.com/2015/day/15
/*
--- Day 15: Science for Hungry People ---

Today, you set out on the task of perfecting your milk-dunking cookie recipe.
All you have to do is find the right balance of ingredients.

Your recipe leaves room for exactly 100 teaspoons of ingredients. You make a
list of the remaining ingredients you could use to finish the recipe (your
puzzle input) and their properties per teaspoon:

- capacity (how well it helps the cookie absorb milk)
- durability (how well it keeps the cookie intact when full of milk)
- flavor (how tasty it makes the cookie)
- texture (how it improves the feel of the cookie)
- calories (how many calories it adds to the cookie)

You can only measure ingredients in whole-teaspoon amounts accurately, and you
have to be accurate so you can reproduce your results in the future. The total
score of a cookie can be found by adding up each of the properties (negative
totals become 0) and then multiplying together everything except calories.

For instance, suppose you have these two ingredients:

Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3

Then, choosing to use 44 teaspoons of butterscotch and 56 teaspoons of cinnamon
(because the amounts of each ingredient must add up to 100) would result in a
cookie with the following properties:

- A capacity of 44*-1 + 56*2 = 68
- A durability of 44*-2 + 56*3 = 80
- A flavor of 44*6 + 56*-2 = 152
- A texture of 44*3 + 56*-1 = 76

Multiplying these together (68 * 80 * 152 * 76, ignoring calories for now)
results in a total score of 62842880, which happens to be the best score
possible given these ingredients. If any properties had produced a negative
total, it would have instead become zero, causing the whole score to multiply to
zero.

Given the ingredients in your kitchen and their properties, what is the total
score of the highest-scoring cookie you can make?
*/
extern crate regex;

use std::io::{self, BufRead};
use self::regex::Regex;

#[derive(Debug, Clone)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

// Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3
fn parse_ingredient(s: &str) -> Option<Ingredient> {
    lazy_static! {
        static ref R_ING: Regex =
            Regex::new(r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
    }

    if R_ING.is_match(s) {
        for cap in R_ING.captures_iter(s) {
            let name = cap[1].to_owned();
            let capa = cap[2].to_owned();
            let dur = cap[3].to_owned();
            let fla = cap[4].to_owned();
            let tex = cap[5].to_owned();
            let cal = cap[6].to_owned();

            if let Ok(capacity) = cap[2].to_owned().parse::<i32>() {
                if let Ok(durability) = cap[3].to_owned().parse::<i32>() {
                    if let Ok(flavor) = cap[4].to_owned().parse::<i32>() {
                        if let Ok(texture) = cap[5].to_owned().parse::<i32>() {
                            if let Ok(calories) = cap[6].to_owned().parse::<i32>() {
                                let ing = Ingredient {
                                    name: cap[1].to_owned(),
                                    capacity: capacity,
                                    durability: durability,
                                    flavor: flavor,
                                    texture: texture,
                                    calories: calories,
                                };

                                return Some(ing);
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn calculate_total(ingredients: &Vec<Ingredient>, amounts: &Vec<i32>) -> Option<i32> {
    if ingredients.len() != amounts.len() {
        return None;
    }

    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut _calories = 1;

    for (ing, amount) in ingredients.iter().zip(amounts.iter()) {
        capacity += ing.capacity * amount;
        durability += ing.durability * amount;
        flavor += ing.flavor * amount;
        texture += ing.texture * amount;
        // _calories += ing.calories * amount;
    }

    if capacity < 0 {
        capacity = 0;
    }
    if durability < 0 {
        durability = 0;
    }
    if flavor < 0 {
        flavor = 0;
    }
    if texture < 0 {
        texture = 0;
    }
    if _calories < 0 {
        _calories = 0;
    }

    Some(capacity * durability * flavor * texture * _calories)
}

// How to divide given num between given slots
fn ration(num: i32, slots: i32) -> Vec<Vec<i32>> {
    if num < 0 {
        return vec![];
    }
    if slots <= 1 {
        return vec![vec![num]];
    } else {
        let mut rs: Vec<Vec<i32>> = vec![];

        for n in 0..num + 1 {
            let mut rss = ration(num - n, slots - 1);

            for rsss in rss {
                let mut rssss = rsss;
                rssss.push(n);
                rs.push(rssss);
            }
        }

        return rs;
    }
}

fn test_rations() {
    let mut rs = ration(10, 2);
    println!("Rations (2): {:?}", rs);
    rs = ration(10, 3);
    println!("Rations (3): {:?}", rs);
}

fn calculate_optimal(ingredients: &Vec<Ingredient>, amount: i32) -> Option<(i32, Vec<i32>)> {
    let mut max_amount: Option<i32> = None;
    let mut max_rations: Option<Vec<i32>> = None;

    for rs in ration(amount, ingredients.len() as i32) {
        let total = calculate_total(ingredients, &rs);

        match total {
            Some(val) => {
                if max_amount == None {
                    max_amount = Some(val);
                    max_rations = Some(rs);
                } else if val > max_amount.unwrap() {
                    max_amount = Some(val);
                    max_rations = Some(rs);
                }
            }
            None => continue,
        }
    }

    match max_amount {
        Some(val) => Some((val, max_rations.unwrap())),
        None => None,
    }
}

pub fn problem() {
    let mut ings : Vec<Ingredient> = vec![];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        if let Some(ing) = parse_ingredient(&sline) {
            ings.push(ing);
        } else {
            println!("Failed to parse: {}", sline);
        }
    }

    let optimal = calculate_optimal(&ings, 100);
    println!("There are {} ingredients", ings.len());
    println!("Optimal: {:?}", optimal);
}

#[test]
fn test_calculate_example() {
    let butterscotch = Ingredient {
        name: "Butterscotch".to_owned(),
        capacity: -1,
        durability: -2,
        flavor: 6,
        texture: 3,
        calories: 8,
    };

    let cinnamon = Ingredient {
        name: "Cinnamon".to_owned(),
        capacity: 2,
        durability: 3,
        flavor: -2,
        texture: -1,
        calories: 3,
    };

    let ingredients = vec![butterscotch, cinnamon];

    let total = calculate_total(&ingredients, &vec![44, 56]);
    assert_eq!(62842880, total.unwrap_or(0));
}

#[test]
fn test_calculate_optimal_example() {
    let butterscotch = Ingredient {
        name: "Butterscotch".to_owned(),
        capacity: -1,
        durability: -2,
        flavor: 6,
        texture: 3,
        calories: 8,
    };

    let cinnamon = Ingredient {
        name: "Cinnamon".to_owned(),
        capacity: 2,
        durability: 3,
        flavor: -2,
        texture: -1,
        calories: 3,
    };

    let ingredients = vec![butterscotch, cinnamon];

    if let Some((opt_val, opt_rations)) = calculate_optimal(&ingredients, 100) {
        assert_eq!(62842880, opt_val);
        assert_eq!(opt_rations, vec![44, 56]);
    } else {
        assert!(false);
    }
}

#[test]
fn test_parsing_ingredients() {
    assert!(
        parse_ingredient("Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3")
            .is_some()
    );

    assert!(
        parse_ingredient(
            "Butterscotch: capacity 0, durability 5, flavor -3, texture 0, calories 3"
        ).is_some()
    );

    assert!(
        parse_ingredient("Chocolate: capacity 0, durability 0, flavor 5, texture -1, calories 8")
            .is_some()
    );

    assert!(
        parse_ingredient("Candy: capacity 0, durability -1, flavor 0, texture 5, calories 8")
            .is_some()
    );
}
