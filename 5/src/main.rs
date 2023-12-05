#[derive(Debug)]
struct Map {
    start: i64,
    end: i64,
    add: i64,
}

impl Map {
    fn new(line: &str) -> Map {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let start = parts[1].parse().unwrap();
        let end = parts[2].parse::<i64>().unwrap() + start;
        let add = parts[0].parse::<i64>().unwrap() - start;
        Map { start, end, add }
    }

    fn try_convert(&self, seed: i64) -> Option<i64> {
        if seed >= self.start && seed <= self.end {
            return Some(seed + self.add);
        }
        None
    }
}

fn convert(input: i64, maps: &Vec<Map>) -> i64 {
    for map in maps {
        if let Some(output) = map.try_convert(input) {
            return output;
        }
    }
    input
}

fn full_conversion(input: i64, maps: &Vec<&mut Vec<Map>>) -> i64 {
    let soil = convert(input, maps[0]);
    let fertilizer = convert(soil, maps[1]);
    let water = convert(fertilizer, maps[2]);
    let light = convert(water, maps[3]);
    let temperature = convert(light, maps[4]);
    let humidity = convert(temperature, maps[5]);
    convert(humidity, maps[6])
}

fn main() {
    let mut seeds: Vec<i64> = Vec::new();
    let mut seed_to_soil: Vec<Map> = Vec::new();
    let mut soil_to_fertilizer: Vec<Map> = Vec::new();
    let mut fertilizer_to_water: Vec<Map> = Vec::new();
    let mut water_to_light: Vec<Map> = Vec::new();
    let mut light_to_temperature: Vec<Map> = Vec::new();
    let mut temperature_to_humidity: Vec<Map> = Vec::new();
    let mut humidity_to_location: Vec<Map> = Vec::new();
    let mut all_maps = vec![
        &mut seed_to_soil,
        &mut soil_to_fertilizer,
        &mut fertilizer_to_water,
        &mut water_to_light,
        &mut light_to_temperature,
        &mut temperature_to_humidity,
        &mut humidity_to_location,
    ];
    let mut maps_iter = all_maps.iter_mut();

    let input = std::fs::read_to_string("input").unwrap();
    let mut lines = input.lines();
    for mut num in lines.next().unwrap().split_once(":").unwrap().1.split(" ") {
        num = num.trim();
        if num != "" {
            seeds.push(num.parse().unwrap());
        }
    }
    lines.next();
    lines.next();
    let mut current_map = maps_iter.next().unwrap();
    while let Some(line) = lines.next() {
        if line == "" {
            current_map = maps_iter.next().unwrap();
            lines.next();
            continue;
        }
        current_map.push(Map::new(line));
    }
    println!("{}", seeds.iter()
             .map(|seed| full_conversion(*seed, &all_maps))
             .min().unwrap()
    );
}
