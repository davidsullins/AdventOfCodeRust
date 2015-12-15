// advent6.rs
// christmas light grid

extern crate bit_vec;
#[macro_use] extern crate scan_fmt;

use bit_vec::BitVec;
use std::io;

#[cfg(not(test))]
const GRID_SIZE: usize = 1000;

#[cfg(test)]
const GRID_SIZE: usize = 8;

fn main() {
    // Part 1 initialization
    // Initialize grid of lights, all off
    let row_default = BitVec::from_elem(GRID_SIZE, false);
    let mut grid: Vec<BitVec> = Vec::new();
    for _ in 0..GRID_SIZE {
        grid.push(row_default.clone());
    }

    // Part 2 initialization
    let mut grid2 = [[0u32; GRID_SIZE]; GRID_SIZE];

    // Part 1 and 2, Parse input
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

        parse_command(&input, &mut grid);
        parse_command2(&input, &mut grid2);
    }

    // Part 1, Count lights
    println!("Total lights: {}", count_bits(&grid));

    // Part 2, Total brightness
    println!("Total brightness: {}", calc_total_brightness(&grid2));
    

}

// Example inputs:
// "toggle 461,550 through 564,900"
// "turn off 812,389 through 865,874"
// "turn on 599,989 through 806,993"
//
// Representation: 
// Vec of 1000 BitVecs, each with width 1000
fn parse_command(cmd: &str, grid: &mut Vec<BitVec>) {
    if cmd.contains("toggle") {
        apply_cmd(cmd, grid, "toggle ", |row, i| { 
            let val = row.get(i).unwrap();
            row.set(i, !val); });
    } else if cmd.contains("off") {
        apply_cmd(cmd, grid, "turn off ", |row, i| row.set(i, false));
    } else if cmd.contains("on") {
        apply_cmd(cmd, grid, "turn on ", |row, i| row.set(i, true));
    }
}

struct Range {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn apply_cmd<F>(cmd: &str, grid: &mut Vec<BitVec>, cmd_head: &str, f: F)
    where F: Fn(&mut BitVec, usize) {
    let cmd_tail = cmd.trim_left_matches(cmd_head);
    let (x1, y1, x2, y2) = scan_fmt!(cmd_tail, "{},{} through {},{}", usize, usize, usize, usize);
    let range = Range {x1: x1.unwrap(), y1: y1.unwrap(), x2: x2.unwrap(), y2: y2.unwrap()};
    let rows = &mut grid[range.y1..range.y2+1];
    for row in rows {
        for i in range.x1..range.x2+1 {
            f(row, i);
        }
    }
}

fn count_bits(grid: &Vec<BitVec>) -> usize {
    let mut total_bits = 0;
    for row in grid {
        total_bits += row.iter().filter(|x| *x).count();
    }

    total_bits
}

#[test]
fn test_part1() {
    let bv = BitVec::from_elem(8, false);
    let mut grid: Vec<BitVec> = Vec::new();
    for _ in 0..8 {
        grid.push(bv.clone());
    }

    let bv00 = BitVec::from_bytes(&[0]);
    let bv20 = BitVec::from_bytes(&[0x20]);
    let bv30 = BitVec::from_bytes(&[0x30]);
    let bv40 = BitVec::from_bytes(&[0x40]);
    let bv48 = BitVec::from_bytes(&[0x48]);

    let mut grid1: Vec<BitVec> = Vec::new();
    grid1.push(bv00.clone());
    grid1.push(bv00.clone());
    grid1.push(bv30.clone());
    grid1.push(bv30.clone());
    grid1.push(bv30.clone());
    grid1.push(bv30.clone());
    grid1.push(bv00.clone());
    grid1.push(bv00.clone());

    let mut grid2: Vec<BitVec> = Vec::new();
    grid2.push(bv00.clone());
    grid2.push(bv00.clone());
    grid2.push(bv48.clone());
    grid2.push(bv48.clone());
    grid2.push(bv48.clone());
    grid2.push(bv30.clone());
    grid2.push(bv00.clone());
    grid2.push(bv00.clone());

    let mut grid3: Vec<BitVec> = Vec::new();
    grid3.push(bv00.clone());
    grid3.push(bv00.clone());
    grid3.push(bv48.clone());
    grid3.push(bv48.clone());
    grid3.push(bv40.clone());
    grid3.push(bv20.clone());
    grid3.push(bv00.clone());
    grid3.push(bv00.clone());

    assert_eq!(count_bits(&grid), 0);
    parse_command("turn on 2,2 through 3,5", &mut grid);
    assert_eq!(grid, grid1);
    assert_eq!(count_bits(&grid), 8);
    parse_command("toggle 1,2 through 4,4", &mut grid);
    assert_eq!(grid, grid2);
    assert_eq!(count_bits(&grid), 8);
    parse_command("turn off 3,4 through 4,6", &mut grid);
    assert_eq!(grid, grid3);
    assert_eq!(count_bits(&grid), 6);
}

fn parse_command2(cmd: &str, grid: &mut [[u32; GRID_SIZE]; GRID_SIZE]) {
    if cmd.contains("toggle") {
        apply_cmd2(cmd, grid, "toggle ", Box::new(|x| *x += 2));
    } else if cmd.contains("off") {
        apply_cmd2(cmd, grid, "turn off ", Box::new(|x| *x = if *x > 0 { *x - 1 } else { *x }));
    } else if cmd.contains("on") {
        apply_cmd2(cmd, grid, "turn on ", Box::new(|x| *x += 1));
    }


}

fn apply_cmd2(cmd: &str, grid: &mut [[u32; GRID_SIZE]; GRID_SIZE], cmd_head: &str, f: Box<Fn(&mut u32)>) {
    let cmd_tail = cmd.trim_left_matches(cmd_head);
    let (x1, y1, x2, y2) = scan_fmt!(cmd_tail, "{},{} through {},{}", usize, usize, usize, usize);
    let range = Range {x1: x1.unwrap(), y1: y1.unwrap(), x2: x2.unwrap(), y2: y2.unwrap()};
    let rows = &mut grid[range.y1..range.y2+1];
    for row in rows {
        let lights = &mut row[range.x1..range.x2+1];
        for light in lights {
            f(light);
        }
    }
}

fn calc_total_brightness(grid: &[[u32; GRID_SIZE]; GRID_SIZE]) -> u32 {
    grid.iter().fold(0, |x,row| x + row.iter().fold(0, |y,z| y + z))
}

#[test]
fn test_part2() {
    let mut grid = [[0u32; GRID_SIZE]; GRID_SIZE];

    parse_command2("turn on 0,0 through 0,0", &mut grid);
    assert_eq!(1, calc_total_brightness(&grid));
    parse_command2("turn off 0,0 through 0,0", &mut grid);
    assert_eq!(0, calc_total_brightness(&grid));
    parse_command2("turn on 2,2 through 3,5", &mut grid);
    assert_eq!(8, calc_total_brightness(&grid));
    parse_command2("toggle 1,2 through 4,4", &mut grid);
    assert_eq!(32, calc_total_brightness(&grid));
    parse_command2("turn off 3,4 through 4,6", &mut grid);
    assert_eq!(29, calc_total_brightness(&grid));
}

