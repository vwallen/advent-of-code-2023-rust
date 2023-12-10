use std::collections::HashMap;
use crate::read_input_lines;
use anyhow::Result;
use colored::Colorize;
use itertools::Itertools;
use std::cmp::{min, max};

pub fn prepare(file_name: &str) -> Result<((isize, isize), HashMap<(isize, isize), Vec<(isize, isize)>>)> {
    let input = read_input_lines(file_name);

    // parse the content into a big ol' map
    let mut start:(isize, isize) = (0,0);
    let mut pipe_segments: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    for (m, line) in input.iter().enumerate() {
        for (n, c) in line.chars().enumerate() {
            let x = n.try_into().unwrap();
            let y = m.try_into().unwrap();
            let n = (x, y - 1);
            let s = (x, y + 1);
            let w = (x - 1, y);
            let e = (x + 1, y);
            pipe_segments.insert((x, y), match c {
                '-' => vec![e, w],
                '|' => vec![n, s],
                '7' => vec![w, s],
                'J' => vec![n, w],
                'F' => vec![s, e],
                'L' => vec![n, e],
                _  => vec![],
            });
            if c == 'S' {
                start = (x, y);
            }
        }
    }

    // figure out who links to start
    let start_links:Vec<(isize, isize)> = vec![
        (start.0, start.1 - 1),
        (start.0, start.1 + 1),
        (start.0 - 1, start.1),
        (start.0 + 1, start.1),
    ]
        .iter()
        .filter(|loc| {
            if let Some(links) = pipe_segments.get(loc) {
                links.contains(&start)
            } else { false }
        })
        .map(|loc| loc.clone())
        .collect();
    pipe_segments.insert(start, start_links);

    Ok((start, pipe_segments))
}

pub fn find_route(start:&(isize, isize), pipe_segments:&HashMap<(isize, isize), Vec<(isize, isize)>>) -> Vec<(isize, isize)> {
    let mut route:Vec<(isize, isize)> = vec![start.clone()];
    let mut next = pipe_segments.get(&start).unwrap()[0];
    loop {
        route.push(next);
        let links = pipe_segments.get(&next).unwrap();
        let options:Vec<(isize, isize)> = links
            .iter()
            .filter(|link| !route.contains(link))
            .map(|link| link.clone())
            .collect();
        if options.len() == 0 { break; }
        next = options[0];
    }
    route
}

pub fn to_polygon(route:&Vec<(isize, isize)>) -> Vec<((isize, isize), (isize, isize))> {
    let mut corners:Vec<(isize, isize)> = vec![route.first().unwrap().clone()];
    let mut prev:&(isize, isize) = route.first().unwrap();
    let mut vertical = route[0].0 == route[1].0;
    for curr in route.iter().skip(1) {
        if vertical != (prev.0 == curr.0) {
            vertical = prev.0 == curr.0;
            corners.push(prev.clone());
        }
        prev = curr;
    }
    corners.push(route.last().unwrap().clone());
    corners
        .iter()
        .tuple_windows()
        .map(|(a,b)| (a.clone(), b.clone()))
        .collect()
}

pub fn ray_crosses(p:&(isize, isize), seg:&((isize, isize), (isize, isize)), ) -> bool {
    // ray is always going to be horizontal
    // seg is always be axis aligned
    let mut out:bool = false;
    if seg.0.0 == seg.1.0 {
        // only care vertical segment
        let miny = min(seg.0.1, seg.1.1);
        let maxy = max(seg.0.1, seg.1.1);
        // include the bottom, but not the top of each segment
        out = (p.0 > seg.0.0) && (p.1 > miny) && (p.1 <= maxy)
    }
    out
}


pub fn part_1((start, pipe_segments): &((isize, isize), HashMap<(isize, isize), Vec<(isize, isize)>>)) -> Option<usize> {
    let route = find_route(start, pipe_segments);
    Some(route.len() / 2)
}

pub fn part_2((start, pipe_segments): &((isize, isize), HashMap<(isize, isize), Vec<(isize, isize)>>)) -> Option<usize> {
    let route = find_route(start, pipe_segments);
    let polygon = to_polygon(&route);

    let mut count:usize = 0;
    for (x, y) in pipe_segments.keys() {
        if !route.contains(&(x.clone(), y.clone())) {
            let cross_count = polygon
                .iter()
                .filter(|seg| ray_crosses(&(x.clone(), y.clone()), seg))
                .count();
            if cross_count % 2 != 0 {
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn char_to_pipe(c:char) -> char {
    // ─│┌┐└┘  ╭╮╰╯
    match c {
        '-' => '─',
        '|' => '│',
        'F' => '╭',
        '7' => '╮',
        'L' => '╰',
        'J' => '╯',
        'S' => '●',
         c  => c,
    }
}

pub fn print_map(map:&Vec<String>, route:&Vec<(isize, isize)>) {

    let polygon = to_polygon(&route);
    let polygon_points:Vec<(isize, isize)> = polygon.iter().map(|(a, _)| a.clone()).collect();

    print!("\n    0123456789abcedf");
    print!("\n");
    for (m, line) in map.iter().enumerate() {
        print!("{m:0>3} ");
        for (n, c) in line.chars().enumerate() {
            let x = n.try_into().unwrap();
            let y = m.try_into().unwrap();
            let p = char_to_pipe(c);
            let cross_count = polygon
                .iter()
                .filter(|seg| ray_crosses(&(x, y), seg))
                .count();
            if route.contains(&(x, y)) {
                if polygon_points.contains(&(x, y)) {
                    print!("{}", p.to_string());
                } else {
                    print!("{}", p.to_string().bright_green());
                }
            } else {
                if cross_count % 2 == 0 {
                    print!("{}", p.to_string().truecolor(96, 96, 96));
                } else {
                    print!("{}", "■".to_string().bright_red());
                }
            }
        }
        print!("\n");
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_map() {
        for file_name in vec!["day10-example-2.txt", "day10-example-3.txt", "day10-example-4.txt"].iter() {
            if let Ok((start, pipe_segments)) = prepare(file_name) {
                let map   = read_input_lines(file_name);
                let route = find_route(&start, &pipe_segments);
                print_map(&map, &route);
            }
        }
    }

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day10-example.txt") {
            assert_eq!(part_1(&input), Some(8))
        }
    }

    #[test]
    #[ignore]
    fn test_part_1_puzzle() {
        if let Ok(input) = prepare("day10.txt") {
            assert_eq!(part_1(&input), Some(6842))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day10-example-2.txt") {
            assert_eq!(part_2(&input), Some(4))
        }
        if let Ok(input) = prepare("day10-example-3.txt") {
            assert_eq!(part_2(&input), Some(8))
        }
        if let Ok(input) = prepare("day10-example-4.txt") {
            assert_eq!(part_2(&input), Some(10))
        }
    }
    #[test]
    #[ignore]
    fn test_part_2_puzzle() {
        if let Ok(input) = prepare("day10.txt") {
            if let Ok((start, pipe_segments)) = prepare("day10.txt") {
                let map   = read_input_lines("day10.txt");
                let route = find_route(&start, &pipe_segments);
                print_map(&map, &route);
            }
            assert_eq!(part_2(&input), Some(393))
        }
    }
}