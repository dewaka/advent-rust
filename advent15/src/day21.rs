// https://adventofcode.com/2015/day/21
/*
--- Day 21: RPG Simulator 20XX ---

Little Henry Case got a new video game for Christmas. It's an RPG, and he's
stuck on a boss. He needs to know what equipment to buy at the shop. He hands
you the controller.

In this game, the player (you) and the enemy (the boss) take turns attacking.
The player always goes first. Each attack reduces the opponent's hit points by
at least 1. The first character at or below 0 hit points loses.

Damage dealt by an attacker each turn is equal to the attacker's damage score
minus the defender's armor score. An attacker always does at least 1 damage. So,
if the attacker has a damage score of 8, and the defender has an armor score of
3, the defender loses 5 hit points. If the defender had an armor score of 300,
the defender would still lose 1 hit point.

Your damage score and armor score both start at zero. They can be increased by
buying items in exchange for gold. You start with no items and have as much gold
as you need. Your total damage or armor is equal to the sum of those stats from
all of your items. You have 100 hit points.

Here is what the item shop is selling:

Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3

You must buy exactly one weapon; no dual-wielding. Armor is optional, but you
can't use more than one. You can buy 0-2 rings (at most one for each hand). You
must use any items you buy. The shop only has one of each item, so you can't
buy, for example, two rings of Damage +3.

For example, suppose you have 8 hit points, 5 damage, and 5 armor, and that the
boss has 12 hit points, 7 damage, and 2 armor:

The player deals 5-2 = 3 damage; the boss goes down to 9 hit points.
The boss deals 7-5 = 2 damage; the player goes down to 6 hit points.
The player deals 5-2 = 3 damage; the boss goes down to 6 hit points.
The boss deals 7-5 = 2 damage; the player goes down to 4 hit points.
The player deals 5-2 = 3 damage; the boss goes down to 3 hit points.
The boss deals 7-5 = 2 damage; the player goes down to 2 hit points.
The player deals 5-2 = 3 damage; the boss goes down to 0 hit points.

In this scenario, the player wins! (Barely.)

You have 100 hit points. The boss's actual stats are in your puzzle input. What
is the least amount of gold you can spend and still win the fight?
*/

use std::fmt;
use std::num::ParseIntError;
use std::io::{self, BufRead};

const DEBUG: bool = false;

#[derive(Debug, Clone, PartialEq)]
enum ItemType {
    Weapon,
    Armor,
    Ring,
}

#[derive(Clone)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
    item_type: ItemType,
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:?}, {}, {}, {}, {})",
            self.item_type, self.name, self.cost, self.damage, self.armor
        )
    }
}

#[derive(Debug, Clone)]
struct Player {
    name: String,
    hit_points: i32,
    damage: i32,
    armor: i32,
}

#[derive(Debug, Clone)]
struct Game {
    player1: Player,
    player2: Player,
    items: Vec<Item>,
}

impl Item {
    pub fn new(item_type: ItemType, name: &str, cost: i32, damage: i32, armor: i32) -> Item {
        Item {
            name: name.to_owned(),
            item_type: item_type,
            cost: cost,
            damage: damage,
            armor: armor,
        }
    }
}

impl Player {
    pub fn deal(&self, p: &mut Player) {
        let d = self.damage - p.armor;
        if d > 0 {
            p.hit_points -= d;
        } else {
            p.hit_points -= 1;
        }

        if DEBUG {
            println!("{} goes down to {} hit point", p.name, p.hit_points);
        }
    }

    pub fn lost(&self) -> bool {
        self.hit_points <= 0
    }

    pub fn add_items(&mut self, items: &Vec<Item>) {
        for i in items {
            self.armor += i.armor;
            self.damage += i.damage;
        }
    }
}

impl Game {
    pub fn fortify(&mut self, first: bool, items: &Vec<Item>) {
        if first {
            self.player1.add_items(items);
        } else {
            self.player2.add_items(items);
        }
    }

    fn armor_combinations(&self) -> Vec<Vec<Item>> {
        let armors: Vec<&Item> = self.items
            .iter()
            .filter(|item| item.item_type == ItemType::Armor)
            .collect();

        // Rule is we can only buy one armor or none
        let mut cmb: Vec<Vec<Item>> = vec![];
        cmb.push(vec![]); // No armor combination
        for a in armors {
            cmb.push(vec![a.clone()]); // single armor combination
        }

        cmb
    }

