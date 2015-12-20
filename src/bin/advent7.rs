// advent7.rs
// circuit

use std::io;
use std::collections::HashMap;

fn main() {
    let mut circuit = Circuit::new();
    loop {
        let mut input = String::new();
        let result = io::stdin().read_line(&mut input);
        match result {
            Ok(byte_count) => if byte_count == 0 { break; },
            Err(_) => {
                println!("error reading from stdin");
                break;
            }
        }
        circuit.add(&input.trim());
    }

    let result = circuit.eval("a");
    println!("Part 1 a = {}", result);

    // Part 2
    circuit.cache.clear();
    circuit.add(&format!("{} -> b", result));
    println!("Part 2 a = {}", circuit.eval("a"));
}

#[derive(Debug)]
struct Circuit {
    sym_table: HashMap<String, String>,
    cache: HashMap<String, u16>,
}

impl Circuit {
    fn new() -> Circuit {
        Circuit {sym_table: HashMap::new(), cache: HashMap::new()}
    }

    fn add(&mut self, s: &str) {
        let mut iter = s.splitn(2, " -> ");
        let val = String::from(iter.next().unwrap());
        let symbol = String::from(iter.next().unwrap());
        self.sym_table.insert(symbol, val);
    }

    fn eval(&mut self, expr: &str) -> u16 {
        // If the expression is in the cache, we're done
        if self.cache.contains_key(expr) {
            return *self.cache.get(expr).unwrap();
        }

        // Otherwise further parse the expression
        let result = if let Ok(val) = expr.parse::<u16>() {
            // expression is a u16
            return val;
        } else if self.sym_table.contains_key(expr) {
            // expression is a symbol in our table
            let symbol_val = self.sym_table.get(expr).unwrap().clone();
            self.eval(&symbol_val)
        } else if expr.contains("AND") {
            let v: Vec<&str> = expr.splitn(2, " AND ").collect();
            let (left, right) = (v[0], v[1]);
            self.eval(&left) & self.eval(&right)
        } else if expr.contains("OR") {
            let v: Vec<&str> = expr.splitn(2, " OR ").collect();
            let (left, right) = (v[0], v[1]);
            self.eval(&left) | self.eval(&right)
        } else if expr.contains("LSHIFT") {
            let v: Vec<&str> = expr.splitn(2, " LSHIFT ").collect();
            let (left, right) = (v[0], v[1]);
            self.eval(&left) << self.eval(&right)
        } else if expr.contains("RSHIFT") {
            let v: Vec<&str> = expr.splitn(2, " RSHIFT ").collect();
            let (left, right) = (v[0], v[1]);
            self.eval(&left) >> self.eval(&right)
        } else if expr.contains("NOT") {
            let right = expr.trim_left_matches("NOT ");
            !self.eval(&right)
        } else {
            panic!(format!("couldn't parse expr {}", expr))
        };

        self.cache.insert(String::from(expr), result);
        result
    }
}

#[test]
#[should_panic]
fn test_eval_fail() {
    let mut circuit = Circuit::new();
    circuit.add("123 -> x");
    circuit.add("456 -> y");
    circuit.add("x AND y -> d");
    circuit.add("x OR y -> e");
    circuit.add("x LSHIFT 2 -> f");
    circuit.add("y RSHIFT 2 -> g");
    circuit.add("NOT x -> h");
    circuit.add("NOT y -> i");

    circuit.eval("xyzzy");
}

#[test]
fn test_eval() {
    let mut circuit = Circuit::new();
    circuit.add("123 -> x");
    circuit.add("456 -> y");
    circuit.add("x AND y -> d");
    circuit.add("x OR y -> e");
    circuit.add("x LSHIFT 2 -> f");
    circuit.add("y RSHIFT 2 -> g");
    circuit.add("NOT x -> h");
    circuit.add("NOT y -> i");

    assert_eq!(123, circuit.eval("x"));
    assert_eq!(456, circuit.eval("y"));
    assert_eq!(72, circuit.eval("d"));
    assert_eq!(507, circuit.eval("e"));
    assert_eq!(492, circuit.eval("f"));
    assert_eq!(114, circuit.eval("g"));
    assert_eq!(65412, circuit.eval("h"));
    assert_eq!(65079, circuit.eval("i"));
}

