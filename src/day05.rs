use std::str::FromStr;
use adventofcode_2023::read_input_lines;
use anyhow::Result;
use crate::util::span::Span;

#[derive(Debug, Eq, PartialEq)]
pub struct ConversionRange {
    input:Span,
    output:Span,
}
impl FromStr for ConversionRange {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values:Vec<usize> = s.split_whitespace().filter_map(|n|n.parse().ok()).collect();
        Ok(ConversionRange{
            input:  Span::new(values[1], values[1] + values[2]),
            output: Span::new(values[0], values[0] + values[2]),
        })
    }
}
impl ConversionRange {
    fn convert(&self, value:usize) -> Option<usize> {
        // Converts value within input range into value in output
        // Unmatched values return None
        //      x     y
        //         |------|
        //      ↓     ↓
        //            z
        if self.input.contains_value(value) {
            Some(value - self.input.start + self.output.start)
        } else {
            None
        }
    }

    fn convert_span(&self, value:&Span) -> Option<Span> {
        // Converts ranges within input range into values in output
        // Unmatched range segments return None
        //     |-------------|
        //         |------|
        //     ↓   ↓      ↓  ↓
        //         |======|
        if let Some(span) = value.intersection(&self.input) {
            Some(Span::new(
                span.start - self.input.start + self.output.start,
                span.end   - self.input.start + self.output.start
            ))
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
        // Converts value within multiple input ranges into value in outputs
        // Unmatched values are returned
        //      x     y      z
        //         |------|
        //                |-----|
        //      ↓     ↓      ↓
        //      x     a      b
        let mut converted_value:usize = value;
        for conversion in self.conversions.iter() {
            if let Some(new_value) = conversion.convert(value) {
                converted_value = new_value
            }
        }
        Some(converted_value)
    }

    fn convert_span(&self, value:&Span) -> Option<Vec<Span>> {
        // Converts ranges within multiple input ranges into ranges in outputs
        // Unmatched range segments are returned
        //      |---------------|
        //         |------|
        //                |--|
        //      ↓  ↓      ↓  ↓  ↓
        //      |--|======|~~|--|
        let mut remaining_values:Vec<Span> = vec![value.clone()];
        let mut converted_values:Vec<Span> = Vec::new();
        for conversion in self.conversions.iter() {
            if let Some(new_value) = conversion.convert_span(value) {
                // remove the converted range segment from the original range
                remaining_values = remaining_values
                    .iter()
                    .flat_map(|v| v.subtraction(&conversion.input).unwrap_or(vec![]))
                    .collect();
                converted_values.push(new_value);
            }
        }
        // include remaining unconverted range segments in output
        converted_values.extend(remaining_values);
        Some(converted_values)
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
    // for each seed range, convert it into a list of new contiguous ranges
    // pass each set of ranges to the next conversion table to get a new list
    // sort final ranges to determine the start value of the lowest range
    let mut locations:Vec<Span> = vec![];
    for seed_span in seeds.chunks(2) {
        let seed = vec![Span::new(seed_span[0], seed_span[0] + seed_span[1])];
        locations.extend(
            conversion_tables
                .iter()
                .fold(seed,
                      |seeds, table| {
                          seeds
                              .iter()
                              .flat_map(|seed| table.convert_span(seed).unwrap())
                              .collect()
                      })
        );
    }
    locations.sort_by(|a, b| a.start.cmp(&b.start));

    Some(locations.first().unwrap().start)
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
                    ConversionRange{input: Span::new(18, 25), output: Span::new(88, 95)},
                    ConversionRange{input: Span::new(25, 95), output: Span::new(18, 88)},
                ]
            });
        }
    }

    #[test]
    fn text_convert() {
        let seed_to_soil = ConversionTable {
            id:"seed-to-soil".to_string(),
            conversions: vec![
                ConversionRange{input: Span::new(98, 100), output: Span::new(50, 52)},
                ConversionRange{input: Span::new(50, 98), output: Span::new(52, 100)},
            ]
        };
        assert_eq!(seed_to_soil.convert(98), Some(50));
        assert_eq!(seed_to_soil.convert(53), Some(55));

        let fertilizer_to_water = ConversionTable {
            id:"fertilizer-to-water".to_string(),
            conversions: vec![
                ConversionRange{input: Span::new(53, 61), output: Span::new(49, 57)},
                ConversionRange{input: Span::new(11, 53), output: Span::new(0, 42)},
                ConversionRange{input: Span::new(0, 7),   output: Span::new(42, 49)},
                ConversionRange{input: Span::new(7, 11),  output: Span::new(57, 61)},
            ]
        };
        assert_eq!(fertilizer_to_water.convert(53), Some(49));
    }

    #[test]
    fn test_convert_span() {
        if let Ok((_, conversion_tables)) = prepare("day05-example.txt") {
            let humidity: Vec<Span> = vec![Span::new(46, 57), Span::new(78, 81)];
            let humidity_to_location = &conversion_tables[6];
            let mut locations:Vec<Span> = humidity
                .iter()
                .flat_map(|seed| humidity_to_location.convert_span(seed).unwrap())
                .collect();
            locations.sort_by(|a, b| a.start.cmp(&b.start));
            assert_eq!(locations,  vec![Span::new(46, 56), Span::new(60, 61), Span::new(82, 85)]);
        }
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

    #[test]
    fn test_part_1_puzzle() {
        if let Ok(input) = prepare("day05.txt") {
            assert_eq!(part_1(&input), Some(462648396))
        }
    }

    #[test]
    fn test_part_2_puzzle() {
        if let Ok(input) = prepare("day05.txt") {
            assert_eq!(part_2(&input), Some(2520479))
        }
    }
}
