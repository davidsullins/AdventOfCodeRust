// advent15.rs
// cookie recipe

#[macro_use] extern crate scan_fmt;

#[cfg(not(test))]
use std::io;
use std::cmp::max;

#[cfg(not(test))]
fn main() {
    let mut ingredients = Vec::new();
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

        let (x1, x2, x3, x4, x5) = scan_fmt!(
            &input,
            "{*}: capacity {}, durability {}, flavor {}, texture {}, calories {}",
            i64,
            i64,
            i64,
            i64,
            i64);
        let p: Properties = [x1.unwrap(), x2.unwrap(), x3.unwrap(), x4.unwrap(), x5.unwrap()];
        ingredients.push(p);
    }

    let max_score = calc_max_score4(&ingredients);
    println!("Max score: {}", max_score);

    let max_score2 = calc_max_score4_part2(&ingredients);
    println!("Max score part 2: {}", max_score2);
}

type Properties = [i64; 5];

fn calc_score(ingredients: &[Properties], quantities: &[i64]) -> i64 {
    (0..4)
        .map(|p| ingredients.iter().zip(quantities.iter())
             .map(|(i, q)| i[p] * q)
             .fold(0, |acc, x| acc + x))
        .fold(1, |acc, x| acc * max(0, x))
}

#[cfg(not(test))]
fn calc_max_score4(ingredients: &[Properties]) -> i64 {
    assert!(ingredients.len() == 4);

    const MAX_TSP: i64 = 100;
    
    let mut max_score = 0;

    // 4 ingredients
    for w in 0..(MAX_TSP + 1) {
        for x in 0..(MAX_TSP + 1 - w) {
            for y in 0..(MAX_TSP + 1 - w - x) {
                let z = MAX_TSP - w - x - y;
                let quantities = vec![w, x, y, z];
                max_score = max(max_score, calc_score(ingredients, &quantities));
            }
        }
    }

    max_score
}

#[cfg(test)]
fn calc_max_score2(ingredients: &[Properties]) -> i64 {
    assert!(ingredients.len() == 2);

    const MAX_TSP: i64 = 100;
    
    let mut max_score = 0;

    // 2 ingredients
    for x in 0..(MAX_TSP + 1) {
        let y = MAX_TSP - x;
        let quantities = vec![x, y];
        max_score = max(max_score, calc_score(ingredients, &quantities));
    }

    max_score
}

#[test]
fn test_calc_score() {
    let butterscotch: Properties = [-1, -2, 6, 3, 8];
    let cinnamon: Properties = [2, 3, -2, -1, 3];

    let ingredients = vec![butterscotch, cinnamon];
    let quantities = vec![44, 56];

    assert_eq!(62842880, calc_score(&ingredients, &quantities));
}

#[test]
fn test_calc_max_score2() {
    let butterscotch: Properties = [-1, -2, 6, 3, 8];
    let cinnamon: Properties = [2, 3, -2, -1, 3];

    let ingredients = vec![butterscotch, cinnamon];

    assert_eq!(62842880, calc_max_score2(&ingredients));
}

// part 2

#[cfg(not(test))]
fn calc_score_part2(ingredients: &[Properties], quantities: &[i64]) -> i64 {
    const TARGET_CALORIES: i64 = 500;

    let total_calories = ingredients.iter().zip(quantities.iter())
        .map(|(i, q)| i[4] * q)
        .fold(0, |acc, x| acc + x);

    if total_calories == TARGET_CALORIES {
        calc_score(ingredients, quantities)
    } else {
        0
    }
}

#[cfg(not(test))]
fn calc_max_score4_part2(ingredients: &[Properties]) -> i64 {
    assert!(ingredients.len() == 4);

    const MAX_TSP: i64 = 100;
    
    let mut max_score = 0;

    // 4 ingredients
    for w in 0..(MAX_TSP + 1) {
        for x in 0..(MAX_TSP + 1 - w) {
            for y in 0..(MAX_TSP + 1 - w - x) {
                let z = MAX_TSP - w - x - y;
                let quantities = vec![w, x, y, z];
                max_score = max(max_score, calc_score_part2(ingredients, &quantities));
            }
        }
    }

    max_score
}

