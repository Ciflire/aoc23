use crate::helper::file_to_str;

pub fn part_two(path: &str) -> u32 {
    let mut res = 0;
    let text = file_to_str(path).unwrap().to_owned();
    let games = text.split('\n');
    for game in games {
        if game == "" {
            break;
        }
        let shows = game.split(":").skip(1).next().unwrap().split(";");
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for show in shows {
            for elt_in_show in show.split(",") {
                if elt_in_show != "" {
                    let number: u32 = elt_in_show
                        .split(" ")
                        .skip(1)
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();
                    let color = elt_in_show.split(" ").skip(2).next().unwrap();
                    match color {
                        "red" => {
                            if number > max_red {
                                max_red = number;
                            }
                        }
                        "green" => {
                            if number > max_green {
                                max_green = number;
                            }
                        }
                        "blue" => {
                            if number > max_blue {
                                max_blue = number;
                            }
                        }
                        _ => panic!("wrong color: {color}"),
                    }
                }
            }
        }
        res += max_blue * max_green * max_red;
    }
    res
}

pub fn part_one(path: &str) -> u32 {
    let mut res = 0;
    let text = file_to_str(path).unwrap().to_owned();
    let games = text.split('\n');
    let mut game_number = 1;
    'game: for game in games {
        if game == "" {
            break;
        }
        let shows = game.split(":").skip(1).next().unwrap().split(";");
        for show in shows {
            for elt_in_show in show.split(",") {
                if elt_in_show != "" {
                    let number: u32 = elt_in_show
                        .split(" ")
                        .skip(1)
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();
                    let color = elt_in_show.split(" ").skip(2).next().unwrap();
                    match color {
                        "red" => {
                            if number > 12 {
                                res += game_number;
                                game_number += 1;
                                continue 'game;
                            }
                        }
                        "green" => {
                            if number > 13 {
                                res += game_number;
                                game_number += 1;
                                continue 'game;
                            }
                        }
                        "blue" => {
                            if number > 14 {
                                res += game_number;
                                game_number += 1;
                                continue 'game;
                            }
                        }
                        _ => panic!("wrong color: {color}"),
                    }
                }
            }
        }
        game_number += 1
    }
    res
}
