use core::panic;

use crate::helper::file_to_str;

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
