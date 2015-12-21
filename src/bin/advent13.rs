// advent13.rs
//
// Seating arrangment
// Note: This is heavily based on the solution to Day 9

extern crate permutohedron;
#[macro_use] extern crate scan_fmt;

use std::io;

fn main() {
    let mut pair_opt = PairOpt::new();
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
        // Parse input
        if input.contains("gain") {
            let (person1, happiness, person2) = 
                scan_fmt!(&input, 
                          "{} would gain {} happiness units by sitting next to {}.", 
                          String, 
                          i32, 
                          String);
            pair_opt.add_pair(&person1.unwrap(), &person2.unwrap(), happiness.unwrap());
        } else {
            let (person1, happiness, person2) = 
                scan_fmt!(&input, 
                          "{} would lose {} happiness units by sitting next to {}.", 
                          String, 
                          i32, 
                          String);
            pair_opt.add_pair(&person1.unwrap(), &person2.unwrap(), -happiness.unwrap());
        }
    }

    let max = calc_max_happiness(&pair_opt);
    println!("Max change in happiness: {}", max);

    let max_with_me = calc_max_happiness_with_me(&pair_opt);
    println!("Max change in happiness: {}", max_with_me);
}

struct PairOpt {
    people: Vec<String>,
    happiness_map: std::collections::HashMap<(usize, usize), i32>,
}

impl PairOpt {
    fn new() -> PairOpt {
        PairOpt{people: Vec::new(), happiness_map: std::collections::HashMap::new()}
    }

    fn find_person_idx(&mut self, person: &str) -> usize {
        match self.people.iter().position(|x| x == person) {
            Some(idx) => idx,
            None => {
                self.people.push(String::from(person));
                self.people.len() - 1
            }
        }
    }

    fn add_pair(&mut self, person1: &str, person2: &str, distance: i32) {
        let idx1 = self.find_person_idx(person1);
        let idx2 = self.find_person_idx(person2);
        self.happiness_map.insert((idx1, idx2), distance);
    }

    fn get_pair_val(&self, idx1: usize, idx2: usize) -> i32 {
        *self.happiness_map.get(&(idx1, idx2)).unwrap() +
        *self.happiness_map.get(&(idx2, idx1)).unwrap()
    }
}

fn calc_max_happiness(pair_opt: &PairOpt) -> i32 {
    let mut indices: Vec<_> = (0..pair_opt.people.len()).collect();
    // check the first and last in the list, then use windows() to get all the other pairs
    permutohedron::Heap::new(&mut indices[..])
        .map(|x| pair_opt.get_pair_val(*x.first().unwrap(), *x.last().unwrap()) + 
             x.windows(2).fold(0, |acc, x| acc + pair_opt.get_pair_val(x[0], x[1])))
        .max().unwrap()
}

fn calc_max_happiness_with_me(pair_opt: &PairOpt) -> i32 {
    let mut indices: Vec<_> = (0..pair_opt.people.len()).collect();
    // just skip inserting the first and last in the list this time, because that's me
    permutohedron::Heap::new(&mut indices[..])
        .map(|x|
             x.windows(2).fold(0, |acc, x| acc + pair_opt.get_pair_val(x[0], x[1])))
        .max().unwrap()
}

#[test]
fn test_distance_struct() {
    let mut pair_opt = PairOpt::new();
    pair_opt.add_pair("Alice", "Bob", 54);
    pair_opt.add_pair("Alice", "Carol", -79);
    pair_opt.add_pair("Alice", "David", -2);
    pair_opt.add_pair("Bob", "Alice", 83);
    pair_opt.add_pair("Bob", "Carol", -7);
    pair_opt.add_pair("Bob", "David", -63);
    pair_opt.add_pair("Carol", "Alice", -62);
    pair_opt.add_pair("Carol", "Bob", 60);
    pair_opt.add_pair("Carol", "David", 55);
    pair_opt.add_pair("David", "Alice", 46);
    pair_opt.add_pair("David", "Bob", -7);
    pair_opt.add_pair("David", "Carol", 41);
    assert_eq!(54 + 83, pair_opt.get_pair_val(0, 1));
    assert_eq!(54 + 83, pair_opt.get_pair_val(1, 0));
    assert_eq!(-79 - 62, pair_opt.get_pair_val(0, 2));
}

#[test]
fn test_calc_max_happiness() {
    let mut pair_opt = PairOpt::new();
    pair_opt.add_pair("Alice", "Bob", 54);
    pair_opt.add_pair("Alice", "Carol", -79);
    pair_opt.add_pair("Alice", "David", -2);
    pair_opt.add_pair("Bob", "Alice", 83);
    pair_opt.add_pair("Bob", "Carol", -7);
    pair_opt.add_pair("Bob", "David", -63);
    pair_opt.add_pair("Carol", "Alice", -62);
    pair_opt.add_pair("Carol", "Bob", 60);
    pair_opt.add_pair("Carol", "David", 55);
    pair_opt.add_pair("David", "Alice", 46);
    pair_opt.add_pair("David", "Bob", -7);
    pair_opt.add_pair("David", "Carol", 41);
    assert_eq!(330, calc_max_happiness(&pair_opt));
}

