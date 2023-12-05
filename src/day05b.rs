use std::str::FromStr;
use adventofcode_2023::read_input_lines;
use anyhow::Result;

#[derive(Debug, Eq, PartialEq)]
pub struct ConversionRange {
    input:(usize, usize),
    output:(usize, usize),
}
impl FromStr for ConversionRange {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Sample: "785532007 269485885 88937774"
        let values:Vec<usize> = s.split_whitespace().filter_map(|n|n.parse().ok()).collect();
        Ok(ConversionRange{
            input:  (values[1], values[1] + values[2]),
            output: (values[0], values[0] + values[2]),
        })
    }
}
impl ConversionRange {
    fn convert(&self, value:usize) -> Option<usize> {
        if self.input.0 <= value && value < self.input.1 {
            Some(value - self.input.0 + self.output.0)
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ConversionTable {
    id: String,
    conversions:Vec<ConversionRange>,
}
impl ConversionTable {
    fn convert(&self, value:usize) -> Option<usize> {
        for conversion in self.conversions.iter() {
            if let Some(new_value) = conversion.convert(value) {
                // early exit because first hit applies
                return Some(new_value)
            }
        }
        Some(value)
    }
}

pub fn prepare(file_name: &str) -> Result<(Vec<usize>, Vec<ConversionTable>)> {
    let input = read_input_lines(file_name);

    let mut seeds:Vec<usize> = vec![];
    let mut conversion_tables:Vec<ConversionTable> = vec![];
    let mut current:ConversionTable = ConversionTable{id:"".to_string(), conversions:vec![]};

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
        // this starts a new conversion table
        if line.contains(":") {
            // save the current conversion table and start a new one
            // don't bother with the empty one we had to initialize before
            if current.id != "" {
                conversion_tables.push(current);
            }
            let (header, _) = line.split_once(" ").unwrap();
            current = ConversionTable{
                id: header.to_string(),
                conversions: vec![],
            };
            continue;
        }

        // parse line values for the conversion tables
        let conversion_range:ConversionRange = line.parse().unwrap();
        current.conversions.push(conversion_range);
    }
    // catch the last conversion table
    // to close out the loop
    conversion_tables.push(current);

    // using Vec for seeds and conversion tables
    // because the tables are applied in the order
    // they appear in the input file
    Ok((seeds, conversion_tables))
}

pub fn part_1((seeds, conversion_tables):&(Vec<usize>, Vec<ConversionTable>)) -> Option<usize> {
    let mut locations:Vec<usize> = vec![];
    for seed in seeds.iter() {
        locations.push(
            conversion_tables
                .iter()
                .fold(*seed, |a, x| {
                    x.convert(a).unwrap()
                }));
    }
    Some(*locations.iter().min().unwrap())
}

pub fn part_2((seeds, conversion_tables):&(Vec<usize>, Vec<ConversionTable>)) -> Option<usize> {
    let mut locations:Vec<usize> = vec![];
    for seed_span in seeds.chunks(2) {
        for seed in seed_span[0]..(seed_span[0] + seed_span[1]) {
            locations.push(
                conversion_tables
                    .iter()
                    .fold(seed, |a, x| {
                        x.convert(a).unwrap()
                    }));
        }
    }
    Some(*locations.iter().min().unwrap())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        if let Ok((seeds, conversion_tables)) = prepare("day05-example.txt") {
            assert_eq!(seeds, vec![79, 14, 55, 13]);
            assert_eq!(conversion_tables[3], ConversionTable{
                id:"water-to-light".to_string(),
                conversions: vec![
                    ConversionRange{input: (18, 25), output: (88, 95)},
                    ConversionRange{input: (25, 95), output: (18, 88)},
                ]
            });
        }
    }

    #[test]
    fn text_convert() {
        let seed_to_soil = ConversionTable {
            id:"seed-to-soil".to_string(),
            conversions: vec![
                ConversionRange{input: (98, 100), output: (50, 52)},
                ConversionRange{input: (50, 98), output: (52, 100)},
            ]
        };
        assert_eq!(seed_to_soil.convert(98), Some(50));
        assert_eq!(seed_to_soil.convert(53), Some(55));

        let fertilizer_to_water = ConversionTable {
            id:"fertilizer-to-water".to_string(),
            conversions: vec![
                ConversionRange{input: (53, 61), output: (49, 57)},
                ConversionRange{input: (11, 53), output: (0, 42)},
                ConversionRange{input: (0, 7),   output: (42, 49)},
                ConversionRange{input: (7, 11),  output: (57, 61)},
            ]
        };
        assert_eq!(fertilizer_to_water.convert(53), Some(49));
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