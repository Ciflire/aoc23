mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod helper;

fn main() {
    println!("Day 1 Part 1: {}", day1::part_one("./src/day1.txt"));
    println!("Day 1 Part 2: {}", day1::part_two("./src/day1.txt"));
    println!("Day 2 Part 1: {}", day2::part_one("./src/day2.txt"));
    println!("Day 2 Part 2: {}", day2::part_two("./src/day2.txt"));
    println!("Day 3 Part 1: {}", day3::part_one("./src/day3.txt"));
    println!("Day 3 Part 2: {}", day3::part_two("./src/day3.txt"));
    println!("Day 4 Part 1: {}", day4::part_one("./src/day4.txt"));
    println!("Day 4 Part 2: {}", day4::part_two("./src/day4.txt"));
    println!(
        "Day 5 Part 1: {}",
        day5::part_one("./src/day5.txt").unwrap()
    );
    println!(
        "Day 5 Part 2: {}",
        day5::part_two("./src/day5.txt").unwrap()
    )
}