    fn weapon_combinations(&self) -> Vec<Vec<Item>> {
        let weapons: Vec<&Item> = self.items
            .iter()
            .filter(|item| item.item_type == ItemType::Weapon)
            .collect();

        // Rule is we can only buy one armor or none
        let mut cmb: Vec<Vec<Item>> = vec![];
        for w in weapons {
            cmb.push(vec![w.clone()]); // single weapon combination
        }

        cmb
    }

    fn ring_combinations(&self) -> Vec<Vec<Item>> {
        let rings: Vec<&Item> = self.items
            .iter()
            .filter(|item| item.item_type == ItemType::Ring)
            .collect();

        // Rule is we can buy zero, one or two armors
        let mut cmb: Vec<Vec<Item>> = vec![];
        cmb.push(vec![]); // zero ring combination
        for i in 0..rings.len() {
            cmb.push(vec![rings[i].clone()]); // single ring combinations

            for j in i + 1..rings.len() {
                cmb.push(vec![rings[i].clone(), rings[j].clone()]); // two rings combinations
            }
        }

        cmb
    }

    // These should be the valid item combinations
    pub fn item_combinations(&self) -> Vec<Vec<Item>> {
        let mut cmbs: Vec<Vec<Item>> = vec![];

        for ac in self.armor_combinations() {
            for wc in self.weapon_combinations() {
                for rc in self.ring_combinations() {
                    let mut cmb: Vec<Item> = vec![];
                    cmb.extend(ac.iter().cloned());
                    cmb.extend(wc.iter().cloned());
                    cmb.extend(rc.iter().cloned());
                    cmbs.push(cmb);
                }
            }
        }

        cmbs
    }

    pub fn play(&self) -> (i32, String) {
        // We do not want play method to change the player's states
        let mut p1 = self.player1.clone();
        let mut p2 = self.player2.clone();
        let mut moves = 0;

        // Check if the players are already in a lost state
        if p2.lost() {
            return (moves, p1.name.to_owned());
        }
        if p2.lost() {
            return (moves, p1.name.to_owned());
        }

        // We play the game by taking turns until one of them lose
        // First deal goes to player1, then player2 and so on
        loop {
            p1.deal(&mut p2);
            moves += 1;
            if p2.lost() {
                return (moves, p1.name.to_owned());
            }

            p2.deal(&mut p1);
            moves += 1;
            if p1.lost() {
                return (moves, p2.name.to_owned());
            }
        }
    }

    fn player1_wins(&self) -> bool {
        let (_, name) = self.play();
        name == self.player1.name
    }

    #[allow(dead_code)]
    fn player2_wins(&self) -> bool {
        let (_, name) = self.play();
        name == self.player2.name
    }

    // These are the pre-defined items available at the store
    fn store_items() -> Vec<Item> {
        vec![
            // Weapons
            Item::new(ItemType::Weapon, "Dagger", 8, 4, 0),
            Item::new(ItemType::Weapon, "Shortsword", 10, 5, 0),
            Item::new(ItemType::Weapon, "Warhammer", 25, 6, 0),
            Item::new(ItemType::Weapon, "Longsword", 40, 7, 0),
            Item::new(ItemType::Weapon, "Greataxe", 74, 8, 0),
            // Armor
            Item::new(ItemType::Armor, "Leather", 13, 0, 1),
            Item::new(ItemType::Armor, "Chainmail", 31, 0, 2),
            Item::new(ItemType::Armor, "Splintmail", 53, 0, 3),
            Item::new(ItemType::Armor, "Bandedmail", 75, 0, 4),
            Item::new(ItemType::Armor, "Platemail", 102, 0, 5),
            // Rings
            Item::new(ItemType::Ring, "Damage +1", 25, 1, 0),
            Item::new(ItemType::Ring, "Damage +2", 50, 2, 0),
            Item::new(ItemType::Ring, "Damage +3", 100, 3, 0),
            Item::new(ItemType::Ring, "Defense +1", 20, 0, 1),
            Item::new(ItemType::Ring, "Defense +2", 40, 0, 2),
            Item::new(ItemType::Ring, "Defense +3", 80, 0, 3),
        ]
    }
}

