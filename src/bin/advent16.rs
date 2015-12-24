// advent16.rs
// find Aunt Sue

extern crate pcre;

use std::io;

fn main() {
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

        if does_sue_match(&input) {
            println!("Found match: {}", input.trim());
        }
        if does_sue_match2(&input) {
            println!("Found match part 2: {}", input.trim());
        }
    }
}

fn does_sue_match(s: &str) -> bool {
    check_match_equal(s, "children", 3) 
        && check_match_equal(s, "cats", 7) 
        && check_match_equal(s, "samoyeds", 2)
        && check_match_equal(s, "pomeranians", 3)
        && check_match_equal(s, "akitas", 0)
        && check_match_equal(s, "vizslas", 0)
        && check_match_equal(s, "goldfish", 5)
        && check_match_equal(s, "trees", 3)
        && check_match_equal(s, "cars", 2)
        && check_match_equal(s, "perfumes", 1)
}

fn check_match_equal(s: &str, property: &str, value: u32) -> bool {
    check_match(s, property, value, |x,y| x == y)
}

// returns false if it has the property and it doesn't match the value
// returns true if the property doesn't exist or it exists but doesn't match
fn check_match<F>(s: &str, property: &str, value: u32, f: F) -> bool
    where F: Fn(u32, u32) -> bool {
    let mut re = pcre::Pcre::compile(&format!("{}: (\\d+)", property)).unwrap();
    if let Some(m) = re.exec(s) {
        assert!(m.string_count() > 1);
        f(m.group(1).parse::<u32>().unwrap(), value)
    } else {
        // property doesn't exist
        true
    }
}

#[test]
fn test_check_match_equal() {
    let s = "junk, foo: 4, bar: 5";
    assert!(check_match_equal(s, "foo", 4));
    assert!(check_match_equal(s, "bar", 5));
    assert!(!check_match_equal(s, "foo", 3));
    assert!(check_match_equal(s, "string that isn't even there", 5));
}

// part 2
fn check_match_less(s: &str, property: &str, value: u32) -> bool {
    check_match(s, property, value, |x,y| x < y)
}

fn check_match_greater(s: &str, property: &str, value: u32) -> bool {
    check_match(s, property, value, |x,y| x > y)
}

fn does_sue_match2(s: &str) -> bool {
    check_match_equal(s, "children", 3) 
        && check_match_greater(s, "cats", 7) 
        && check_match_equal(s, "samoyeds", 2)
        && check_match_less(s, "pomeranians", 3)
        && check_match_equal(s, "akitas", 0)
        && check_match_equal(s, "vizslas", 0)
        && check_match_less(s, "goldfish", 5)
        && check_match_greater(s, "trees", 3)
        && check_match_equal(s, "cars", 2)
        && check_match_equal(s, "perfumes", 1)
}

