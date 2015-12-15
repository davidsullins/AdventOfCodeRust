// advent4.rs
// mining Advent Coins

extern crate md5;
use std::io;

fn main() {
    // Part 1 and 2
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let input = input.trim();

    let key = find_key(&input);
    println!("Key: {}", key);

}

fn find_key(input: &str) -> String {
    for i in 0.. {
        let mut candidate: String = input.to_string();
        candidate.push_str(&i.to_string());
        if is_md5_candidate(&candidate) {
            println!("Found candidate: {}", i);
            return i.to_string();
        } 
    }
    unreachable!();
}

fn is_md5_candidate(guess: &str) -> bool {
    let md5sum = md5::compute(guess.as_bytes());
    // commented out part 1
    //if md5sum[0] == 0 && md5sum[1] == 0 && md5sum[2] <= 0xf {
    if md5sum[0] == 0 && md5sum[1] == 0 && md5sum[2] == 0 {
        true
    } else {
        false
    }
}

// Commented out, these are part 1 tests
//#[test]
//fn test_find_key() {
//    assert_eq!("609043", find_key("abcdef"));
//    assert_eq!("1048970", find_key("pqrstuv"));
//}
//
//#[test]
//fn test_is_md5_candidate() {
//    assert!(is_md5_candidate("abcdef609043"));
//    assert!(is_md5_candidate("pqrstuv1048970"));
//    assert!(!is_md5_candidate("pqrstuv1048969"));
//}

