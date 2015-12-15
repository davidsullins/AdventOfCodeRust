// advent2.rs
//
// wrapping paper

use std::io;

fn main() {
    // Part 1 and 2
    let mut total = 0;
    let mut total_ribbon = 0;
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

        let dims = parse_lwh_str(&input);
        total += calc_area(dims.unwrap());
        total_ribbon += calc_ribbon_length(dims.unwrap());
    }
    println!("Total paper area: {}", total);
    println!("Total ribbon length: {}", total_ribbon);
}

fn calc_ribbon_length(dims: (u32, u32, u32)) -> u32 {
    let (l, h, w) = dims;
    // inelegant, should have used vec instead of tuple
    let dim1 = std::cmp::min(l, std::cmp::min(h, w));
    let dim2 = if l == dim1 {
        std::cmp::min(h, w)
    } else if h == dim1 {
        std::cmp::min(l, w)
    } else {
        std::cmp::min(l, h)
    };

    2 * (dim1 + dim2) + l * h * w
}

fn calc_area(dims: (u32, u32, u32)) -> u32 {
    let (l, h, w) = dims;
    2 * (l * h + l * w + h * w) + std::cmp::min(l*h, std::cmp::min(l*w, h*w))
}

fn parse_lwh_str(lwh_str: &str) -> Option<(u32, u32, u32)> {
    // Split string into a vec of strings
    let dims: Vec<&str> = lwh_str.trim().split('x').collect();
    
    // convert strings to vec of 3 Option<u32>, if we can
    if dims.len() != 3 {
        return None;
    }
    let dims = dims.into_iter().map(|x| x.parse().ok()).collect::<Vec<Option<u32>>>();

    // convert vec Option<u32> to vec u32, if we can
    if dims.iter().any(|x| x.is_none()) {
        return None;
    }
    let dims = dims.into_iter().map(|x| x.unwrap()).collect::<Vec<u32>>();

    // convert vec u32 to tuple
    Some((dims[0], dims[1], dims[2]))
}

#[test]
fn test_calc_ribbon_length() {
    assert_eq!(34, calc_ribbon_length((2,3,4)));
    assert_eq!(14, calc_ribbon_length((1,1,10)));
}

#[test]
fn test_calc_area() {
    assert_eq!(58, calc_area((2,3,4)));
    assert_eq!(43, calc_area((1,1,10)));
}

#[test]
fn test_parse_lwh_str() {
    assert_eq!(None, parse_lwh_str(""));
    assert_eq!(None, parse_lwh_str("2x3x"));
    assert_eq!(None, parse_lwh_str("2x3x5x"));
    assert_eq!(None, parse_lwh_str("2x3x5x300"));
    assert_eq!(Some((1, 2, 3)), parse_lwh_str("1x2x3"));
    assert_eq!(Some((1, 20, 333)), parse_lwh_str("1x20x333"));
}

