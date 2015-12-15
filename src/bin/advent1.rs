use std::io;

fn main() {
    // Part 1
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let total2 = input.chars().map(paren_val).fold(0, std::ops::Add::add);

    println!("Total: {}", total2);

    // Part 2
    let basement = find_basement(&input);
    println!("First hit basement at position {}", basement);
}

fn paren_val(c: char) -> i32 {
        match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        }
}

fn find_basement(input: &str) -> i32 {
    let mut total = 0;
    let mut basement = 0;
    for c in input.chars() {
        basement += 1;
        total += paren_val(c);
        if total < 0 {
            return basement;
        }
    }
    return 0;   // 0 means no basement
}

#[test]
fn test_basement() {
    assert_eq!(0, find_basement(""));
    assert_eq!(0, find_basement("()"));
    assert_eq!(1, find_basement(")"));
    assert_eq!(5, find_basement("()())"));
}

