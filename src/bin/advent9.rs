// advent9.rs
//
// Traveling Santa Problem

extern crate permutohedron;
#[macro_use] extern crate scan_fmt;

use std::io;

fn main() {
    let mut city_table = CityDistances::new();
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
        let (city1, city2, distance) = scan_fmt!(&input, "{} to {} = {}", String, String, u32);
        city_table.add_distance(&city1.unwrap(), &city2.unwrap(), distance.unwrap());
    }

    let min = calc_shortest_distance(&city_table);
    println!("Shortest route: {}", min);

    let max = calc_longest_distance(&city_table);
    println!("Longest route: {}", max);

}

struct CityDistances {
    cities: Vec<String>,
    distance_map: std::collections::HashMap<(usize, usize), u32>,
}

impl CityDistances {
    fn new() -> CityDistances {
        CityDistances{cities: Vec::new(), distance_map: std::collections::HashMap::new()}
    }

    fn find_city_idx(&mut self, city: &str) -> usize {
        match self.cities.iter().position(|x| x == city) {
            Some(idx) => idx,
            None => {
                self.cities.push(String::from(city));
                self.cities.len() - 1
            }
        }
    }

    fn add_distance(&mut self, city1: &str, city2: &str, distance: u32) {
        let idx1 = self.find_city_idx(city1);
        let idx2 = self.find_city_idx(city2);
        // insert both possible tuples; we won't run out of memory and it makes things simpler
        self.distance_map.insert((idx1, idx2), distance);
        self.distance_map.insert((idx2, idx1), distance);
    }

    fn find_distance(&self, idx1: usize, idx2: usize) -> u32 {
        *self.distance_map.get(&(idx1, idx2)).unwrap()
    }
}

fn calc_shortest_distance(city_table: &CityDistances) -> u32 {
    let mut indices: Vec<_> = (0..city_table.cities.len()).collect();
    permutohedron::Heap::new(&mut indices[..])
        .map(|x| x.windows(2).fold(0, |acc, x| acc + city_table.find_distance(x[0], x[1])))
        .min().unwrap()
}

#[test]
fn test_distance_struct() {
    let mut cities = CityDistances::new();
    cities.add_distance("Hoth", "Tatooine", 1000);
    cities.add_distance("Hoth", "Coruscant", 300);
    cities.add_distance("Coruscant", "Tatooine", 900);
    assert_eq!(1000, cities.find_distance(0, 1));
    assert_eq!(1000, cities.find_distance(1, 0));
    assert_eq!(300, cities.find_distance(0, 2));
    assert_eq!(300, cities.find_distance(2, 0));
    assert_eq!(900, cities.find_distance(1, 2));
    assert_eq!(900, cities.find_distance(2, 1));
}

#[test]
fn test_calc_shortest_distance() {
    let mut cities = CityDistances::new();
    cities.add_distance("London", "Dublin", 464);
    cities.add_distance("London", "Belfast", 518);
    cities.add_distance("Dublin", "Belfast", 141);
    assert_eq!(605, calc_shortest_distance(&cities));
}

// Part 2
fn calc_longest_distance(city_table: &CityDistances) -> u32 {
    let mut indices: Vec<_> = (0..city_table.cities.len()).collect();
    permutohedron::Heap::new(&mut indices[..])
        .map(|x| x.windows(2).fold(0, |acc, x| acc + city_table.find_distance(x[0], x[1])))
        .max().unwrap()
}


