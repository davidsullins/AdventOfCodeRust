// advent17.rs
// eggnog perfect bin packing

use std::io;

fn main() {
    let mut containers: Vec<u32> = Vec::new();
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
        
        // Parse container sizes from input
        if let Ok(x) = input.trim().parse::<u32>() {
            containers.push(x);
        }
    }

    println!("exact fits: {}", count_exact_fits(&containers, 150));
    println!("min exact fits: {}", count_min_exact_fits(&containers, 150));

}

// check if we can exactly get target size from combination of bins
fn is_exact_fit(bins: &[u32], bit_mask: u32, target: u32) -> bool {
    let total = bins.iter().enumerate()
        .fold(0, |sum, (i, bin)| 
              if bit_mask & (1 << i) == 0 { 
                  sum 
              } else { 
                  sum + bin 
              });
    total == target
}

fn count_exact_fits(bins: &[u32], target: u32) -> usize {
    // because I'm storing a bit mask in a u32
    assert!(bins.len() <= 32);

    (1u32..(1 << bins.len())).filter(|x| is_exact_fit(bins, *x, target)).count()
}


#[test]
fn test_is_exact_fit() {
    let bins = [20, 15, 10, 5, 5];
    assert!(is_exact_fit(&bins, 0b0_0110, 25));
    assert!(!is_exact_fit(&bins, 0b1_0110, 25));
}

#[test]
fn test_count_exact_fits() {
    let bins = [20, 15, 10, 5, 5];

    assert_eq!(4, count_exact_fits(&bins, 25));
}

// part 2
fn find_exact_fits(bins: &[u32], target: u32) -> Vec<u32> {
    // because I'm storing a bit mask in a u32
    assert!(bins.len() <= 32);

    (1u32..(1 << bins.len())).filter(|x| is_exact_fit(bins, *x, target)).collect()
}

fn count_bits(x: u32) -> u32 {
    let mut total = 0;
    let mut val = x;

    while val > 0 {
        if (val & 1) != 0 {
            total += 1;
        }
        val >>= 1;
    }

    total
}

fn count_min_exact_fits(bins: &[u32], target: u32) -> usize {
    let fit_bits: Vec<u32> = find_exact_fits(bins, target).iter().map(|x| count_bits(*x)).collect();
    let min_bits = fit_bits.iter().min().unwrap();
    
    fit_bits.iter().filter(|x| *x == min_bits).count()
}

#[test]
fn test_find_exact_fits() {
    let bins = [20, 15, 10, 5, 5];
    let expected_result: Vec<u32> = vec![0b0_0110, 0b0_1001, 0b1_0001, 0b1_1010];

    assert_eq!(expected_result, find_exact_fits(&bins, 25));
}

#[test]
fn test_count_min_exact_fits() {
    let bins = [20, 15, 10, 5, 5];

    assert_eq!(3, count_min_exact_fits(&bins, 25));
}

