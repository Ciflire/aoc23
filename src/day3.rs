use crate::helper::file_to_str;

pub fn part_two(file: &str) -> u64 {
    let mut res: u64 = 0;
    let text = file_to_str(file).unwrap();
    let mut table: Vec<Vec<char>> = Vec::new();
    table.extend(text.split("\n").map(|line| line.chars().collect()));
    for i in 0..(table.len() - 1) {
        for j in 0..(table[0].len()) {
            match table[i][j] {
                '*' => {
                    res += helper_part_two(&mut table, &i, &j);
                    table[i][j] = '.';
                }
                _ => (),
            }
        }
    }
    res
}

fn helper_part_two(table: &mut Vec<Vec<char>>, i: &usize, j: &usize) -> u64 {
    let mut res_product: u64 = 0;
    let mut vec_numbers = Vec::new();

    let position = [
        (*i - 1, *j),
        (*i - 1, *j - 1),
        (*i, *j - 1),
        (*i + 1, *j - 1),
        (*i + 1, *j),
        (*i + 1, *j + 1),
        (*i, *j + 1),
        (*i - 1, *j + 1),
    ];

    let n = table.clone().len();
    let m = table[0].clone().len();

    let valid_position = position
        .into_iter()
        .filter(|(k, l)| k >= &0 && l >= &0 && k < &n && l < &n);

    for (k, l) in valid_position {
        let mut i: i32 = 0;
        let mut part_res = 0;
        while l as i32 + i >= 0 && table[k][(l as i32 + i) as usize].is_digit(10) {
            part_res += table[k][(l as i32 + i) as usize].to_digit(10).unwrap()
                * (10 as u32).pow(-i as u32) as u32;
            // table[k][(l as i32 + i) as usize] = '.';
            i -= 1;
        }
        i = if part_res != 0 { 1 as i32 } else { 0 as i32 };
        while l as i32 + i < m as i32 && table[k][(l as i32 + i) as usize].is_digit(10) {
            part_res = table[k][(l as i32 + i) as usize].to_digit(10).unwrap() + part_res * 10;
            // table[k][(l as i32 + i) as usize] = '.';
            i += 1
        }

        if !vec_numbers.contains(&part_res) && part_res != 0 {
            vec_numbers.push(part_res);
        }
    }

    if vec_numbers.len() == 2 {
        res_product = (vec_numbers[0] * vec_numbers[1]) as u64
    };

    res_product
}

pub fn part_one(file: &str) -> u64 {
    let mut res: u64 = 0;
    let text = file_to_str(file).unwrap();
    let mut table: Vec<Vec<char>> = Vec::new();
    table.extend(text.split("\n").map(|line| line.chars().collect()));

    for i in 0..(table.len() - 1) {
        for j in 0..(table[0].len()) {
            match table[i][j] {
                '$' | '@' | '*' | '+' | '&' | '%' | '/' | '=' | '-' | '#' => {
                    res += helper_part_one(&mut table, &i, &j);
                    table[i][j] = '.';
                }
                _ => (),
            }
        }
    }

    res
}

fn helper_part_one(table: &mut Vec<Vec<char>>, i: &usize, j: &usize) -> u64 {
    let mut res: u64 = 0;

    let position = [
        (*i - 1, *j),
        (*i - 1, *j - 1),
        (*i, *j - 1),
        (*i + 1, *j - 1),
        (*i + 1, *j),
        (*i + 1, *j + 1),
        (*i, *j + 1),
        (*i - 1, *j + 1),
    ];

    let n = table.clone().len();
    let m = table[0].clone().len();

    let valid_position = position
        .into_iter()
        .filter(|(k, l)| k >= &0 && l >= &0 && k < &n && l < &n);

    for (k, l) in valid_position {
        let mut i: i32 = 0;
        let mut part_res = 0;
        while l as i32 + i >= 0 && table[k][(l as i32 + i) as usize].is_digit(10) {
            part_res += table[k][(l as i32 + i) as usize].to_digit(10).unwrap()
                * (10 as u32).pow(-i as u32) as u32;
            table[k][(l as i32 + i) as usize] = '.';
            i -= 1;
        }
        i = if part_res != 0 { 1 as i32 } else { 0 as i32 };
        while l as i32 + i < m as i32 && table[k][(l as i32 + i) as usize].is_digit(10) {
            part_res = table[k][(l as i32 + i) as usize].to_digit(10).unwrap() + part_res * 10;
            table[k][(l as i32 + i) as usize] = '.';
            i += 1
        }
        res += part_res as u64;
    }

    res
}
