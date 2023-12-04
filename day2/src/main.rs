use std::fs;
use std::path::Path;


#[derive(Debug)]
pub struct CubeSet {
    pub blues: u32,
    pub reds: u32,
    pub greens: u32,
}

impl CubeSet {
    pub fn new(reds: u32, greens: u32, blues: u32) -> Self {
        Self { reds, greens, blues }
    }

    pub fn power(&self) -> u32 {
        self.reds * self.greens * self.blues
    }
}

impl From<&str> for CubeSet {
    fn from(value: &str) -> Self {
        let mut res = Self {
            blues: 0,
            reds: 0,
            greens: 0,
        };
        value.split(",")
            .for_each(|color_count| {
                let mut split = color_count.trim().split(" ");
                let count: u32 = split.next().unwrap().parse().unwrap();
                let color = split.next().unwrap();
                match color {
                    "blue" => { res.blues = count; }
                    "red" => { res.reds = count; }
                    "green" => { res.greens = count; }
                    _ => unreachable!()
                }
            });
        return res;
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub cube_sets: Vec<CubeSet>,
}

impl Game {
    pub fn new(id: u32, cube_sets: Vec<CubeSet>) -> Self {
        Self {id, cube_sets}
    }

    pub fn mins(&self) -> CubeSet {
        let mut min_reds: u32 = 0;
        let mut min_greens: u32 = 0;
        let mut min_blues: u32 = 0;


        self.cube_sets.iter().for_each(|set| {
            if set.reds > min_reds {
                min_reds = set.reds;
            }
            if set.greens > min_greens {
                min_greens = set.greens;
            }
            if set.blues > min_blues {
                min_blues = set.blues;
            }
        });

        return CubeSet::new(min_reds, min_greens, min_blues);
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut s = value.split(":");
        let id = s.next()
            .unwrap()
            .trim()
            .strip_prefix("Game ")
            .unwrap()
            .parse()
            .unwrap();
        let cube_sets: Vec<CubeSet> = s.next()
            .unwrap()
            .split(";")
            .map(|s| s.trim().into())
            .collect();
        Self { id, cube_sets }
    }
}

fn main() {
    let contents = fs::read_to_string(Path::new("day2/input.txt")).unwrap();
    let games: Vec<Game> = contents.lines().map(|l| l.into()).collect();
    let red_bound: u32 = 12;
    let green_bound: u32 = 13;
    let blue_bound: u32 = 14;

    let mut sum: u32 = 0;

    for game in games.iter() {
        let mut valid = true;
        for set in game.cube_sets.iter() {
            if set.reds > red_bound || set.blues > blue_bound || set.greens > green_bound {
                valid = false;
            }
        }
        if valid {
            sum += game.id;
        }
    }

    println!("sum: {}", sum);

    let power_sum: u32 = games.iter().map(|g| g.mins().power()).sum();
    println!("Sum of the powers of the minimum required colors for each game: {}", power_sum);
}

#[cfg(test)]
mod test {
    use crate::{Game, CubeSet};

    #[test]
    fn test_parse() {
        let game: Game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".into();
        assert_eq!(game.id, 1);
        assert_eq!(game.cube_sets[0].reds, 4);
        assert_eq!(game.cube_sets[0].greens, 0);
        assert_eq!(game.cube_sets[0].blues, 3);

        assert_eq!(game.cube_sets[1].reds, 1);
        assert_eq!(game.cube_sets[1].greens, 2);
        assert_eq!(game.cube_sets[1].blues, 6);

        assert_eq!(game.cube_sets[2].reds, 0);
        assert_eq!(game.cube_sets[2].greens, 2);
        assert_eq!(game.cube_sets[2].blues, 0);
    }

    #[test]
    fn test_power() {
        assert_eq!(CubeSet::new(4,2,6).power(), 48);
    }


    #[test]
    fn test_mins() {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let cube_sets = vec![
            CubeSet::new(4, 0, 3),
            CubeSet::new(1, 2, 6),
            CubeSet::new(0, 2, 0),
        ];

        let game = Game::new(1, cube_sets);
        let mins = game.mins();
        assert_eq!(mins.reds, 4);
        assert_eq!(mins.greens, 2);
        assert_eq!(mins.blues, 6);
    }
}