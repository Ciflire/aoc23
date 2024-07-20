use std::f64::MAX;

use crate::helper::file_to_str;

pub fn part_one(filename: &str) -> u32 {
    let mut res = 0;

    let file = file_to_str(filename).unwrap();
    let lines = file.split('\n').collect::<Vec<&str>>();

    let mut offset = -1;

    let max_seed = {
        let mut res = 0;
        for line in lines.clone() {
            if line.contains("seeds:") {
                // first line
                continue;
            } else if line.contains(":") {
                // new section starting
                offset += 1;
            } else if line == "" {
                // line is empty
                continue;
            } else {
                let numbers = line
                    .split(" ")
                    .filter_map(|e| e.parse().ok())
                    .collect::<Vec<u64>>();
                let start_first = numbers[0];
                let start_second = numbers[1];
                let length = numbers[2];
                res = if res <= start_first + length {
                    start_first + length
                } else if res <= start_second + length {
                    start_second + length
                } else {
                    res
                };
            }
        }
        res
    };

    let mut map_correspondance = Vec::new();

    for i in 0..max_seed {
        map_correspondance.push([i, i, i, i, i, i, i, i]);
    }

    println!("{:?}", map_correspondance);

    for line in lines {
        if line.contains("seeds:") {
            // first line
            continue;
        } else if line.contains(":") {
            // new section starting
            offset += 1;
        } else if line == "" {
            // line is empty
            continue;
        } else {
            let numbers = line
                .split(" ")
                .filter_map(|e| e.parse().ok())
                .collect::<Vec<u64>>();
            let start_first = numbers[0];
            let start_second = numbers[1];
            let length = numbers[2];
        }
    }

    res
}
