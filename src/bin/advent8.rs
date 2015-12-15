// advent8.rs
// Santa's list

use std::io;

fn main() {
    // Part 1
    let mut total_mem_diff = 0;
    let mut total_mem_diff2 = 0;
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

        total_mem_diff += count_mem_diff(&input);
        total_mem_diff2 += count_mem_diff2(&input);
    }

    println!("Total difference decoding: {}", total_mem_diff);
    println!("Total difference encoding: {}", total_mem_diff2);
   

}

// My assumptions: 
// * all strings are correctly quoted
// * \x is always correctly followed by two hexadecimal characters
// * all other instances of \ are followed by a single character to be escaped
//
// Therefore: 
// string length in memory =
//  original string length
//  - 2 (for quotes)
//  - 3 * number of occurences of \x
//  - number of occurences of \ anything else
//
//  Or simplifying further:
//  original - 2 - 2*[\x] - [\]
//
//  so the difference is merely 2 + 2*[\x] + [\]
//
//  Oops, that doesn't work in the case of "\\x00" because of the escaped \\
//  Need to account for every case of an even number of \ before the x
fn _count_mem_diff_wrong(s: &str) -> usize {
    //s.len() - 2 - 2 * s.matches("\\x").count() - s.matches('\\').count()
    2 + 2 * s.matches("\\x").count() + s.matches('\\').count()
}

// Here's a correct but ugly solution. I feel I've failed. I want a one-liner like I had above
fn count_mem_diff(s: &str) -> usize {
    // initialize to 2 for the quotes
    let mut removed_chars = 2;

    let mut chars = s.chars().peekable();

    while chars.peek().is_some() {
        let c = chars.next().unwrap();
        if c == '\\' {
            removed_chars += 1;
            let c2 = chars.next().unwrap(); // making assumptions about well-formed input
            if c2 == 'x' {
                // discard next 2 chars
                chars.nth(1);
                removed_chars += 2;
            }
        }
    }

    removed_chars
}

#[test]
fn test_part1() {
    assert_eq!(2, count_mem_diff("\"\""));
    assert_eq!(2, count_mem_diff("\"abc\""));
    assert_eq!(3, count_mem_diff("\"aaa\\\"aaa\""));
    assert_eq!(5, count_mem_diff("\"\\x27\""));
    // "nb\\x\\uhn" -> 4
    assert_eq!(4, count_mem_diff("\"nb\\\\x\\\\uhn\""))
}

// Part 2

// 2nd part is simpler than first part
// Every string adds 2 characters to add quotes around it 
// Escape the original quotes wherever they appear
// Escape the original backslashes wherever they appear
fn count_mem_diff2(s: &str) -> usize {
    2 + s.matches('"').count() + s.matches('\\').count()
}

#[test]
fn test_part2() {
    assert_eq!(4, count_mem_diff2("\"\""));
    assert_eq!(4, count_mem_diff2("\"abc\""));
    assert_eq!(6, count_mem_diff2("\"aaa\\\"aaa\""));
    assert_eq!(5, count_mem_diff2("\"\\x27\""));
}


