use std::collections::HashMap;
use crate::read_input_lines;
use anyhow::Result;

#[derive(Debug)]
pub struct Point {
    x:isize,
    y:isize,
}

#[derive(Debug)]
pub struct Span {
    xa:isize,
    ya:isize,
    xb:isize,
    yb:isize,
}

#[derive(Debug)]
pub struct Part {
    number: usize,
    span: Span,
}
impl Part {
    fn from_buffer(digit_buffer: &Vec<char>, x: isize, y: isize) -> Part {
        let part_digits: String = digit_buffer.iter().collect();
        Part {
            number: part_digits.parse().unwrap(),
            span: Span {
                xa: x - (digit_buffer.len() as isize),
                ya: y - 1,
                xb: x + 1,
                yb: y + 1,
            }
        }
    }

    fn adjacent_to(&self, p:&Point) -> bool {
        p.x >= self.span.xa &&
        p.x <= self.span.xb &&
        p.y >= self.span.ya &&
        p.y <= self.span.yb
    }
}

pub fn prepare(file_name: &str) -> Result<(Vec<Part>, HashMap<char, Vec<Point>>)> {
    let input = read_input_lines(file_name);
    let mut digit_buffer:Vec<char> = Vec::new();
    let mut part_numbers:Vec<Part> = Vec::new();
    let mut symbol_map:HashMap<char, Vec<Point>> = HashMap::new();

    for (iy, line) in input.iter().enumerate() {
        let mut buffer_end:isize = 0;
        let y = iy.try_into().unwrap();
        for (ix, ch) in line.chars().enumerate() {
            let x = ix.try_into().unwrap();
            match ch {
                ch if ch.is_digit(10) => {
                    digit_buffer.push(ch);
                    buffer_end = x;
                },
                _ => {
                    if digit_buffer.len() > 0 {
                        let part = Part::from_buffer(&digit_buffer, buffer_end, y);
                        digit_buffer.clear();
                        part_numbers.push(part);
                    }
                    if ch == '.' { continue; }
                    if !symbol_map.contains_key(&ch) {
                        symbol_map.insert(ch, Vec::new());
                    }
                    if let Some(points) = symbol_map.get_mut(&ch) {
                        points.push(Point {x, y});
                    }
                }
            }
        }
        // catch parts that are at the end of the line
        if digit_buffer.len() > 0 {
            let part = Part::from_buffer(&digit_buffer, buffer_end, y);
            digit_buffer.clear();
            part_numbers.push(part);
        }
    }
    Ok((part_numbers, symbol_map))
}

pub fn part_1(input: &(Vec<Part>, HashMap<char, Vec<Point>>)) -> Option<usize> {
    let (part_numbers, symbol_map) = input;
    let mut valid_parts:Vec<usize> = Vec::new();
    'partloop: for part in part_numbers.iter() {
        for (_key, value) in symbol_map.iter() {
            for p in value.iter() {
                if part.adjacent_to(p) {
                    valid_parts.push(part.number);
                    continue 'partloop;
                }
            }
        }
    }
    Some(valid_parts.iter().sum())
}

pub fn part_2(input: &(Vec<Part>, HashMap<char, Vec<Point>>)) -> Option<usize> {
    let (part_numbers, symbol_map) = input;
    let mut gears:Vec<usize> = Vec::new();
    let mut ratios:Vec<usize> = Vec::new();
    for (_key, value) in symbol_map.iter() {
        for p in value.iter() {
            for part in part_numbers.iter() {
                if part.adjacent_to(p) {
                    gears.push(part.number);
                }
            }
            if gears.len() == 2 {
                ratios.push(gears.iter().product())
            }
            gears.clear();
        }
    }

    Some(ratios.iter().sum())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        if let Ok((part_numbers, symbol_map)) = prepare("day03-example.txt") {
            assert_eq!(part_numbers.len(), 10);
            assert_eq!(symbol_map.len(), 4);
        }
    }
    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day03-example.txt") {
            assert_eq!(part_1(&input), Some(4361))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day03-example.txt") {
            assert_eq!(part_2(&input), Some(467835))
        }
    }
}