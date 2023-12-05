// Advent of Code Day THREEE
//
// Some imports not needed every time.
#![allow(dead_code)]
use aoc_23::*; //import custom lib.rs module
use grid::Grid;
use std::io::Error; // use std::process;
                    // use itertools::{Itertools, MultiPeek};
use std::collections::HashSet;

const DAY: u8 = 3;
const TEST1_EXPECTED_OUTPUT: &str = "4361";
const TEST2_EXPECTED_OUTPUT: &str = "9999";

const GRID_ROWS: usize = 144;
const GRID_COLS: usize = 144;

#[derive(Debug, Copy, Clone)]
pub struct PartNumber {
    row: usize, //first digit
    col: usize,
    len: usize,
    linked: bool,
    value: usize,
}


pub fn day_3_challenge_1(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();
    let mut input_grid: Grid<Option<char>> = Grid::new(GRID_ROWS, GRID_COLS);
    let mut part_mask_grid: Grid<Option<PartNumber>> = Grid::new(GRID_ROWS, GRID_COLS);
    let mut symbol_mask_grid: Grid<Option<bool>> = Grid::new(GRID_ROWS, GRID_COLS);

    // assemble the input grid
    for (vec_line_index, vec_line) in vec_strings.into_iter().enumerate() {
        for (vec_col_index, item) in vec_line.chars().enumerate() {
            if item == '.' {
                input_grid[vec_line_index][vec_col_index] = None;
            } else if item.is_digit(10) {
                input_grid[vec_line_index][vec_col_index] = Some(item);
                part_mask_grid[vec_line_index][vec_col_index] = Some(PartNumber {
                    row: vec_line_index,
                    col: vec_col_index,
                    len: 0,
                    linked: false,
                    value: 0,
                });
            } else {
                input_grid[vec_line_index][vec_col_index] = Some(item);
                symbol_mask_grid[vec_line_index][vec_col_index] = Some(false);
            }
        }
    }
    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            if symbol_mask_grid[row][col] == Some(false) {
                //chaning to true to show the mask as
                //set
                symbol_mask_grid[row - 1][col + 1] = Some(true);
                symbol_mask_grid[row - 1][col] = Some(true);
                symbol_mask_grid[row - 1][col - 1] = Some(true);
                symbol_mask_grid[row][col + 1] = Some(true);
                symbol_mask_grid[row][col] = Some(true);
                symbol_mask_grid[row][col - 1] = Some(true);
                symbol_mask_grid[row + 1][col + 1] = Some(true);
                symbol_mask_grid[row + 1][col] = Some(true);
                symbol_mask_grid[row + 1][col - 1] = Some(true);
            }
        }
    }

    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            if part_mask_grid[row][col].is_some_and(|x| x.linked == false) {
                if part_mask_grid[row][col + 1].is_some_and(|x| x.linked == false) {
                    if part_mask_grid[row][col + 2].is_some_and(|x| x.linked == false) {
                        let mut value: usize =
                            input_grid[row][col].unwrap().to_digit(10).unwrap() as usize * 100;
                        value +=
                            input_grid[row][col + 1].unwrap().to_digit(10).unwrap() as usize * 10;
                        value += input_grid[row][col + 2].unwrap().to_digit(10).unwrap() as usize;
                        part_mask_grid[row][col] = Some(PartNumber {
                            row,
                            col,
                            len: 3,
                            linked: true,
                            value,
                        });
                        part_mask_grid[row][col + 1] = Some(PartNumber {
                            row,
                            col,
                            len: 3,
                            linked: true,
                            value,
                        });
                        part_mask_grid[row][col + 2] = Some(PartNumber {
                            row,
                            col,
                            len: 3,
                            linked: true,
                            value,
                        });
                    } else {
                        let mut value: usize =
                            input_grid[row][col].unwrap().to_digit(10).unwrap() as usize * 10;
                        value += input_grid[row][col + 1].unwrap().to_digit(10).unwrap() as usize;
                        part_mask_grid[row][col] = Some(PartNumber {
                            row,
                            col,
                            len: 2,
                            linked: true,
                            value,
                        });
                        part_mask_grid[row][col + 1] = Some(PartNumber {
                            row,
                            col,
                            len: 2,
                            linked: true,
                            value,
                        });
                    }
                } else {
                    let value: usize =
                        input_grid[row][col].unwrap().to_digit(10).unwrap() as usize;
                    part_mask_grid[row][col] = Some(PartNumber {
                        row,
                        col,
                        len: 1,
                        linked: true,
                        value,
                    });
                }
            }
        }
    }
    // dbg!(&input_grid);
    // dbg!(&part_mask_grid);
    // dbg!(&symbol_mask_grid);
    let mut challenge_sum: usize = 0;
    let mut counted_coords: HashSet<(usize, usize)> = HashSet::new();
    //I tried my hardest to not use hashsets so early, but here we are.
    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            if part_mask_grid[row][col].is_some() && symbol_mask_grid[row][col].is_some() {
                counted_coords.insert((
                    part_mask_grid[row][col].unwrap().row,
                    part_mask_grid[row][col].unwrap().col,
                ));
            }
        }
    }
    for (row, col) in counted_coords.drain() {
        challenge_sum += part_mask_grid[row][col].unwrap().value;
    }
    Ok(challenge_sum as i128)
}

pub fn day_3_challenge_2(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();

    let test_pass: i128 = 0;
    Ok(test_pass)
}

pub fn test_config_d3() -> Config {
    let test_config: Config = Config {
        challenge: 3,
        filename: "./input/test03.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(3, 1, test_config_d3());
    test_expected_computed!(3, 2, test_config_d3());
}
