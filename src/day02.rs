use adventofcode_2023::read_input_lines;
use anyhow::Result;
use std::cmp;

#[derive(Debug)]
pub struct Draw {
    reds: usize,
    greens: usize,
    blues: usize,
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    draws: Vec<Draw>,
}
impl Game {
    fn max_drawn(&self) -> Draw {
        let mut out = Draw{reds: 0, greens: 0, blues: 0};
        for draw in self.draws.iter() {
            out.reds = cmp::max(out.reds, draw.reds);
            out.greens = cmp::max(out.greens, draw.greens);
            out.blues = cmp::max(out.blues, draw.blues);
        }
        out
    }
    fn power(&self) -> usize {
        let Draw{reds, greens, blues} = self.max_drawn();
        (reds * greens * blues) as usize
    }
}

pub fn prepare(file_name: &str) -> Result<Vec<Game>> {
    let input = read_input_lines(file_name);
    let mut output:Vec<Game> = Vec::new();
    for line in input.iter() {
        let (game_number, suffix) = sscanf!(line, "Game {usize}: {str}").unwrap();
        let mut game = Game{id: game_number, draws: Vec::new()};
        for draw in suffix.split("; ") {
            let (mut reds, mut greens, mut blues) = (0, 0, 0);
            for cubes in draw.split(", ") {
                if let Ok(r) = sscanf!(cubes, "{usize} red")   { reds = r };
                if let Ok(g) = sscanf!(cubes, "{usize} green") { greens = g };
                if let Ok(b) = sscanf!(cubes, "{usize} blue")  { blues = b };
            }
            game.draws.push(Draw{reds, greens, blues});
        }
        output.push(game);
    }
    Ok(output)
}

pub fn part_1(_input: &Vec<Game>) -> Option<usize> {
    let mut qualified_games:Vec<usize> = Vec::new();
    for game in _input.iter() {
        let Draw{ reds, greens, blues } = game.max_drawn();
        if reds <= 12 && greens <= 13 && blues <= 14 {
            qualified_games.push(game.id as usize);
        }
    }
    Some(qualified_games.iter().sum())
}

pub fn part_2(_input: &Vec<Game>) -> Option<usize> {
    let mut game_powers:Vec<usize> = Vec::new();
    for game in _input.iter() {
        game_powers.push(game.power())
    }
    Some(game_powers.iter().sum())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        if let Ok(input) = prepare("day02-example.txt") {
            assert_eq!(input.len(), 5);
        }
    }

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day02-example.txt") {
            assert_eq!(part_1(&input), Some(8))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day02-example.txt") {
            assert_eq!(part_2(&input), Some(2286))
        }
    }
}