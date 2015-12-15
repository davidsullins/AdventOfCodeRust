// advent10.rs
// look-and-say

use std::io;

fn main() {
    // Part 1 and 2
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let mut seq = input.trim().to_string();

    // part 1
    for _ in 0..40 {
        seq = look_and_say(&seq);
    }
    println!("length 40:{}", seq.len());

    // part 2
    for _ in 0..10 {
        seq = look_and_say(&seq);
    }
    println!("length 50:{}", seq.len());
}

// Part 1
fn look_and_say(s: &str) -> String {
    let mut say = String::new();
    if s.len() == 0 {
        return say;
    }

    let mut look = s.chars();

    let mut prev_char = look.next().unwrap();
    let mut run_length = 1;
    for c in look {
        if c != prev_char {
            say.push_str(&run_length.to_string());
            say.push(prev_char);
            run_length = 0;
            prev_char = c;
        }
        run_length += 1;
    }
    say.push_str(&run_length.to_string());
    say.push(prev_char);

    say
}

#[test]
fn test_part1() {
    assert_eq!("11", look_and_say("1"));
    assert_eq!("21", look_and_say("11"));
    assert_eq!("1211", look_and_say("21"));
    assert_eq!("111221", look_and_say("1211"));
    assert_eq!("312211", look_and_say("111221"));
}

