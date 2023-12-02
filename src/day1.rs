// Advent of Code Day ONE
//
// Some imports not needed every time.
#![allow(dead_code)]
use aoc_23::*; //import custom lib.rs module
use std::io::Error; // use std::process;
                    // use grid::Grid;
                    // use itertools::{Itertools, MultiPeek};
                    // use std::collections::{HashMap, HashSet};

const DAY: u8 = 1;
// Expected answer from challange small example input and result. Used by tests.
const TEST1_EXPECTED_OUTPUT: &str = "142";
const TEST2_EXPECTED_OUTPUT: &str = "281";

/// Main function to execute Day One Challenge One
pub fn day_1_challenge_1(config: &Config) -> Result<i128, Error> {
    // Open the puzzle input to a buffer and create a vector of single line strings.
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();
    let mut sum: u32 = 0;
    // iterate over the vector of strings line by line.
    for single_line in vec_strings.into_iter() {
        let mut temp: String = String::new();
        // let mut temp_chars: Chars = Chars::new();
        let num_ones_place: u32;
        let num_tens_place: u32;
        // filter through the string as an iterator of characters and remove anything that is not a number. Order is kept.
        temp = single_line
            .chars()
            .filter_map(|x| x.is_numeric().then(|| x))
            .collect();
        // convert the last number to a number type (u32) with base 10.
        num_ones_place = temp.chars().last().as_ref().unwrap().to_digit(10).unwrap();
        // reverse the string of charagers and convert the first number to a number type (u32) with base ten to get the 10s place.
        num_tens_place = temp
            .chars()
            .rev()
            .last()
            .as_ref()
            .unwrap()
            .to_digit(10)
            .unwrap();
        // add it all up in the loop
        sum += num_tens_place * 10 + num_ones_place;
    }
    Ok(sum as i128)
}

/// Main function to execute Day One Challenge Two
pub fn day_1_challenge_2(config: &Config) -> Result<i128, Error> {
    // Open the puzzle input to a buffer and create a vector of single line strings.
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();
    let mut sum: usize = 0;
    // Initiate an array of string slices
    let word_numbers2: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for single_line in vec_strings.into_iter() {
        let mut temp10s: usize = 0;
        let temp1s: usize;
        let mut decoded_postion_number: Vec<(usize, usize)> = Vec::new();
        for (position_index, character) in single_line.chars().enumerate() {
            if character.is_numeric() {
                let digit: usize = character.to_digit(10).unwrap() as usize;
                decoded_postion_number.push((position_index, digit));
            }
        }
        for (decoded_number, word) in word_numbers2.iter().enumerate() {
            if let Some(position_index) = single_line.find(word) {
                decoded_postion_number.push((position_index, decoded_number));
            }
        }
        decoded_postion_number.sort_unstable_by_key(|x| x.0);
        dbg!(&decoded_postion_number);
        temp1s = decoded_postion_number.pop().unwrap().1;
        if decoded_postion_number.len() > 0 {
            decoded_postion_number.reverse();
            temp10s = decoded_postion_number.pop().unwrap().1;
        }
        sum += temp10s * 10 + temp1s;
    }
    Ok(sum as i128)
}

pub fn test_config_d1_1() -> Config {
    let test_config: Config = Config {
        challenge: 1,
        filename: "./input/test01.txt".to_string(),
    };
    test_config
}
pub fn test_config_d1_2() -> Config {
    let test_config: Config = Config {
        challenge: 1,
        filename: "./input/test01b.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(1, 1, test_config_d1_1());
    test_expected_computed!(1, 2, test_config_d1_2());
}
