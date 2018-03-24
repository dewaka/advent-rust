// https://adventofcode.com/2015/day/7
/*
--- Day 7: Some Assembly Required ---

This year, Santa brought little Bobby Tables a set of wires and bitwise logic
gates! Unfortunately, little Bobby is a little under the recommended age range,
and he needs help assembling the circuit.

Each wire has an identifier (some lowercase letters) and can carry a 16-bit
signal (a number from 0 to 65535). A signal is provided to each wire by a gate,
another wire, or some specific value. Each wire can only get a signal from one
source, but can provide its signal to multiple destinations. A gate provides no
signal until all of its inputs have a signal.

The included instructions booklet describes how to connect the parts together: x
AND y -> z means to connect wires x and y to an AND gate, and then connect its
output to wire z.

For example:

- 123 -> x means that the signal 123 is provided to wire x.
- x AND y -> z means that the bitwise AND of wire x and wire y is provided to wire z.
- p LSHIFT 2 -> q means that the value from wire p is left-shifted by 2 and then
  provided to wire q.
- NOT e -> f means that the bitwise complement of the value from wire e is
  provided to wire f.

Other possible gates include OR (bitwise OR) and RSHIFT (right-shift). If, for
some reason, you'd like to emulate the circuit instead, almost all programming
languages (for example, C, JavaScript, or Python) provide operators for these
gates.

For example, here is a simple circuit:
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i

After it is run, these are the signals on the wires:
d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456

In little Bobby's kit's instructions booklet (provided as your puzzle input),
what signal is ultimately provided to wire a?
*/

extern crate regex;

use self::regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

static DEBUG: bool = false;

type Wire = String;
type Value = u16;

#[derive(Debug)]
enum Operand {
    OpWire { name: Wire },
    OpValue { value: Value },
}

#[derive(Debug)]
enum Operation {
    Set {
        value: Operand,
        wire: Wire,
    },
    Not {
        value: Operand,
        wire: Wire,
    },
    And {
        left: Operand,
        right: Operand,
        wire: Wire,
    },
    Or {
        left: Operand,
        right: Operand,
        wire: Wire,
    },
    LShift {
        left: Operand,
        right: Operand,
        wire: Wire,
    },
    RShift {
        left: Operand,
        right: Operand,
        wire: Wire,
    },
}

type Circuit = HashMap<Wire, Value>;

fn operate(circuit: &mut Circuit, op: &Operation) -> bool {
    // println!("{:?}", op);

    match op {
        &Operation::Set {
            ref value,
            ref wire,
        } => return operate_set(circuit, value, wire),

        &Operation::Not {
            ref value,
            ref wire,
        } => return operate_not(circuit, value, wire),

        &Operation::And {
            ref left,
            ref right,
            ref wire,
        } => return operate_and(circuit, left, right, wire),

        &Operation::Or {
            ref left,
            ref right,
            ref wire,
        } => return operate_or(circuit, left, right, wire),

        &Operation::LShift {
            ref left,
            ref right,
            ref wire,
        } => return operate_lshift(circuit, left, right, wire),

        &Operation::RShift {
            ref left,
            ref right,
            ref wire,
        } => return operate_rshift(circuit, left, right, wire),
    }
}

fn operand_value(circuit: &Circuit, op: &Operand) -> Option<Value> {
    match op {
        &Operand::OpWire { ref name } => {
            if circuit.contains_key(name) {
                return Some(*circuit.get(name).unwrap());
            }
        }
        // Here we are setting the wire to a simple value
        &Operand::OpValue { ref value } => {
            return Some(*value);
        }
    }
    None
}

fn operate_set(circuit: &mut Circuit, value: &Operand, wire: &Wire) -> bool {
    match operand_value(circuit, value) {
        Some(val) => {
            circuit.insert(wire.clone(), val);
            return true;
        }
        None => return false,
    }
}

fn operate_not(circuit: &mut Circuit, value: &Operand, wire: &Wire) -> bool {
    match operand_value(circuit, value) {
        Some(val) => {
            circuit.insert(wire.clone(), !val); // set to bit-wise not value
            return true;
        }
        None => return false,
    }
}

