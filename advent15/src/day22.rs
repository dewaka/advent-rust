// https://adventofcode.com/2015/day/22
/*
--- Day 22: Wizard Simulator 20XX ---

Little Henry Case decides that defeating bosses with swords and stuff is boring.
Now he's playing the game with a wizard. Of course, he gets stuck on another
boss and needs your help again.

In this version, combat still proceeds with the player and the boss taking
alternating turns. The player still goes first. Now, however, you don't get any
equipment; instead, you must choose one of your spells to cast. The first
character at or below 0 hit points loses.

Since you're a wizard, you don't get to wear armor, and you can't attack
normally. However, since you do magic damage, your opponent's armor is ignored,
and so the boss effectively has zero armor as well. As before, if armor (from a
spell, in this case) would reduce damage below 1, it becomes 1 instead - that
is, the boss' attacks always deal at least 1 damage.

On each of your turns, you must select one of your spells to cast. If you cannot
afford to cast any spell, you lose. Spells cost mana; you start with 500 mana,
but have no maximum limit. You must have enough mana to cast a spell, and its
cost is immediately deducted when you cast it. Your spells are Magic Missile,
Drain, Shield, Poison, and Recharge.

- Magic Missile costs 53 mana. It instantly does 4 damage.
- Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
- Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is
  active, your armor is increased by 7.
- Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start
  of each turn while it is active, it deals the boss 3 damage.
- Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the
  start of each turn while it is active, it gives you 101 new mana.

Effects all work the same way. Effects apply at the start of both the player's
turns and the boss' turns. Effects are created with a timer (the number of turns
they last); at the start of each turn, after they apply any effect they have,
their timer is decreased by one. If this decreases the timer to zero, the effect
ends. You cannot cast a spell that would start an effect which is already
active. However, effects can be started on the same turn they end.

For example, suppose the player has 10 hit points and 250 mana, and that the
boss has 13 hit points and 8 damage:

-- Player turn --
- Player has 10 hit points, 0 armor, 250 mana
- Boss has 13 hit points
Player casts Poison.

-- Boss turn --
- Player has 10 hit points, 0 armor, 77 mana
- Boss has 13 hit points
Poison deals 3 damage; its timer is now 5.
Boss attacks for 8 damage.

-- Player turn --
- Player has 2 hit points, 0 armor, 77 mana
- Boss has 10 hit points
Poison deals 3 damage; its timer is now 4.
Player casts Magic Missile, dealing 4 damage.

-- Boss turn --
- Player has 2 hit points, 0 armor, 24 mana
- Boss has 3 hit points
Poison deals 3 damage. This kills the boss, and the player wins.

Now, suppose the same initial conditions, except that the boss has 14 hit points
instead:

-- Player turn --
- Player has 10 hit points, 0 armor, 250 mana
- Boss has 14 hit points
Player casts Recharge.

-- Boss turn --
- Player has 10 hit points, 0 armor, 21 mana
- Boss has 14 hit points
Recharge provides 101 mana; its timer is now 4.
Boss attacks for 8 damage!

-- Player turn --
- Player has 2 hit points, 0 armor, 122 mana
- Boss has 14 hit points
Recharge provides 101 mana; its timer is now 3.
Player casts Shield, increasing armor by 7.

-- Boss turn --
- Player has 2 hit points, 7 armor, 110 mana
- Boss has 14 hit points
Shield's timer is now 5.
Recharge provides 101 mana; its timer is now 2.
Boss attacks for 8 - 7 = 1 damage!

-- Player turn --
- Player has 1 hit point, 7 armor, 211 mana
- Boss has 14 hit points
Shield's timer is now 4.
Recharge provides 101 mana; its timer is now 1.
Player casts Drain, dealing 2 damage, and healing 2 hit points.

-- Boss turn --
- Player has 3 hit points, 7 armor, 239 mana
- Boss has 12 hit points
Shield's timer is now 3.
Recharge provides 101 mana; its timer is now 0.
Recharge wears off.
Boss attacks for 8 - 7 = 1 damage!

-- Player turn --
- Player has 2 hit points, 7 armor, 340 mana
- Boss has 12 hit points
Shield's timer is now 2.
Player casts Poison.

-- Boss turn --
- Player has 2 hit points, 7 armor, 167 mana
- Boss has 12 hit points
Shield's timer is now 1.
Poison deals 3 damage; its timer is now 5.
Boss attacks for 8 - 7 = 1 damage!

-- Player turn --
- Player has 1 hit point, 7 armor, 167 mana
- Boss has 9 hit points
Shield's timer is now 0.
Shield wears off, decreasing armor by 7.
Poison deals 3 damage; its timer is now 4.
Player casts Magic Missile, dealing 4 damage.

-- Boss turn --
- Player has 1 hit point, 0 armor, 114 mana
- Boss has 2 hit points
Poison deals 3 damage. This kills the boss, and the player wins.

You start with 50 hit points and 500 mana points. The boss's actual stats are in
your puzzle input. What is the least amount of mana you can spend and still win
the fight? (Do not include mana recharge effects as "spending" negative mana.)
*/
extern crate log;
extern crate env_logger;

