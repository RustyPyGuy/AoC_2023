// Advent of Code Day TWO
//
#![allow(dead_code)]
use aoc_23::*; //import custom lib.rs module
use std::io::Error; // use std::process;

const DAY: u8 = 0;
const TEST1_EXPECTED_OUTPUT: &str = "8";
const TEST2_EXPECTED_OUTPUT: &str = "2286";

#[derive(Debug, Copy, Clone)]
pub struct CubeGame {
    game_num: usize,
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeGame {
    pub fn new() -> Result<CubeGame, &'static str> {
        let game_num: usize = 0;
        let red: usize = 0;
        let green: usize = 0;
        let blue: usize = 0;
        Ok(CubeGame {
            game_num,
            red,
            green,
            blue,
        })
    }
}

pub fn day_2_challenge_1(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();
    let mut challenge_sum: usize = 0;
    const GAME_COLORS: [&str; 3] = ["red", "green", "blue"];
    for single_line in vec_strings.into_iter() {
        // println!("\n input {}", &single_line);
        // dbg!(&single_line);
        let mut current_game = CubeGame::new().unwrap();
        let mut current_game_possible: bool = true;
        let mut split_colon: Vec<&str> = single_line.split(|c| c == ':').collect();
        split_colon.reverse();

        let first_element = split_colon.pop().unwrap().chars();
        current_game.game_num = first_element
            .filter_map(|x| x.is_numeric().then(|| x))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        // Pop the single element off the vector stack and spit again.
        let split_semicolon: Vec<&str> = split_colon.pop().unwrap().split(|c| c == ';').collect();

        for sub_game in split_semicolon.into_iter() {
            let mut current_sub_game = CubeGame::new().unwrap();

            let sub_game_color: Vec<&str> = sub_game.split(|c| c == ',').collect();
            for color_compare in sub_game_color.into_iter() {
                for color in GAME_COLORS.into_iter() {
                    if color_compare.find(color).is_some() {
                        let element_value = color_compare
                            .chars()
                            .filter_map(|x| x.is_numeric().then(|| x))
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap();
                        if color == "red" {
                            current_sub_game.red += element_value;
                        } else if color == "blue" {
                            current_sub_game.blue += element_value;
                        } else if color == "green" {
                            current_sub_game.green += element_value;
                        }
                    }
                }
            }

            // only 12 red cubes, 13 green cubes, and 14 blue cubes.
            if current_sub_game.red <= 12
                && current_sub_game.green <= 13
                && current_sub_game.blue <= 14
            {
                // println!(
                //     "Sub Game possible. game_num: {}  RGB: {} {} {}",
                //     &current_sub_game.game_num,
                //     current_sub_game.red,
                //     current_sub_game.green,
                //     current_sub_game.blue
                // );
                // challenge_sum += current_game.game_num;
                current_game_possible = current_game_possible && true;
            } else {
                current_game_possible = false;
            }
        }
        if current_game_possible {
            // println!("Game possible. game_num: {}", &current_game.game_num);
            challenge_sum += current_game.game_num;
        }
        // else {
        //     println!("Game NOT possible. game num {}", &current_game.game_num);
        // }
    }
    Ok(challenge_sum as i128)
}

pub fn day_2_challenge_2(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();
    // let game_vec: Vec<CubeGame> = Vec::with_capacity(100);
    let mut challenge_sum: usize = 0;
    const GAME_COLORS: [&str; 3] = ["red", "green", "blue"];
    for single_line in vec_strings.into_iter() {
        // println!("\n input {}", &single_line);
        let mut current_game_min_cubes = CubeGame::new().unwrap();
        let mut split_colon: Vec<&str> = single_line.split(|c| c == ':').collect();
        split_colon.reverse();

        // current_game.game_num = line_elements.pop().unwrap().chars().filter_map(|x| x.to_digit(10)).collect().parse() as usize;
        let first_element = split_colon.pop().unwrap().chars();
        current_game_min_cubes.game_num = first_element
            .filter_map(|x| x.is_numeric().then(|| x))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        // split_colon.deref().split(|c| c== &";").collect();
        // Pop the single element off the vector stack and spit again.
        let split_semicolon: Vec<&str> = split_colon.pop().unwrap().split(|c| c == ';').collect();

        for sub_game in split_semicolon.into_iter() {
            let sub_game_color: Vec<&str> = sub_game.split(|c| c == ',').collect();
            for color_compare in sub_game_color.into_iter() {
                for color in GAME_COLORS.into_iter() {
                    if color_compare.find(color).is_some() {
                        let element_value = color_compare
                            .chars()
                            .filter_map(|x| x.is_numeric().then(|| x))
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap();
                        if color == "red" {
                            if current_game_min_cubes.red < element_value {
                                current_game_min_cubes.red = element_value
                            }
                        } else if color == "blue" {
                            if current_game_min_cubes.blue < element_value {
                                current_game_min_cubes.blue = element_value
                            }
                        } else if color == "green" {
                            if current_game_min_cubes.green < element_value {
                                current_game_min_cubes.green = element_value
                            }
                        }

                        // add_color_macro!(current_game, color, element_num);
                    }
                }
            }

            // only 12 red cubes, 13 green cubes, and 14 blue cubes.
            //
        }

        // println!( "Game minimums: {}  RGB: {} {} {}", &current_game_min_cubes.game_num, current_game_min_cubes.red, current_game_min_cubes.green, current_game_min_cubes.blue);
        let game_power =
            current_game_min_cubes.red * current_game_min_cubes.green * current_game_min_cubes.blue;
        challenge_sum += game_power;
    }
    Ok(challenge_sum as i128)
}

pub fn test_config_d2() -> Config {
    let test_config: Config = Config {
        challenge: 2,
        filename: "./input/test02.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(2, 1, test_config_d2());
    test_expected_computed!(2, 2, test_config_d2());
}