fn operate_and(circuit: &mut Circuit, left: &Operand, right: &Operand, wire: &Wire) -> bool {
    match operand_value(circuit, left) {
        Some(left_val) => match operand_value(circuit, right) {
            Some(right_val) => {
                circuit.insert(wire.clone(), left_val & right_val); // bitwise and
                return true;
            }
            None => return false,
        },
        None => return false,
    }
}

fn operate_or(circuit: &mut Circuit, left: &Operand, right: &Operand, wire: &Wire) -> bool {
    match operand_value(circuit, left) {
        Some(left_val) => match operand_value(circuit, right) {
            Some(right_val) => {
                circuit.insert(wire.clone(), left_val | right_val); // bitwise or
                return true;
            }
            None => return false,
        },
        None => return false,
    }
}

fn operate_lshift(circuit: &mut Circuit, left: &Operand, right: &Operand, wire: &Wire) -> bool {
    match operand_value(circuit, left) {
        Some(left_val) => match operand_value(circuit, right) {
            Some(right_val) => {
                let val = left_val << right_val; // bitwise left shift by right_val
                circuit.insert(wire.clone(), val);
                return true;
            }
            None => return false,
        },
        None => return false,
    }
}

fn operate_rshift(circuit: &mut Circuit, left: &Operand, right: &Operand, wire: &Wire) -> bool {
    match operand_value(circuit, left) {
        Some(left_val) => match operand_value(circuit, right) {
            Some(right_val) => {
                let val = left_val >> right_val; // bitwise right shift by right_val
                circuit.insert(wire.clone(), val);
                return true;
            }
            None => return false,
        },
        None => return false,
    }
}

fn print_circuit(circuit: &Circuit) {
    for (key, value) in circuit.into_iter() {
        println!("{} -> {}", key, value);
    }
}

fn run_operations(circuit: &mut Circuit, ops: &mut Vec<Operation>) -> bool {
    println!("There are {} operations to run", ops.len());

    let mut count = 1;
    loop {
        let orig_size = ops.len();

        // retain only operations which weren't successful operations on the circuit
        ops.retain(|op| {
            if operate(circuit, &op) {
                println!("Reduced op: {:?}", op);
                return false;
            } else {
                true
            }
        });

        println!(
            "Operation count - original: {}, now: {}",
            orig_size,
            ops.len()
        );

        // if operations weren't reduced at all, then no point continuing anymore
        if orig_size == ops.len() {
            break;
        }

        count += 1;
    }

    println!("Operations run in {} rounds", count);

    if DEBUG {
        println!("** Operations ***");
        for op in ops.iter() {
            println!("{:?}", op);
        }
        println!("*****************");
    }

    ops.is_empty() // have we finished all possible operations?
}

fn parse_set(left: &String, right: &String) -> Option<Operation> {
    Some(match left.parse::<u16>() {
        Ok(num) => Operation::Set {
            value: Operand::OpValue { value: num },
            wire: right.clone(),
        },
        Err(_) => Operation::Set {
            value: Operand::OpWire { name: left.clone() },
            wire: right.clone(),
        },
    })
}

fn parse_not(left: &String, right: &String) -> Option<Operation> {
    Some(match left.parse::<u16>() {
        Ok(num) => Operation::Not {
            value: Operand::OpValue { value: num },
            wire: right.clone(),
        },
        Err(_) => Operation::Not {
            value: Operand::OpWire { name: left.clone() },
            wire: right.clone(),
        },
    })
}

fn parse_and(left: &String, right: &String, wire: &String) -> Option<Operation> {
    Some(match left.parse::<u16>() {
        Ok(lval) => match right.parse::<u16>() {
            Ok(rval) => Operation::And {
                left: Operand::OpValue { value: lval },
                right: Operand::OpValue { value: rval },
                wire: wire.clone(),
            },
            Err(_) => Operation::And {
                left: Operand::OpValue { value: lval },
                right: Operand::OpWire {
                    name: right.clone(),
                },
                wire: wire.clone(),
            },
        },
        Err(_) => match right.parse::<u16>() {
            Ok(rval) => Operation::And {
                left: Operand::OpWire { name: left.clone() },
                right: Operand::OpValue { value: rval },
                wire: wire.clone(),
            },
            Err(_) => Operation::And {
                left: Operand::OpWire { name: left.clone() },
                right: Operand::OpWire {
                    name: right.clone(),
                },
                wire: wire.clone(),
            },
        },
    })
}