use std::io::{self, BufRead};
use day21::{parse_damage, parse_hit_points};

#[derive(Debug, Clone, PartialEq)]
struct Player {
    name: String,
    hit_points: i32,
    armor: i32,
    damage: i32,
    mana: i32,
}

impl Player {
    pub fn lost(&self) -> bool {
        self.hit_points <= 0
    }

    pub fn incur_damage(&mut self, attack: i32) {
        let d = attack - self.armor;
        if d < 1 {
            debug!("incurring damage of 1 to {}", self.name);
            self.hit_points -= 1; // at least 1 point damage always
        } else {
            debug!("incurring damage of {} to {}", d, self.name);
            self.hit_points -= d;
        }
    }

    pub fn apply_healing(&mut self, healing: i32) {
        debug!("applying healing of {} to {}", healing, self.name);
        self.hit_points += healing;
    }

    pub fn apply_armor(&mut self, armor: i32) {
        debug!("adding {} armor to {}", armor, self.name);
        self.armor += armor;
    }

    pub fn add_mana(&mut self, mana: i32) {
        debug!("adding {} mana to {}", mana, self.name);
        self.mana += mana;
    }

    // This is a simple attack of the type boss would do
    pub fn attack(&self, p: &mut Player) {
        p.incur_damage(self.damage);
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Spell {
    name: String,
    cost: i32,
    damage: (i32, i32),
    healing: (i32, i32),
    armor: (i32, i32),
    mana: (i32, i32),
}

trait CastSpell {
    fn active(&self) -> bool;
    fn cast(&mut self, player: &mut Player, opponent: &mut Player) -> bool;
}

impl Spell {
    fn can_damage(&self) -> bool {
        self.damage.1 > 0
    }
    fn can_heal(&self) -> bool {
        self.healing.1 > 0
    }
    fn has_armor(&self) -> bool {
        self.armor.1 > 0
    }
    fn has_mana(&self) -> bool {
        self.mana.1 > 0
    }

    fn use_damage(&mut self) -> i32 {
        if self.can_damage() {
            self.damage.1 -= 1;
            return self.damage.0;
        }

        0
    }

    fn use_healing(&mut self) -> i32 {
        if self.can_heal() {
            self.healing.1 -= 1;
            return self.healing.0;
        }

        0
    }

    fn use_armor(&mut self) -> i32 {
        if self.has_armor() {
            self.armor.1 -= 1;
            return self.armor.0;
        }

        0
    }

    fn use_mana(&mut self) -> i32 {
        if self.has_mana() {
            self.mana.1 -= 1;
            return self.mana.0;
        }

        0
    }
}

impl CastSpell for Spell {
    fn active(&self) -> bool {
        // This if any one of Spell's effects are active, then it is still active
        (self.damage.1 > 0) || (self.healing.1 > 0) || (self.armor.1 > 0) || (self.mana.1 > 0)
    }

    fn cast(&mut self, player: &mut Player, opponent: &mut Player) -> bool {
        if !self.active() {
            debug!("spell {} is not active", self.name);
            return false;
        }

        debug!("spell {} is active", self.name);

        if self.can_damage() {
            opponent.incur_damage(self.use_damage());
        }

        if self.can_heal() {
            player.apply_healing(self.use_healing());
        }

        if self.has_armor() {
            player.apply_armor(self.use_armor());
        }

        if self.has_mana() {
            player.add_mana(self.use_mana());
        }

        true
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Game {
    player1: Player,
    player2: Player,
}

impl Game {
    pub fn play(&self, spells: Vec<Spell>) -> String {
        debug!("start of play player1: {:?}", self.player1);
        debug!("start of play player2: {:?}", self.player2);
        debug!("start spells: {:?}", spells);

        let mut active_spells: Vec<Spell> = vec![];
        let mut p1 = self.player1.clone();
        let mut p2 = self.player2.clone();

        if p2.lost() {
            return p1.name;
        }
        if p1.lost() {
            return p2.name;
        }

        loop {
            // How to add to active spells?

            // Cast all active spells
            for s in &mut active_spells {
                s.cast(&mut p1, &mut p2);
            }

            // Do p1 attacking
            if p2.lost() {
                return p1.name;
            }

            p2.attack(&mut p1);
            // Do p2 attacking
            if p1.lost() {
                return p2.name;
            }
        }
    }
}

pub fn problem() {
    env_logger::init();

    let mut hit_points: Option<i32> = None;
    let mut damage: Option<i32> = None;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        if let Ok(num) = parse_hit_points(&sline) {
            hit_points = Some(num);
        }
        if let Ok(num) = parse_damage(&sline) {
            damage = Some(num);
        }
    }

    if hit_points.is_none() || damage.is_none() {
        println!("Error: required hit points or damage points not found.");
        return;
    }

    println!(
        "Hit points: {}, Damage points: {}",
        hit_points.unwrap(),
        damage.unwrap()
    );
}

#[test]
fn test_example() {
    assert_eq!(10, 10);
}
