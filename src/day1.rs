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
        #[allow(unused_assignments)]
        let mut temp: String = String::with_capacity(100);
        // let mut temp_chars: Chars = Chars::new();

        // filter through the string as an iterator of characters and remove anything that is not a number. Order is kept.
        temp = single_line
            .chars()
            .filter_map(|x| x.is_numeric().then_some(x))
            .collect();
        // convert the last number to a number type (u32) with base 10.
        let num_ones_place: u32 = temp.chars().last().as_ref().unwrap().to_digit(10).unwrap();
        // reverse the string of charagers and convert the first number to a number type (u32) with base ten to get the 10s place.
        let num_tens_place: u32 = temp
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
    // Return a result of Ok with the value.
    Ok(sum as i128)
}

/// Main function to execute Day One Challenge Two
pub fn day_1_challenge_2(config: &Config) -> Result<i128, Error> {
    // Open the puzzle input to a buffer and create a vector of single line strings.
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();
    let mut sum: usize = 0;
    // Initiate an array of string slices. Arrays in Rust are uncommon compared to Vectors.
    const WORD_NUMBERS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    // iterate over the vector of strings line by line.
    for single_line in vec_strings.into_iter() {
        let temp10s: usize;
        let temp1s: usize;
        // initiate a Vector of tuples of the position index and value to store the results.
        let mut decoded_postion_number: Vec<(usize, usize)> = Vec::new();
        // Iterate over the characters in the line with the position index  explicitly tracted.
        // dbg!(&single_line);
        for (position_index, character) in single_line.chars().enumerate() {
            // test if the character is numeric, and if so, push the position index on the string
            // with the value on to the vector.
            // index in the string.
            if character.is_numeric() {
                let digit: usize = character.to_digit(10).unwrap() as usize;
                decoded_postion_number.push((position_index, digit));
            }
        }
        // test if the the string contains one of the number words, and if so, push the postion
        // and the value onto the vector.
        for (decoded_number, word) in WORD_NUMBERS.iter().enumerate() {
            if let Some(position_index) = single_line.find(word) {
                decoded_postion_number.push((position_index, decoded_number));
            }
            //check for the last occurance in case the same word is there twice
            if let Some(position_index) = single_line.rfind(word) {
                decoded_postion_number.push((position_index, decoded_number));
            }
        }
        // Sort the vector by position number so that the mix of decoded numbers and words are
        // correctly ordered.
        decoded_postion_number.sort_unstable_by_key(|x| x.0);
        // Pop the last value as the one's place.
        // decoded_postion_number.reverse();
        // if the vector has more than one value, reverse and pop the last value for the 10s place.
        if decoded_postion_number.len() > 1 {
            temp1s = decoded_postion_number.pop().unwrap().1;
            decoded_postion_number.reverse();
            temp10s = decoded_postion_number.pop().unwrap().1;
        } else {
            temp10s = decoded_postion_number.pop().unwrap().1;
            temp1s = temp10s;
        }
        // sum it all up in the loop.
        // dbg!(&temp10s, &temp1s);
        sum += temp10s * 10 + temp1s;
    }
    // Return a result of Ok with the value.
    Ok(sum as i128)
}

//  TEST FUNCTIONS.
//  Everything below is to configure and run the tests.
//  The tests take the sample input, execute the challenge code, and compare it to the expected
//  sample output.
//
//  These two functions return a Config struct for each challenge, which is defined in lib.rs
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

// The functions below run the tests by calling a maco that is defined in lib.rs
// The advantage of this approach is that the functions do not need rewriting each day.
#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(1, 1, test_config_d1_1());
    test_expected_computed!(1, 2, test_config_d1_2());
}