// Helper function to calculate the cost of items
fn item_costs(items: &Vec<Item>) -> i32 {
    items.iter().fold(0, |sum, item| sum + item.cost)
}

// We need to compute valid combinations of items one can buy by the order of
// cost, and then try to find the lowest amount we have to spend and still win.
fn best_items_to_buy(boss: Player) {
    let game = Game {
        player1: Player {
            name: "player".to_owned(),
            hit_points: 100,
            damage: 0,
            armor: 0,
        },
        player2: boss,
        items: Game::store_items(),
    };

    let mut cmbs = game.item_combinations();
    // We want to sort item combinations by their cost
    cmbs.sort_by(|a, b| {
        let s1 = item_costs(a);
        let s2 = item_costs(b);
        s1.cmp(&s2)
    });

    // Now we try to find an item combination which allows player1 to win the
    // game with minimum expenditure
    let mut player1_won = false;
    for cmb in &cmbs {
        let mut g = game.clone();
        g.fortify(true, cmb);

        if g.player1_wins() {
            println!("Player 1 wins by combination: {:?}", cmb);
            let cost = item_costs(cmb);
            println!("Cost of the combination: {}", cost);
            player1_won = true;
            break;
        }
    }

    if !player1_won {
        println!("There are no items which allows player to win. Boss always wins!");
    }
}

pub fn parse_hit_points(s: &String) -> Result<i32, ParseIntError> {
    let hs: String = s.chars().skip("Hit Points: ".len()).collect();
    hs.parse::<i32>()
}

pub fn parse_damage(s: &String) -> Result<i32, ParseIntError> {
    let hs: String = s.chars().skip("Damage: ".len()).collect();
    hs.parse::<i32>()
}

fn parse_armor(s: &String) -> Result<i32, ParseIntError> {
    let hs: String = s.chars().skip("Armor: ".len()).collect();
    hs.parse::<i32>()
}

pub fn problem() {
    let mut boss = Player {
        name: format!("boss"),
        hit_points: 0,
        damage: 0,
        armor: 0,
    };

    let mut hp_found = false;
    let mut dm_found = false;
    let mut ar_found = false;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        if sline.starts_with("Hit Points: ") {
            match parse_hit_points(&sline) {
                Ok(num) => {
                    boss.hit_points = num;
                    hp_found = true;
                }
                Err(_) => {
                    println!("Invalid hit points: {}", sline);
                }
            }
        } else if sline.starts_with("Damage: ") {
            match parse_damage(&sline) {
                Ok(num) => {
                    boss.damage = num;
                    dm_found = true;
                }
                Err(_) => {
                    println!("Invalid hit points: {}", sline);
                }
            }
        } else if sline.starts_with("Armor: ") {
            match parse_armor(&sline) {
                Ok(num) => {
                    boss.armor = num;
                    ar_found = true;
                }
                Err(_) => {
                    println!("Invalid hit points: {}", sline);
                }
            }
        }
    }

    if hp_found && dm_found && ar_found {
        best_items_to_buy(boss);
    } else {
        println!("Error: required information missing for boss");
    }
}

#[test]
fn test_example_winner() {
    // In this given (example) game, the player (not boss) should win in 7 moves
    let game = Game {
        player1: Player {
            name: "player".to_owned(),
            hit_points: 8,
            damage: 5,
            armor: 5,
        },
        player2: Player {
            name: "boss".to_owned(),
            hit_points: 12,
            damage: 7,
            armor: 2,
        },
        items: vec![],
    };

    let (moves, winner) = game.play();
    assert_eq!(winner, "player");
    assert_eq!(moves, 7);
    // Test player wins methods
    assert!(game.player1_wins());
    assert!(!game.player2_wins());
}

#[test]
fn test_parse_hit_points() {
    let pr = parse_hit_points(&"Hit Points: 34".to_owned());
    assert_eq!(pr, Ok(34));
}

#[test]
fn test_parse_damage() {
    let pr = parse_damage(&"Damage: 48".to_owned());
    assert_eq!(pr, Ok(48));
}

#[test]
fn test_parse_armor() {
    let pr = parse_armor(&"Armor: 48".to_owned());
    assert_eq!(pr, Ok(48));
}
