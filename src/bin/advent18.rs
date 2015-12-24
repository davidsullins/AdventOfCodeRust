// advent18.rs
// game of life

use std::io;
use std::cmp::min;

#[cfg(not(test))]
const GRID_SIZE: usize = 100;

#[cfg(test)]
const GRID_SIZE: usize = 6;

type LightGrid = [[bool; GRID_SIZE]; GRID_SIZE];

fn main() {
    let mut grid = [[false; GRID_SIZE]; GRID_SIZE];
    let mut row = 0;

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

        assert!(row < GRID_SIZE);
        for (i, c) in input.trim().char_indices() {
            grid[row][i] = match c {
                '.' => false,
                '#' => true,
                _ => panic!("Unexpected character!")
            };
        }

        row += 1;
    }

    assert!(row == GRID_SIZE);

    let mut grid2 = grid;

    for _ in 0..100 {
        grid = calc_new_grid(&grid);
        print_grid(&grid);
    }

    println!("Lights: {}", count_lights(&grid));
    
    set_corners(&mut grid2);
    for _ in 0..100 {
        grid2 = calc_new_grid2(&grid2);
        print_grid(&grid2);
    }

    println!("Lights part 2: {}", count_lights(&grid2));
}

// just for fun
fn print_grid(grid: &LightGrid) {
    for j in 0..GRID_SIZE {
        let mut row_string = String::new();
        for i in 0..GRID_SIZE {
            row_string.push(if grid[j][i] { '#' } else { '.' });
        }
        println!("{}", row_string);
    }
}

// count how many neighbor lights are on around point (x,y)
fn count_neighbors(grid: &LightGrid, x: usize, y: usize) -> usize {
    let x1 = if x == 0 { 0 } else { x - 1 };
    let x2 = min(x + 2, GRID_SIZE);
    let y1 = if y == 0 { 0 } else { y - 1 };
    let y2 = min(y + 2, GRID_SIZE);

    let mut total = 0;
    for i in x1..x2 {
        for j in y1..y2 {
            if (i != x || j != y) && grid[j][i] {
                total += 1;
            }
        }
    }
    total
}

// new state of the light at this location
fn new_light(grid: &LightGrid, x: usize, y: usize) -> bool {
    let neighbors = count_neighbors(grid, x, y);

    if grid[y][x] {
        neighbors == 2 || neighbors == 3
    } else {
        neighbors == 3
    }
}

fn calc_new_grid(grid: &LightGrid) -> LightGrid {
    let mut new_grid = [[false; GRID_SIZE]; GRID_SIZE];
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            new_grid[j][i] = new_light(grid, i, j);
        }
    }
    new_grid
}

fn count_lights(grid: &LightGrid) -> u32 {
    let mut total = 0;

    for row in grid.iter() {
        for light in row.iter() {
            if *light {
                total += 1;
            }
        }
    }

    total
}

#[test]
fn test_count_neighbors() {
    let grid: LightGrid = [
        [false, true, false, true, false, true],
        [false, false, false, true, true, false],
        [true, false, false, false, false, true],
        [false, false, true, false, false, false],
        [true, false, true, false, false, true],
        [true, true, true, true, false, false]];

    assert_eq!(1, count_neighbors(&grid, 0, 0));
    assert_eq!(2, count_neighbors(&grid, 0, 5));
    assert_eq!(1, count_neighbors(&grid, 5, 0));
    assert_eq!(1, count_neighbors(&grid, 5, 5));
    assert_eq!(2, count_neighbors(&grid, 1, 1));
}

#[test]
fn test_calc_new_grid() {
    const GRID_TEMPLATE: LightGrid = [
        [false, true, false, true, false, true],
        [false, false, false, true, true, false],
        [true, false, false, false, false, true],
        [false, false, true, false, false, false],
        [true, false, true, false, false, true],
        [true, true, true, true, false, false]];

    const GRID_SOLUTION: LightGrid = [
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, true, true, false, false],
        [false, false, true, true, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false]];

    let mut grid = GRID_TEMPLATE;
    for _ in 0..4 {
        grid = calc_new_grid(&grid);
    }

    assert_eq!(GRID_SOLUTION, grid);
    assert_eq!(4, count_lights(&grid));
}

// part 2

fn set_corners(grid: &mut LightGrid) {
   grid[0][0] = true; 
   grid[0][GRID_SIZE - 1] = true; 
   grid[GRID_SIZE - 1][0] = true; 
   grid[GRID_SIZE - 1][GRID_SIZE - 1] = true; 
}

fn calc_new_grid2(grid: &LightGrid) -> LightGrid {
    let mut new_grid = calc_new_grid(grid);
    set_corners(&mut new_grid);
    new_grid
}

#[test]
fn test_calc_new_grid2() {
    const GRID_TEMPLATE: LightGrid = [
        [false, true, false, true, false, true],
        [false, false, false, true, true, false],
        [true, false, false, false, false, true],
        [false, false, true, false, false, false],
        [true, false, true, false, false, true],
        [true, true, true, true, false, false]];

    const GRID_SOLUTION: LightGrid = [
        [true, true, false, true, true, true],
        [false, true, true, false, false, true],
        [false, true, true, false, false, false],
        [false, true, true, false, false, false],
        [true, false, true, false, false, false],
        [true, true, false, false, false, true]];

    let mut grid = GRID_TEMPLATE;
    set_corners(&mut grid);

    for _ in 0..5 {
        grid = calc_new_grid2(&grid);
    }

    assert_eq!(GRID_SOLUTION, grid);
    assert_eq!(17, count_lights(&grid));
}

