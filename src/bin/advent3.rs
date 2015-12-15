// advent3.rs
// delivering presents

use std::io;

fn main() {
    // Part 1
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");


    let total = house_count(&input);
    println!("Total: {}", total);

    // Part 2
    let total2 = house_count2(&input);
    println!("Total2: {}", total2);

}

#[derive(Debug)]
#[derive(Eq, PartialEq, Hash, Clone)]
struct Location {
    x: i32,
    y: i32,
}

fn house_count(input: &str) -> u32 {
    let mut current_location = Location {x: 0, y: 0};

    let mut locations = std::collections::HashSet::new();
    locations.insert(current_location.clone());

    for c in input.chars() {
        //println!("cur: {:?}", current_location);
        current_location = move_location(&current_location, c);
        locations.insert(current_location.clone());
    }

    locations.len() as u32
}

fn move_location(current: &Location, dir: char) -> Location {
    match dir {
        '>' => Location {x: current.x + 1, y: current.y},
        '<' => Location {x: current.x - 1, y: current.y},
        '^' => Location {x: current.x, y: current.y + 1},
        'v' => Location {x: current.x, y: current.y - 1},
         _ => Location {x: current.x, y: current.y},
    }
}

#[test]
fn test_house_count() {
    assert_eq!(2, house_count(">"));
    assert_eq!(4, house_count("^>v<"));
    assert_eq!(2, house_count("^v^v^v^v^v"));
}

// part 2
fn house_count2(input: &str) -> u32 {
    let mut santa_location = Location {x: 0, y: 0};
    let mut robo_location = Location {x: 0, y: 0};

    let mut locations = std::collections::HashSet::new();
    locations.insert(santa_location.clone());
    locations.insert(robo_location.clone());

    // too bad we don't have chunks() for string slices
    let mut santa = true;
    for c in input.chars() {
        if santa {
            santa_location = move_location(&santa_location, c);
            locations.insert(santa_location.clone());
        } else {
            robo_location = move_location(&robo_location, c);
            locations.insert(robo_location.clone());
        }
        santa = !santa;
    }

    locations.len() as u32
}

#[test]
fn test_house_count2() {
    assert_eq!(3, house_count2("^v"));
    assert_eq!(3, house_count2("^>v<"));
    assert_eq!(11, house_count2("^v^v^v^v^v"));
}