fn parse_or(left: &String, right: &String, wire: &String) -> Option<Operation> {
    Some(match left.parse::<u16>() {
        Ok(lval) => match right.parse::<u16>() {
            Ok(rval) => Operation::Or {
                left: Operand::OpValue { value: lval },
                right: Operand::OpValue { value: rval },
                wire: wire.clone(),
            },
            Err(_) => Operation::Or {
                left: Operand::OpValue { value: lval },
                right: Operand::OpWire {
                    name: right.clone(),
                },
                wire: wire.clone(),
            },
        },
        Err(_) => match right.parse::<u16>() {
            Ok(rval) => Operation::Or {
                left: Operand::OpWire { name: left.clone() },
                right: Operand::OpValue { value: rval },
                wire: wire.clone(),
            },
            Err(_) => Operation::Or {
                left: Operand::OpWire { name: left.clone() },
                right: Operand::OpWire {
                    name: right.clone(),
                },
                wire: wire.clone(),
            },
        },
    })
}

fn parse_lshift(left: &String, right: &String, wire: &String) -> Option<Operation> {
    Some(match left.parse::<u16>() {
        Ok(lval) => match right.parse::<u16>() {
            Ok(rval) => Operation::LShift {
                left: Operand::OpValue { value: lval },
                right: Operand::OpValue { value: rval },
                wire: wire.clone(),
            },
            Err(_) => Operation::LShift {
                left: Operand::OpValue { value: lval },
                right: Operand::OpWire {
                    name: right.clone(),
                },
                wire: wire.clone(),
            },
        },
        Err(_) => match right.parse::<u16>() {
            Ok(rval) => Operation::LShift {
                left: Operand::OpWire { name: left.clone() },
                right: Operand::OpValue { value: rval },
                wire: wire.clone(),
            },
            Err(_) => Operation::LShift {
                left: Operand::OpWire { name: left.clone() },
                right: Operand::OpWire {
                    name: right.clone(),
                },
                wire: wire.clone(),
            },
        },
    })
}

fn parse_rshift(left: &String, right: &String, wire: &String) -> Option<Operation> {
    Some(match left.parse::<u16>() {
        Ok(lval) => match right.parse::<u16>() {
            Ok(rval) => Operation::RShift {
                left: Operand::OpValue { value: lval },
                right: Operand::OpValue { value: rval },
                wire: wire.clone(),
            },
            Err(_) => Operation::RShift {
                left: Operand::OpValue { value: lval },
                right: Operand::OpWire {
                    name: right.clone(),
                },
                wire: wire.clone(),
            },
        },
        Err(_) => match right.parse::<u16>() {
            Ok(rval) => Operation::RShift {
                left: Operand::OpWire { name: left.clone() },
                right: Operand::OpValue { value: rval },
                wire: wire.clone(),
            },
            Err(_) => Operation::RShift {
                left: Operand::OpWire { name: left.clone() },
                right: Operand::OpWire {
                    name: right.clone(),
                },
                wire: wire.clone(),
            },
        },
    })
}

