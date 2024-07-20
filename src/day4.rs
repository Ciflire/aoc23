use crate::helper::file_to_str;

pub fn part_two(filename: &str) -> u32 {
    let file = file_to_str(filename).unwrap();

    let games = file
        .lines()
        .map(|l| l.split(&[':', '|']).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut scratchcards = [1; 203];

    let mut i = 0;
    for game in games {
        let winning_numbers = game.clone()[1]
            .split(" ")
            .filter_map(|e| e.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        let numbers = game[2]
            .split(" ")
            .filter_map(|e| e.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let mut qte_win = 0;

        for number in numbers {
            winning_numbers
                .clone()
                .into_iter()
                .for_each(|winning_number| {
                    if number == winning_number {
                        qte_win += 1
                    }
                });
        }
        for j in 1..=qte_win {
            scratchcards[i + j] += scratchcards[i]
        }
        i += 1;
    }

    scratchcards.iter().sum()
}

pub fn part_one(filename: &str) -> u32 {
    let mut res: u32 = 0;
    let file = file_to_str(filename).unwrap();

    let games = file
        .lines()
        .map(|l| l.split(&[':', '|']).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for game in games {
        let winning_numbers = game.clone()[1]
            .split(" ")
            .filter_map(|e| e.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        let numbers = game[2]
            .split(" ")
            .filter_map(|e| e.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        let mut part_res = 0;
        for number in numbers {
            winning_numbers
                .clone()
                .into_iter()
                .for_each(|winning_number| {
                    if number == winning_number {
                        if part_res == 0 {
                            part_res = 1;
                        } else {
                            part_res *= 2
                        }
                    }
                });
        }
        res += part_res
    }

    res
}
