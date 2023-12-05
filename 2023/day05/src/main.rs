use std::io;

#[derive(Debug)]
struct ConversionMap {
    pub dest_start: u64,
    pub src_start: u64,
    pub length: u64,
}

struct Maps {
    pub seed_to_soil: Vec<ConversionMap>,
    pub soil_to_fertilizer: Vec<ConversionMap>,
    pub fertilizer_to_water: Vec<ConversionMap>,
    pub water_to_light: Vec<ConversionMap>,
    pub light_to_temperature: Vec<ConversionMap>,
    pub temperature_to_humidity: Vec<ConversionMap>,
    pub humidity_to_location: Vec<ConversionMap>,
}

fn parse_conversion_maps(lines: &mut impl Iterator<Item = String>) -> Vec<ConversionMap> {
    let mut maps = Vec::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(' ');
        let dest_start = parts.next().unwrap().parse::<u64>().unwrap();
        let src_start = parts.next().unwrap().parse::<u64>().unwrap();
        let length = parts.next().unwrap().parse::<u64>().unwrap();
        maps.push(ConversionMap {
            dest_start,
            src_start,
            length,
        });
    }
    maps
}

fn parse(lines: &mut impl Iterator<Item = String>) -> Maps {
    assert_eq!(lines.next().unwrap(), "seed-to-soil map:");
    let seed_to_soil = parse_conversion_maps(lines);
    // println!("1 {:?}", seed_to_soil);

    assert_eq!(lines.next().unwrap(), "soil-to-fertilizer map:");
    let soil_to_fertilizer = parse_conversion_maps(lines);
    // println!("2 {:?}", soil_to_fertilizer);

    assert_eq!(lines.next().unwrap(), "fertilizer-to-water map:");
    let fertilizer_to_water = parse_conversion_maps(lines);
    // println!("3 {:?}", fertilizer_to_water);

    assert_eq!(lines.next().unwrap(), "water-to-light map:");
    let water_to_light = parse_conversion_maps(lines);
    // println!("4 {:?}", water_to_light);

    assert_eq!(lines.next().unwrap(), "light-to-temperature map:");
    let light_to_temperature = parse_conversion_maps(lines);
    // println!("5 {:?}", light_to_temperature);

    assert_eq!(lines.next().unwrap(), "temperature-to-humidity map:");
    let temperature_to_humidity = parse_conversion_maps(lines);
    // println!("6 {:?}", temperature_to_humidity);

    assert_eq!(lines.next().unwrap(), "humidity-to-location map:");
    let humidity_to_location = parse_conversion_maps(lines);
    // println!("7 {:?}", humidity_to_location);
    Maps {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn traverse0(input: u64, conversions: &[ConversionMap]) -> u64 {
    let m = conversions
        .iter()
        .find(|c| c.src_start <= input && input < c.src_start + c.length);
    match m {
        Some(c) => input - c.src_start + c.dest_start,
        None => input,
    }
}

fn traverse(seed: u64, maps: &Maps) -> u64 {
    let x = traverse0(seed, &maps.seed_to_soil);
    let x = traverse0(x, &maps.soil_to_fertilizer);
    let x = traverse0(x, &maps.fertilizer_to_water);
    let x = traverse0(x, &maps.water_to_light);
    let x = traverse0(x, &maps.light_to_temperature);
    let x = traverse0(x, &maps.temperature_to_humidity);
    traverse0(x, &maps.humidity_to_location)
}

fn main() {
    let mut lines = io::stdin().lines().map(|l| l.unwrap());
    let seeds = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|t| t.parse::<u64>().ok())
        .collect::<Vec<_>>();
    lines.next();
    let maps = parse(&mut lines);
    let r1 = seeds.iter().map(|s| traverse(*s, &maps)).min().unwrap();
    println!("{:?}", r1);
}
