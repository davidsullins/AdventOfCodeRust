// advent12.rs
// JSON

extern crate pcre;

use std::io;

fn main() {
    // Part 1
    let mut input = String::new();

    // input is all one line
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let total = sum_nums(&input);
    println!("{}", total);
}

fn sum_nums(s: &str) -> i32 {
    s.split(|x| !(x >= '0' && x <= '9' || x == '-')).fold(0i32, |acc, x| 
        if x.len() > 0 {
            acc + x.parse::<i32>().unwrap()
        } else {
            acc
        })
}

#[test]
fn test_sum_nums() {
    assert_eq!(6, sum_nums("[1,2,3]"));
    assert_eq!(6, sum_nums("{\"a\":2,\"b\":4}"));
    assert_eq!(3, sum_nums("[[[3]]]"));
    assert_eq!(3, sum_nums("{\"a\":{\"b\":4},\"c\":-1}"));
    assert_eq!(0, sum_nums("{\"a\":[-1,1]}"));
    assert_eq!(0, sum_nums("[-1,{\"a\":1}]"));
    assert_eq!(0, sum_nums("[]"));
    assert_eq!(0, sum_nums("{}"));
}

// part 2
fn sum_nums2(s: &str) -> i32 {
    // TBD
    sum_nums(s)
}

#[test]
fn test_sum_nums2() {
    assert_eq!(6, sum_nums2("[1,2,3]"));
    assert_eq!(4, sum_nums2("[1,{\"c\":\"red\",\"b\":2},3]"));
    assert_eq!(0, sum_nums2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"));
    assert_eq!(6, sum_nums2("[1,\"red\",5]"));


}
