pub fn solve1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let n = line.chars().fold((0, 0), |mut acc, c| {
                if c.is_numeric() {
                    let n = c.to_digit(10).unwrap();
                    if acc.0 == 0 {
                        acc.0 = n
                    } else {
                        acc.1 = n
                    }
                }
                acc
            });
            match n {
                (a, 0) => a * 10 + a,
                (a, b) => a * 10 + b,
            }
        })
        .sum()
}

pub fn solve2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let tmp = (0..line.len())
                .filter_map(|i| match &line[i..] {
                    s if s.starts_with("one") => Some(1),
                    s if s.starts_with("two") => Some(2),
                    s if s.starts_with("three") => Some(3),
                    s if s.starts_with("four") => Some(4),
                    s if s.starts_with("five") => Some(5),
                    s if s.starts_with("six") => Some(6),
                    s if s.starts_with("seven") => Some(7),
                    s if s.starts_with("eight") => Some(8),
                    s if s.starts_with("nine") => Some(9),
                    s => s.chars().next().and_then(|c| c.to_digit(10)),
                })
                .fold((None, None), |acc, x| match acc {
                    (None, _) => (Some(x), Some(x)),
                    _ => (acc.0, Some(x)),
                });

            match tmp {
                (Some(n), Some(m)) => n * 10 + m,
                _ => 0,
            }
        })
        .sum()
}

pub fn day1_part2(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .enumerate()
                .filter_map(|(i, _)| match &line[i..] {
                    s if s.starts_with("one") => Some(1),
                    s if s.starts_with("two") => Some(2),
                    s if s.starts_with("three") => Some(3),
                    s if s.starts_with("four") => Some(4),
                    s if s.starts_with("five") => Some(5),
                    s if s.starts_with("six") => Some(6),
                    s if s.starts_with("seven") => Some(7),
                    s if s.starts_with("eight") => Some(8),
                    s if s.starts_with("nine") => Some(9),
                    s => s.chars().next().and_then(|c| c.to_digit(10)),
                })
                .fold((None, None), |acc, x| match acc {
                    (None, _) => (Some(x), Some(x)),
                    _ => (acc.0, Some(x)),
                });

            nums.0.unwrap() * 10 + nums.1.unwrap()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../fixtures/input");

    #[test]
    fn test_solve1() {
        let data = "1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet";
        assert_eq!(solve1(data), 142);

        assert_eq!(solve1(INPUT), 55834);
    }

    #[test]
    fn test_solve2() {
        let data = "two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen";
        assert_eq!(solve2(data), 281);

        assert_eq!(solve2(INPUT), 53221);
    }
}
