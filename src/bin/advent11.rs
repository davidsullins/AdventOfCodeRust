// advent11.rs
// password security

extern crate pcre;

use std::io;

fn main() {
    // Part 1
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let next_password = find_next_password(&input);
    println!("Next password after {} is {}", input, next_password);
    // part 2
    let next_password2 = find_next_password(&next_password);
    println!("Next password after {} is {}", next_password, next_password2);
}

// assumption: input is all lowercase letters a-z

fn find_next_password(s: &str) -> String {
    let mut candidate = String::from(s);

    candidate = increment(&candidate);
    while !is_password_secure(&candidate) {
        candidate = increment(&candidate);
    }

    candidate
}

fn increment(s: &str) -> String {
    const ASCII_LOWER_A: u8 = 97;
    const ASCII_LOWER_Z: u8 = 122;
    let mut result = vec![0u8; s.len()];

    let mut carry = 1;
    for (i, c) in s.bytes().enumerate().rev() {
        if c + carry > ASCII_LOWER_Z {
            result[i] = ASCII_LOWER_A;
            carry = 1;
        } else {
            result[i] = c + carry;
            carry = 0;
        }
    }

    String::from_utf8(result).unwrap()
}


fn is_password_secure(s: &str) -> bool {
    is_run_of_three(s) && has_no_confusing_letters(s) && has_two_doubles(s)
}

fn is_run_of_three(s: &str) -> bool {
    // treat it as an array of bytes to get windows()
    s.as_bytes().windows(3).any(|x| x[0] + 1 == x[1] && x[1] + 1 == x[2])
}

fn has_no_confusing_letters(s: &str) -> bool {
    !(s.contains('i') || s.contains('l') || s.contains('o'))
}

fn has_two_doubles(s: &str) -> bool {
    // Oops, this doesn't work. I give up, just use regular expressions instead
    //s.as_bytes().windows(2).fold(0u32, |acc, x| if x[0] == x[1] { acc + 1 } else { acc }) > 1
    
    let mut re = pcre::Pcre::compile("([a-z])\\1.*([a-z])\\2").unwrap();
    re.exec(s).is_some()
}

#[test]
fn test_is_run_of_three() {
    assert!(is_run_of_three("abc"));
    assert!(is_run_of_three("xyz"));
    assert!(!is_run_of_three("yza"));
    assert!(!is_run_of_three("abd"));
    assert!(is_run_of_three("hijklmmn"));
    assert!(!is_run_of_three("abbceffg"));
}

#[test]
fn test_has_no_confusing_letters() {
    assert!(has_no_confusing_letters("abcd"));
    assert!(!has_no_confusing_letters("abcid"));
    assert!(!has_no_confusing_letters("labcd"));
    assert!(!has_no_confusing_letters("abcdo"));
    assert!(!has_no_confusing_letters("hijklmmn"));
}

#[test]
fn test_has_two_doubles() {
    assert!(has_two_doubles("aabb"));
    assert!(has_two_doubles("aacbb"));
    assert!(!has_two_doubles("aabcb"));
    assert!(has_two_doubles("abbceffg"));
    assert!(!has_two_doubles("abbcegjk"));
}

#[test]
fn test_is_password_secure() {
    assert!(!is_password_secure("hijklmmn"));
    assert!(!is_password_secure("abbceffg"));
    assert!(!is_password_secure("abbcegjk"));
    assert!(is_password_secure("abcdffaa"));
    assert!(is_password_secure("ghjaabcc"));
    assert!(!is_password_secure("abcdeggg"));
}

#[test]
fn test_find_next_password() {
    assert_eq!("abcdffaa", find_next_password("abcdefgh"));
    assert_eq!("ghjaabcc", find_next_password("ghijklmn"));
}

#[test]
fn test_increment() {
    assert_eq!("abce", increment("abcd"));
    assert_eq!("abda", increment("abcz"));
}

