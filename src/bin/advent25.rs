// advent25.rs
// copy protection code

#[macro_use] extern crate scan_fmt;

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let (row, col) = scan_fmt!(
        &input, 
        "To continue, please consult the code grid in the manual.  Enter the code at row {}, column {}.", 
        u32, u32);

    println!("code = {}", get_code(col.unwrap(), row.unwrap()));
}

fn get_triangular(x: u32) -> u32 {
    x * (x + 1) / 2
}

fn get_ordinal(x: u32, y: u32) -> u32 {
    x + get_triangular(x + y - 2)
}

fn get_code(x: u32, y: u32) -> u64 {
    const START_VAL: u64 = 20151125;
    const MULT_VAL: u64 = 252533;
    const MOD_VAL: u64 = 33554393;

    let mut val = START_VAL;

    for _ in 1..get_ordinal(x, y) {
        val = val * MULT_VAL % MOD_VAL;
    }

    val
}

#[test]
fn test_get_ordinal() {
    assert_eq!(1, get_ordinal(1, 1));
    assert_eq!(2, get_ordinal(1, 2));
    assert_eq!(3, get_ordinal(2, 1));
    assert_eq!(4, get_ordinal(1, 3));
    assert_eq!(5, get_ordinal(2, 2));
    assert_eq!(18, get_ordinal(3, 4));
    assert_eq!(21, get_ordinal(6, 1));
}

#[test]
fn test_get_code() {
    assert_eq!(20151125, get_code(1, 1));
    assert_eq!(77061, get_code(1, 5));
    assert_eq!(1534922, get_code(5, 6));
}

