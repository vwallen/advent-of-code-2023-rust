use std::collections::HashMap;
use adventofcode_2023::read_input_lines;
use anyhow::Result;

pub fn prepare(file_name: &str) -> Result<(Vec<usize>, HashMap<String, Vec<(usize, usize, usize)>>)> {
    let input = read_input_lines(file_name);
    let mut seeds:Vec<usize> = vec![];
    let mut converters:HashMap<String, Vec<(usize, usize, usize)>> = HashMap::new();
    let mut conversions:Vec<Vec<(usize, usize, usize)>> = vec![];

    for line in input.iter() {
        // skip empty lines
        if line.len() == 0 { continue; }

        // grab the list of seeds from the header
        if line.starts_with("seeds:") {
            let (_, seed_string) = line.split_once(":").unwrap();
            seeds = seed_string
                .trim()
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            continue;
        }

        // grab the conversion table headers to use as keys
        if line.contains(":") {
            // can skip parsing the header since we insert it manually later ::sigh::
            // let (header, _) = line.split_once(" ").unwrap();
            // // have to convert the header to a String because the &str
            // // still belongs to line and I can't return it as part of
            // // a HashMap
            // converters.insert(String::from(header), vec![]);

            // start a new list of conversion triptychs
            conversions.push(vec![]);
            continue;
        }

        // parse the conversion range triptychs and add to a vec to drop into
        // the HashMap later
        let values:Vec<usize> = line.trim().split_whitespace().filter_map(|n|n.parse().ok()).collect();
        let current = conversions.last_mut().unwrap();
        current.push((values[0], values[1], values[2]));
    }

    // manual get things into the hashmap because the borrow checker won't let a Vec
    // exist in this loop and in a HashMap at the same time
    converters.insert("seed-to-soil".to_string(), conversions[0].clone());
    converters.insert("soil-to-fertilizer".to_string(), conversions[1].clone());
    converters.insert("fertilizer-to-water".to_string(), conversions[2].clone());
    converters.insert("water-to-light".to_string(), conversions[3].clone());
    converters.insert("light-to-temperature".to_string(), conversions[4].clone());
    converters.insert("temperature-to-humidity".to_string(), conversions[5].clone());
    converters.insert("humidity-to-location".to_string(), conversions[6].clone());

    Ok((seeds, converters))
}

fn convert(conversion:(usize, usize, usize), input:usize) -> Option<usize> {
    let in_range = conversion.1..(conversion.1 + conversion.2);
    let ot_range = conversion.0..(conversion.0 + conversion.2);
    if in_range.contains(&input) {
        Some(input - in_range.start + ot_range.start)
    } else {
        None
    }
}

fn convert_chain(chain:&Vec<(usize, usize, usize)>, input:usize) -> usize {
    for conversion in chain.iter() {
        if let Some(output) = convert(*conversion, input) {
            return output
        }
    }
    input
}

pub fn part_1((seeds, converters): &(Vec<usize>, HashMap<String, Vec<(usize, usize, usize)>>)) -> Option<usize> {
    // TODO - waves hand at all this
    let seed_2_soil = converters.get("seed-to-soil").unwrap();
    let soil_2_fertilizer = converters.get("soil-to-fertilizer").unwrap();
    let fertilizer_2_water = converters.get("fertilizer-to-water").unwrap();
    let water_2_light = converters.get("water-to-light").unwrap();
    let light_2_temperature = converters.get("light-to-temperature").unwrap();
    let temperature_2_humidity = converters.get("temperature-to-humidity").unwrap();
    let humidity_2_location = converters.get("humidity-to-location").unwrap();

    let mut locations:Vec<usize> = vec![];
    for seed in seeds {
        // Yo, dawg
        locations.push(
            convert_chain(humidity_2_location,
              convert_chain(temperature_2_humidity,
                convert_chain(light_2_temperature,
                  convert_chain(water_2_light,
                    convert_chain(fertilizer_2_water,
                      convert_chain(soil_2_fertilizer,
                        convert_chain(seed_2_soil,
                                      *seed))))))))
    }

    Some(*locations.iter().min().unwrap())
}

pub fn part_2((seeds, converters): &(Vec<usize>, HashMap<String, Vec<(usize, usize, usize)>>)) -> Option<usize> {
    let seed_2_soil = converters.get("seed-to-soil").unwrap();
    let soil_2_fertilizer = converters.get("soil-to-fertilizer").unwrap();
    let fertilizer_2_water = converters.get("fertilizer-to-water").unwrap();
    let water_2_light = converters.get("water-to-light").unwrap();
    let light_2_temperature = converters.get("light-to-temperature").unwrap();
    let temperature_2_humidity = converters.get("temperature-to-humidity").unwrap();
    let humidity_2_location = converters.get("humidity-to-location").unwrap();

    let mut locations:Vec<usize> = vec![];
    for seed_span in seeds.chunks(2) {
        // There's no force like brute force
        for seed in seed_span[0]..(seed_span[0] + seed_span[1]) {
            // YO, DAWG!
            locations.push(
                convert_chain(humidity_2_location,
                  convert_chain(temperature_2_humidity,
                    convert_chain(light_2_temperature,
                      convert_chain(water_2_light,
                        convert_chain(fertilizer_2_water,
                          convert_chain(soil_2_fertilizer,
                            convert_chain(seed_2_soil,
                                          seed))))))))
        }
    }

    Some(*locations.iter().min().unwrap())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        if let Ok((seeds, converters)) = prepare("day05-example.txt") {
            assert_eq!(seeds, vec![79, 14, 55, 13]);
            assert_eq!(converters.get("water-to-light").unwrap(), &vec![(88, 18, 7), (18, 25, 70)])
        }
    }

    #[test]
    fn text_convert() {
        // seed-to-soil map:
        // 50 98 2
        // 52 50 48
        // seed number 98 corresponds to soil number 50
        // seed number 53 corresponds to soil number 55
        assert_eq!(convert((50, 98, 2), 98), Some(50));
        assert_eq!(convert( (52, 50, 48), 53), Some(55));
        assert_eq!(convert_chain(&vec![(50, 98, 2), (52, 50, 48)], 53), 55);

        // Welp, turns out the ranges can overlap so order and early exit are important
        // fertilizer-to-water map:
        // 49 53 8
        // 0 11 42
        // 42 0 7
        // 57 7 4
        assert_eq!(convert_chain( &vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)], 53), 49);

    }

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day05-example.txt") {
            assert_eq!(part_1(&input), Some(35))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day05-example.txt") {
            assert_eq!(part_2(&input), Some(46))
        }
    }
}