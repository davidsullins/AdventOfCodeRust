// advent5.rs
// naughty or nice words?

extern crate pcre;

use std::io;

fn main() {
    // Part 1 & 2
    let mut total_nice = 0;
    let mut total_nice2 = 0;
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

        total_nice += if is_nice(&input) { 1 } else { 0 };
        total_nice2 += if is_nice2(&input) { 1 } else { 0 };
    }

    println!("part 1 nice strings: {}", total_nice);
    println!("part 2 nice strings: {}", total_nice2);
}

fn is_nice(s: &str) -> bool {
    has_three_vowels(s) && has_double_letter(s) && has_no_forbiddens(s)
}

fn has_three_vowels(s: &str) -> bool {
    let v: Vec<&str> = s.matches(|x| x == 'a' || x == 'e' || x == 'i' || x == 'o' || x == 'u').collect();
    v.len() >= 3
}

fn has_double_letter(s: &str) -> bool {
    // it's all ASCII input (I checked) and the solution is much more elegant with windows
    // I wish we had a windows iterator for &str
    s.as_bytes().windows(2).fold(false, |has_double, win| has_double || win[0] == win[1])
}

fn has_no_forbiddens(s: &str) -> bool {
    // forbidden: ab, cd, pq, or xy
    !(s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy"))
}

#[test]
fn test_is_nice() {
    assert!(is_nice("ugknbfddgicrmopn"));
    assert!(is_nice("aaa"));
    assert!(!is_nice("jchzalrnumimnmhp"));
    assert!(!is_nice("haegwjzuvuyypxyu"));
    assert!(!is_nice("dvszwmarrgswjxmb"));
}

#[test]
fn test_has_three_vowels() {
    assert!(has_three_vowels("aei"));
    assert!(has_three_vowels("absieniei"));
    assert!(!has_three_vowels("absin"));
}

#[test]
fn test_has_double_letter() {
    assert!(has_double_letter("aa"));
    assert!(has_double_letter("baa"));
    assert!(!has_double_letter("absin"));
}

// part 2

fn is_nice2(s: &str) -> bool {
    has_two_pairs(s) && has_xyx(s)
}

fn has_two_pairs(s: &str) -> bool {
    let mut re = pcre::Pcre::compile("([a-zA-Z][a-zA-Z]).*\\1").unwrap();
    re.exec(s).is_some()
}

fn has_xyx(s: &str) -> bool {
    let mut re = pcre::Pcre::compile("([a-zA-Z])[a-zA-Z]\\1").unwrap();
    re.exec(s).is_some()
}

#[test]
fn test_has_two_pairs() {
    assert!(has_two_pairs("qjhvhtzxzqqjkmpb"));
    assert!(has_two_pairs("xxyxx"));
    assert!(has_two_pairs("uurcxstgmygtbstg"));
    assert!(!has_two_pairs("ieodomkazucvgmuy"));
}

#[test]
fn test_has_xyx() {
    assert!(has_xyx("qjhvhtzxzqqjkmpb"));
    assert!(has_xyx("xxyxx"));
    assert!(!has_xyx("uurcxstgmygtbstg"));
    assert!(has_xyx("ieodomkazucvgmuy"));
}

#[test]
fn test_is_nice2() {
    assert!(is_nice2("qjhvhtzxzqqjkmpb"));
    assert!(is_nice2("xxyxx"));
    assert!(!is_nice2("uurcxstgmygtbstg"));
    assert!(!is_nice2("ieodomkazucvgmuy"));
}

