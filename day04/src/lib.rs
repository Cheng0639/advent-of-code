use std::collections::HashSet;

pub fn solve1(input: &str) -> usize {
    input
        .lines()
        .map(|line| Game::from(line).score())
        .sum()
}

struct Game {
    id: usize,
    win_numbers: HashSet<usize>,
    have_numbers: HashSet<usize>,
}

impl Game {
    fn score(&self) -> usize {
        let base: usize = 2;
        let count = self.win_numbers.intersection(&self.have_numbers).count();
        match count {
            0 => 0,
            n => base.pow((n - 1) as u32),
        }
    }
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let (info, nums) = s.split_once(':').unwrap();
        let (_, id) = info.split_once(' ').unwrap();
        let id = id.trim().parse().unwrap();
        let (win_nums, have_nums) = nums.split_once('|').unwrap();
        let win_nums = win_nums
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let have_nums = have_nums
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Self {
            id,
            win_numbers: win_nums,
            have_numbers: have_nums,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = include_str!("../fixtures/test");
    const INPUT_DATA: &str = include_str!("../fixtures/input");

    #[test]
    fn it_works() {
        assert_eq!(solve1(TEST_DATA), 13);
        assert_eq!(solve1(INPUT_DATA), 20107);
    }
}
