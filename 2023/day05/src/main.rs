use std::io;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct ConversionMap {
    pub dest_start: u64,
    pub src_start: u64,
    pub length: u64,
}

#[derive(Clone)]
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

fn traverse_back0(input: u64, conversions: &[ConversionMap]) -> u64 {
    let m = conversions
        .iter()
        .find(|c| c.dest_start <= input && input < c.dest_start + c.length);
    match m {
        Some(c) => input - c.dest_start + c.src_start,
        None => input,
    }
}

fn merge_maps0(from: &[ConversionMap], to: &[ConversionMap]) -> Vec<ConversionMap> {
    let mut from = from.to_owned().clone();
    let mut to = to.to_owned().clone();
    from.sort_unstable_by_key(|c| c.dest_start);
    to.sort_unstable_by_key(|c| c.src_start);

    let mid_points = from
        .iter()
        .flat_map(|c| [c.dest_start, c.dest_start + c.length])
        .chain(
            to.iter()
                .flat_map(|c| [c.src_start, c.src_start + c.length]),
        )
        .sorted_unstable()
        .unique()
        .collect_vec();

    mid_points
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let length = b - a;
            let src_start = traverse_back0(*a, &from);
            let dest_start = traverse0(*a, &to);
            ConversionMap {
                dest_start,
                src_start,
                length,
            }
        })
        .collect_vec()
}

fn merge_maps(maps: &Maps) -> Vec<ConversionMap> {
    let x = merge_maps0(&maps.seed_to_soil, &maps.soil_to_fertilizer);
    let x = merge_maps0(&x, &maps.fertilizer_to_water);
    let x = merge_maps0(&x, &maps.water_to_light);
    let x = merge_maps0(&x, &maps.light_to_temperature);
    let x = merge_maps0(&x, &maps.temperature_to_humidity);
    merge_maps0(&x, &maps.humidity_to_location)
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

    let merged_maps = merge_maps(&maps);

    let r1 = seeds
        .iter()
        .map(|s| traverse0(*s, &merged_maps))
        .min()
        .unwrap();
    println!("{:?}", r1);

    let seeds2 = seeds
        .into_iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let a = chunk.next().unwrap();
            let b = chunk.next().unwrap();
            (a, b)
        })
        .collect_vec();

    let r2 = merged_maps
        .into_iter()
        .filter(|c| {
            seeds2.iter().any(|(seed_start, seed_length)| {
                c.src_start <= *seed_start && *seed_start < c.src_start + c.length
                    || *seed_start <= c.src_start && c.src_start < *seed_start + *seed_length
            })
        })
        .map(|c| c.dest_start)
        .min()
        .unwrap();
    println!("{:?}", r2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_maps() {
        let c1 = vec![
            ConversionMap {
                dest_start: 50,
                src_start: 98,
                length: 2,
            },
            ConversionMap {
                dest_start: 52,
                src_start: 50,
                length: 48,
            },
        ];
        let c2 = vec![
            ConversionMap {
                dest_start: 0,
                src_start: 15,
                length: 37,
            },
            ConversionMap {
                dest_start: 37,
                src_start: 52,
                length: 2,
            },
            ConversionMap {
                dest_start: 39,
                src_start: 0,
                length: 15,
            },
        ];

        let c = merge_maps0(&c1, &c2);

        (0..=100).for_each(|i| {
            let expected = traverse0(i, &c1);
            let expected = traverse0(expected, &c2);

            let result = traverse0(i, &c);

            assert_eq!(expected, result);
        });
    }
}
