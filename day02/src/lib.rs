use std::str::FromStr;

use anyhow::Context;

pub fn solve1(input: &str, limits: (u32, u32, u32)) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            line.parse::<Game>()
                .ok()
                .filter(|game| game.valid(limits))
                .map(|game| game.id)
        })
        .sum()
}

pub fn solve2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| line.parse::<Game>().ok().map(|game| game.max_product()))
        .sum()
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn valid(&self, limits: (u32, u32, u32)) -> bool {
        self.rounds.iter().all(|round| round.within_limits(limits))
    }

    fn max_product(&self) -> u32 {
        self.rounds
            .iter()
            .fold(Round::default(), |mut acc, each| {
                acc.red = acc.red.max(each.red);
                acc.green = acc.green.max(each.green);
                acc.blue = acc.blue.max(each.blue);
                acc
            })
            .product()
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stat, rounds) = s.trim().split_once(':').ok_or("cant find ':'")?;
        let (_, n) = stat
            .split_once(' ')
            .ok_or("no ' ' between game and number")?;
        let n = n
            .parse::<u32>()
            .context("parse game number failed")
            .or_else(|_| Err(format!("parse '{}' to u32 failed", n)))?;
        let rounds: Vec<Round> = rounds
            .split(';')
            .filter_map(|round| round.parse().ok())
            .collect();

        Ok(Self { id: n, rounds })
    }
}

#[derive(Debug, Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn product(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn within_limits(&self, (red, green, blue): (u32, u32, u32)) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round: Self = Default::default();

        for each in s.split(',') {
            let (n, color) = each
                .trim()
                .split_once(' ')
                .ok_or("Can't split by ' '".to_string())?;
            let n = n
                .trim()
                .parse::<u32>()
                .map_err(|_| format!("Failed to parse {}", n).to_string())?;

            match color {
                "red" => {
                    round.red = n;
                }
                "green" => {
                    round.green = n;
                }
                "blue" => {
                    round.blue = n;
                }
                _ => {}
            }
        }

        Ok(round)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_DATA: &str = include_str!("../fixtures/input");
    const TEST_DATA: &str = include_str!("../fixtures/test");

    #[test]
    fn test_solve1() {
        // assert_eq!(solve1(TEST_DATA, (12, 13, 14)), 8);
        assert_eq!(solve1(INPUT_DATA, (12, 13, 14)), 2406);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(TEST_DATA), 2286);
        assert_eq!(solve2(INPUT_DATA), 78375);
    }
}