fn parse_operation(s: &String) -> Option<Operation> {
    lazy_static! {
        static ref R_SET: Regex = Regex::new(r"(\w+) -> (\w+)").unwrap();
        static ref R_NOT: Regex = Regex::new(r"NOT (\w+) -> (\w+)").unwrap();
        static ref R_AND: Regex = Regex::new(r"(\w+) AND (\w+) -> (\w+)").unwrap();
        static ref R_OR: Regex = Regex::new(r"(\w+) OR (\w+) -> (\w+)").unwrap();
        static ref R_LSHIFT: Regex = Regex::new(r"(\w+) LSHIFT (\w+) -> (\w+)").unwrap();
        static ref R_RSHIFT: Regex = Regex::new(r"(\w+) RSHIFT (\w+) -> (\w+)").unwrap();
    }

    if R_AND.is_match(s) {
        for cap in R_AND.captures_iter(s) {
            return parse_and(&cap[1].to_owned(), &cap[2].to_owned(), &cap[3].to_owned());
        }
    }

    if R_OR.is_match(s) {
        for cap in R_OR.captures_iter(s) {
            return parse_or(&cap[1].to_owned(), &cap[2].to_owned(), &cap[3].to_owned());
        }
    }

    if R_LSHIFT.is_match(s) {
        for cap in R_LSHIFT.captures_iter(s) {
            return parse_lshift(&cap[1].to_owned(), &cap[2].to_owned(), &cap[3].to_owned());
        }
    }

    if R_RSHIFT.is_match(s) {
        for cap in R_RSHIFT.captures_iter(s) {
            return parse_rshift(&cap[1].to_owned(), &cap[2].to_owned(), &cap[3].to_owned());
        }
    }

    // TODO: Fix the situation that we have to check if NOT is matched first
    // before SET is matched.
    if R_NOT.is_match(s) {
        for cap in R_NOT.captures_iter(s) {
            return parse_not(&cap[1].to_owned(), &cap[2].to_owned());
        }
    }

    if R_SET.is_match(s) {
        for cap in R_SET.captures_iter(s) {
            return parse_set(&cap[1].to_owned(), &cap[2].to_owned());
        }
    }

    None
}

pub fn problem() {
    println!("2015, day 7");

    let mut circuit: Circuit = Circuit::new();
    let mut ops: Vec<Operation> = vec![];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();

        match parse_operation(&sline) {
            Some(op) => ops.push(op),
            None => println!("Failed to parse: {}", sline),
        }
    }

    if DEBUG {
        println!("** Original Operations ***");
        for op in &mut ops {
            println!("{:?}", op);
        }
        println!("*****************");
    }

    run_operations(&mut circuit, &mut ops);

    print_circuit(&circuit);

    if circuit.contains_key("a") {
        println!("Value of gate 'a' -> {}", circuit.get("a").unwrap());
    }
}

#[test]
fn test_gates1() {
    let mut circuit: Circuit = Circuit::new();
    let mut ops = vec![
        parse_operation(&"x LSHIFT 2 -> f".to_owned()).unwrap(),
        parse_operation(&"x -> y".to_owned()).unwrap(),
        parse_operation(&"1 -> x".to_owned()).unwrap(),
    ];

    assert!(run_operations(&mut circuit, &mut ops));
    assert_eq!(*circuit.get("x").unwrap(), 1);
    assert_eq!(*circuit.get("f").unwrap(), 4);
    assert_eq!(*circuit.get("y").unwrap(), 1);
}

#[test]
fn test_gates2() {
    let mut circuit: Circuit = Circuit::new();
    let mut ops = vec![
        parse_operation(&"123 -> x".to_owned()).unwrap(),
        parse_operation(&"456 -> y".to_owned()).unwrap(),
        parse_operation(&"x AND y -> d".to_owned()).unwrap(),
        parse_operation(&"x OR y -> e".to_owned()).unwrap(),
        parse_operation(&"x LSHIFT 2 -> f".to_owned()).unwrap(),
        parse_operation(&"y RSHIFT 2 -> g".to_owned()).unwrap(),
        parse_operation(&"NOT x -> h".to_owned()).unwrap(),
        parse_operation(&"NOT y -> i".to_owned()).unwrap(),
    ];

    assert!(run_operations(&mut circuit, &mut ops));
    assert_eq!(*circuit.get("f").unwrap(), 492);
    assert_eq!(*circuit.get("h").unwrap(), 65412);
    assert_eq!(*circuit.get("x").unwrap(), 123);
    assert_eq!(*circuit.get("d").unwrap(), 72);
    assert_eq!(*circuit.get("i").unwrap(), 65079);
    assert_eq!(*circuit.get("y").unwrap(), 456);
    assert_eq!(*circuit.get("g").unwrap(), 114);
    assert_eq!(*circuit.get("e").unwrap(), 507);
}
