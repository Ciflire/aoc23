use crate::helper::file_to_str;

pub fn part_two(filename: &str) -> Result<u64, String> {
    let file = file_to_str(filename).unwrap();
    let lines = file
        .split('\n')
        .map(|elt| {
            elt.split(" ")
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let seeds = lines[0].clone();
    let mut seed_to_soil = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temperature = Vec::new();
    let mut temperature_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();
    let mut index_pointer = 1;
    let mut map_pointer = 0;
    while index_pointer < lines.len() {
        if lines[index_pointer].len() == 0 {
            index_pointer += 1;
            map_pointer += 1;
        } else {
            match map_pointer {
                2 => seed_to_soil.push(lines[index_pointer].clone()),
                4 => soil_to_fertilizer.push(lines[index_pointer].clone()),
                6 => fertilizer_to_water.push(lines[index_pointer].clone()),
                8 => water_to_light.push(lines[index_pointer].clone()),
                10 => light_to_temperature.push(lines[index_pointer].clone()),
                12 => temperature_to_humidity.push(lines[index_pointer].clone()),
                14 => humidity_to_location.push(lines[index_pointer].clone()),
                _ => return Err("Wrong map pointer".to_string()),
            }
            index_pointer += 1;
        }
    }

    // println!(
    //     "{:?}\n {:?}\n {:?}\n {:?}\n {:?}\n {:?}\n {:?}\n {:?}",
    //     seeds.len(),
    //     seed_to_soil.len(),
    //     soil_to_fertilizer.len(),
    //     fertilizer_to_water.len(),
    //     water_to_light.len(),
    //     light_to_temperature.len(),
    //     temperature_to_humidity.len(),
    //     humidity_to_location.len()
    // );
    let mut min_location = std::u64::MAX;
    for seed in seeds {
        let mut soil = seed;
        for line in seed_to_soil.clone() {
            let seed_start = line[1];
            let soil_start = line[0];
            let range = line[2];
            if (seed >= seed_start) && (seed < seed_start + range) {
                soil = soil_start + seed - seed_start;
            }
        }
        let mut fertilizer = soil;
        for line in soil_to_fertilizer.clone() {
            let soil_start = line[1];
            let fertilizer_start = line[0];
            let range = line[2];
            if (soil >= soil_start) && (soil < soil_start + range) {
                fertilizer = fertilizer_start + soil - soil_start;
            }
        }
        let mut water = fertilizer;
        for line in fertilizer_to_water.clone() {
            let fertilizer_start = line[1];
            let water_start = line[0];
            let range = line[2];
            if (fertilizer >= fertilizer_start) && (fertilizer < fertilizer_start + range) {
                water = water_start + fertilizer - fertilizer_start;
            }
        }
        let mut light = water;
        for line in water_to_light.clone() {
            let water_start = line[1];
            let light_start = line[0];
            let range = line[2];
            if (water >= water_start) && (water < water_start + range) {
                light = light_start + water - water_start;
            }
        }
        let mut temperature = light;
        for line in light_to_temperature.clone() {
            let light_start = line[1];
            let temperature_start = line[0];
            let range = line[2];
            if (light >= light_start) && (light < light_start + range) {
                temperature = temperature_start + light - light_start;
            }
        }
        let mut humidity = temperature;
        for line in temperature_to_humidity.clone() {
            let temperature_start = line[1];
            let humidity_start = line[0];
            let range = line[2];
            if (temperature >= temperature_start) && (temperature < temperature_start + range) {
                humidity = humidity_start + temperature - temperature_start;
            }
        }
        let mut location = humidity;
        for line in humidity_to_location.clone() {
            let humidity_start = line[1];
            let location_start = line[0];
            let range = line[2];
            if (humidity >= humidity_start) && (humidity < humidity_start + range) {
                location = location_start + humidity - humidity_start
            }
        }
        if location <= min_location {
            min_location = location;
        }
    }

    Ok(min_location)
}

pub fn part_one(filename: &str) -> Result<u64, String> {
    let file = file_to_str(filename).unwrap();
    let lines = file
        .split('\n')
        .map(|elt| {
            elt.split(" ")
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let seeds = lines[0].clone();
    let mut seed_to_soil = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temperature = Vec::new();
    let mut temperature_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();
    let mut index_pointer = 1;
    let mut map_pointer = 0;
    while index_pointer < lines.len() {
        if lines[index_pointer].len() == 0 {
            index_pointer += 1;
            map_pointer += 1;
        } else {
            match map_pointer {
                2 => seed_to_soil.push(lines[index_pointer].clone()),
                4 => soil_to_fertilizer.push(lines[index_pointer].clone()),
                6 => fertilizer_to_water.push(lines[index_pointer].clone()),
                8 => water_to_light.push(lines[index_pointer].clone()),
                10 => light_to_temperature.push(lines[index_pointer].clone()),
                12 => temperature_to_humidity.push(lines[index_pointer].clone()),
                14 => humidity_to_location.push(lines[index_pointer].clone()),
                _ => return Err("Wrong map pointer".to_string()),
            }
            index_pointer += 1;
        }
    }

    // println!(
    //     "{:?}\n {:?}\n {:?}\n {:?}\n {:?}\n {:?}\n {:?}\n {:?}",
    //     seeds.len(),
    //     seed_to_soil.len(),
    //     soil_to_fertilizer.len(),
    //     fertilizer_to_water.len(),
    //     water_to_light.len(),
    //     light_to_temperature.len(),
    //     temperature_to_humidity.len(),
    //     humidity_to_location.len()
    // );
    let mut min_location = std::u64::MAX;
    for seed in seeds {
        let mut soil = seed;
        for line in seed_to_soil.clone() {
            let seed_start = line[1];
            let soil_start = line[0];
            let range = line[2];
            if (seed >= seed_start) && (seed < seed_start + range) {
                soil = soil_start + seed - seed_start;
            }
        }
        let mut fertilizer = soil;
        for line in soil_to_fertilizer.clone() {
            let soil_start = line[1];
            let fertilizer_start = line[0];
            let range = line[2];
            if (soil >= soil_start) && (soil < soil_start + range) {
                fertilizer = fertilizer_start + soil - soil_start;
            }
        }
        let mut water = fertilizer;
        for line in fertilizer_to_water.clone() {
            let fertilizer_start = line[1];
            let water_start = line[0];
            let range = line[2];
            if (fertilizer >= fertilizer_start) && (fertilizer < fertilizer_start + range) {
                water = water_start + fertilizer - fertilizer_start;
            }
        }
        let mut light = water;
        for line in water_to_light.clone() {
            let water_start = line[1];
            let light_start = line[0];
            let range = line[2];
            if (water >= water_start) && (water < water_start + range) {
                light = light_start + water - water_start;
            }
        }
        let mut temperature = light;
        for line in light_to_temperature.clone() {
            let light_start = line[1];
            let temperature_start = line[0];
            let range = line[2];
            if (light >= light_start) && (light < light_start + range) {
                temperature = temperature_start + light - light_start;
            }
        }
        let mut humidity = temperature;
        for line in temperature_to_humidity.clone() {
            let temperature_start = line[1];
            let humidity_start = line[0];
            let range = line[2];
            if (temperature >= temperature_start) && (temperature < temperature_start + range) {
                humidity = humidity_start + temperature - temperature_start;
            }
        }
        let mut location = humidity;
        for line in humidity_to_location.clone() {
            let humidity_start = line[1];
            let location_start = line[0];
            let range = line[2];
            if (humidity >= humidity_start) && (humidity < humidity_start + range) {
                location = location_start + humidity - humidity_start
            }
        }
        if location <= min_location {
            min_location = location;
        }
    }

    Ok(min_location)
}
