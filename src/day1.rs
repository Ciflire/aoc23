use core::panic;
use std::char;

use crate::helper::file_to_str;
use fancy_regex::Regex;

pub fn part_two(file: &str) -> u32 {
    let file = match file_to_str(file) {
        Ok(file_string) => file_string,
        Err(e) => panic!("file could not be read {e:?}"),
    };

    let lines = file.lines();

    let mut res = 0;

    let regex_find_all_number =
        Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();

    for line in lines {
        if line.is_empty() {
            break;
        }
        let mut all_found_numbers = Regex::captures_iter(&regex_find_all_number, line);
        let first_digit: u32;
        let last_digit: u32;
        first_digit = {
            let first_match = all_found_numbers
                .next()
                .unwrap()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
            if first_match.len() == 1 {
                first_match.chars().next().unwrap().to_digit(10).unwrap()
            } else {
                match first_match {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => panic!(),
                }
            }
        };
        last_digit = 'last: {
            let Some(last_match) = all_found_numbers.last() else {
                break 'last 0;
            };
            let last_match = last_match.unwrap().get(1).unwrap().as_str();
            if last_match.len() == 1 {
                last_match.chars().collect::<Vec<char>>()[0]
                    .to_digit(10)
                    .unwrap()
            } else {
                match last_match {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => unreachable!(),
                }
            }
        };
        if last_digit != 0 {
            res += first_digit * 10 + last_digit
        } else {
            res += first_digit * 11
        }
    }
    res
}

pub fn part_one(file: &str) -> u32 {
    let file = match file_to_str(file) {
        Ok(file_string) => file_string,
        Err(e) => panic!("file could not be read {e:?}"),
    };

    let lines = file.lines();

    let mut res = 0;

    for line in lines {
        let mut line_res = 0;
        let elts = line.chars();
        for character in elts {
            if line_res == 0 && character.is_digit(10) {
                line_res += character.to_digit(10).unwrap()
            } else if character.is_digit(10) && line_res < 10 {
                line_res = line_res * 10 + character.to_digit(10).unwrap();
            } else if character.is_digit(10) {
                line_res = line_res - line_res % 10 + character.to_digit(10).unwrap()
            }
        }
        if line_res < 10 {
            line_res = line_res * 11;
        }
        res += line_res;
    }

    res
}
