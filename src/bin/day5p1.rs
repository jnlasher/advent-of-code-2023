use std::{fs, u64::MAX};

pub fn main() {
    let input = fs::read_to_string("./inputs/day5.txt").expect("Failed to read input");
    let minimum_location = find_minimum_location(&input);
    println!("Minimum location: {}", minimum_location);
}

#[derive(Debug)]
struct MapEntry {
    destination: u64,
    source: u64,
    range: u64,
}

fn read_seeds<'a, I>(mut lines: I) -> (Vec<u64>, I)
where
    I: Iterator<Item = &'a str>,
{
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    // Consume the empty line after as well to make my life easier
    lines.next();

    (seeds, lines)
}

fn read_map<'a, I>(mut lines: I) -> (Vec<MapEntry>, I)
where
    I: Iterator<Item = &'a str>,
{
    let _name = lines.next().unwrap().trim_end_matches(':');
    let mut map_values: Vec<MapEntry> = Vec::new();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let entries: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        map_values.push(MapEntry {
            destination: entries[0],
            source: entries[1],
            range: entries[2],
        });
    }

    (map_values, lines)
}

// Not sure if this is idiomatic Rust, but I liked the idea of passing an iterator back to
// the same function so that I can just keep reading from the same thing until its used up and
// all the map data is stored in variables for me to reference later ¯\_(ツ)_/¯
fn find_minimum_location(input: &str) -> u64 {
    // Remove the blank lines
    let lines = input.lines();

    // Read in the seeds
    let (seeds, lines) = read_seeds(lines);

    // Read in the maps
    // A smarter person might read this in to a Vec<Vec> and make it more readable
    // to create them and then loop through them. Maybe I'll do that later
    let (seed_soil_map, lines) = read_map(lines);
    let (soil_fertilizer_map, lines) = read_map(lines);
    let (fertilizer_water_map, lines) = read_map(lines);
    let (water_light_map, lines) = read_map(lines);
    let (light_temperature_map, lines) = read_map(lines);
    let (temperature_humidity_map, lines) = read_map(lines);
    let (humidity_location_map, _) = read_map(lines);

    let mut min_location: u64 = MAX;
    for seed in seeds {
        let mut plant = seed;
        plant = grow_seed(plant, &seed_soil_map);
        plant = grow_seed(plant, &soil_fertilizer_map);
        plant = grow_seed(plant, &fertilizer_water_map);
        plant = grow_seed(plant, &water_light_map);
        plant = grow_seed(plant, &light_temperature_map);
        plant = grow_seed(plant, &temperature_humidity_map);
        plant = grow_seed(plant, &humidity_location_map);

        if plant < min_location {
            min_location = plant;
        }
    }

    min_location
}

fn grow_seed(seed: u64, map: &Vec<MapEntry>) -> u64 {
    let mut sprout = seed;
    for entry in map {
        if seed >= entry.source && seed < entry.source + entry.range {
            sprout = seed - entry.source + entry.destination;
            break;
        }
    }

    sprout
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_parse_map() {
        assert_eq!(find_minimum_location(INPUT), 35);
    }
}
