// advent14.rs
// reindeer races

#[macro_use] extern crate scan_fmt;

use std::io;

fn main() {
    let mut reindeers = Vec::new();
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

        let (s, on, off) = scan_fmt!(
            &input,
            "{*} can fly {} km/s for {} seconds, but then must rest for {} seconds.", 
            u32, 
            u32, 
            u32);
        reindeers.push(Reindeer{speed: s.unwrap(), on_time: on.unwrap(), off_time: off.unwrap()});
    }

    const RACE_TIME: u32 = 2503;

    let max = calc_max_distance(&reindeers, RACE_TIME);
    println!("{} km", max);

    // part 2
    // unspecified what happens when two reindeer are tied: I'm guessing they both get the point
    let max2 = reindeers.iter()
        .map(|r| (1..RACE_TIME)
             .map(|t| if calc_distance(r, t) == calc_max_distance(&reindeers, t) {1} else {0})
             .fold(0, |acc, x| acc + x))
        .max().unwrap();

    println!("{} points", max2);
}

struct Reindeer {
    speed: u32,
    on_time: u32,
    off_time: u32,
}

fn calc_distance(reindeer: &Reindeer, duration: u32) -> u32 {
    let cycle_time = reindeer.on_time + reindeer.off_time;
    let cycles = duration / cycle_time;
    let remainder = duration % cycle_time;

    reindeer.speed * (cycles * reindeer.on_time + std::cmp::min(reindeer.on_time, remainder))
}

fn calc_max_distance(reindeers: &Vec<Reindeer>, duration: u32) -> u32 {
    reindeers.iter().map(|r| calc_distance(r, duration)).max().unwrap()
}

#[test]
fn test_calc_distance() {
    let comet = Reindeer {speed: 14, on_time: 10, off_time: 127};
    let dancer = Reindeer {speed: 16, on_time: 11, off_time: 162};

    assert_eq!(14, calc_distance(&comet, 1));
    assert_eq!(16, calc_distance(&dancer, 1));
    assert_eq!(140, calc_distance(&comet, 10));
    assert_eq!(160, calc_distance(&dancer, 10));
    assert_eq!(140, calc_distance(&comet, 11));
    assert_eq!(176, calc_distance(&dancer, 11));
    assert_eq!(1120, calc_distance(&comet, 1000));
    assert_eq!(1056, calc_distance(&dancer, 1000));
}

#[test]
fn test_calc_max_distance() {
    let v = vec![Reindeer {speed: 14, on_time: 10, off_time: 127}, 
        Reindeer {speed: 16, on_time: 11, off_time: 162}];

    assert_eq!(1120, calc_max_distance(&v, 1000));
}


