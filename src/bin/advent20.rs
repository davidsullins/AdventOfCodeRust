// advent20.rs
// factorizing

// Used the first crate I found that did what I needed; turned out to be quite slow, oh well
extern crate tcorp_math_mods;

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let presents = input.trim().parse::<u32>().unwrap();
    println!("first house: {}", find_lowest_house(presents));
    println!("first house part 2: {}", find_lowest_house2(presents));
}

fn find_lowest_house(min_presents: u32) -> u32 {
    (1u32..).find(|x| count_presents(*x) >= min_presents).unwrap()
}

fn count_presents(house: u32) -> u32 {
    assert!(house > 0);
    10 * tcorp_math_mods::factors::factors_for(house).iter().fold(0, |acc, x| acc + x)
}

#[test]
fn test_count_presents() {
    assert_eq!(10, count_presents(1));
    assert_eq!(30, count_presents(2));
    assert_eq!(40, count_presents(3));
    assert_eq!(70, count_presents(4));
    assert_eq!(60, count_presents(5));
    assert_eq!(120, count_presents(6));
    assert_eq!(80, count_presents(7));
    assert_eq!(150, count_presents(8));
    assert_eq!(130, count_presents(9));
}

#[test]
fn test_find_lowest_house() {
    assert_eq!(6, find_lowest_house(100));
    assert_eq!(6, find_lowest_house(120));
    assert_eq!(8, find_lowest_house(121));
}

// part 2
fn find_lowest_house2(min_presents: u32) -> u32 {
    (1u32..).find(|x| count_presents2(*x) >= min_presents).unwrap()
}

fn count_presents2(house: u32) -> u32 {
    assert!(house > 0);

    let v = tcorp_math_mods::factors::factors_for(house);

    // skip the factors that are less than house/50
    let mut split_idx = 0;
    for x in v.iter() {
        if *x * 50 >= house {
            break;
        }
        split_idx += 1;
    }
    let (_, factors) = v.split_at(split_idx);

    11 * factors.iter().fold(0, |acc, x| acc + x)
}

#[test]
fn test_count_presents2() {
    assert_eq!(11, count_presents2(1));
    assert_eq!(33, count_presents2(2));
    assert_eq!(44, count_presents2(3));
    assert_eq!(77, count_presents2(4));
    assert_eq!(66, count_presents2(5));
    assert_eq!(132, count_presents2(6));
    assert_eq!(88, count_presents2(7));
    assert_eq!(165, count_presents2(8));
    assert_eq!(143, count_presents2(9));
    assert_eq!(count_presents(50) / 10 * 11, count_presents2(50));
    assert_eq!(count_presents(51) / 10 * 11 - 11, count_presents2(51));
}

#[test]
fn test_find_lowest_house2() {
    assert_eq!(6, find_lowest_house2(100));
    assert_eq!(6, find_lowest_house2(132));
    assert_eq!(8, find_lowest_house2(133));
}

