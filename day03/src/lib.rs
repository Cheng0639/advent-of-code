pub fn solve1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut nums: Vec<u32> = Vec::new();
    let mut index: (Option<usize>, Option<usize>) = (None, None);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                match index {
                    (None, _) => {
                        index.0 = Some(x);
                        index.1 = Some(x);
                        {}
                    }
                    _ => index.1 = Some(x),
                }
                if x != line.len() - 1 {
                    continue;
                }
            }

            if let (None, None) = index {
                continue;
            }

            let (start, end) = index;
            let start = start.unwrap();
            let end = end.unwrap();
            let n = line[start..end + 1].parse::<u32>().unwrap();
            index = (None, None);
            let start = if start > 0 { start - 1 } else { start };
            let end = if end < line.len() - 1 { end + 1 } else { end };
            let valid_symbol = |c: char| c.is_ascii_punctuation() && c != '.';

            // up row
            if y != 0 {
                let above_chars = lines[y - 1]
                    .chars()
                    .skip(start)
                    .take(end - start + 1)
                    .collect::<String>();
                if above_chars.chars().any(valid_symbol) {
                    nums.push(n);
                    continue;
                }
            }

            let left_chars = line.chars().nth(start).unwrap();
            let right_chars = line.chars().nth(end).unwrap();
            // current row
            if valid_symbol(left_chars) || valid_symbol(right_chars) {
                nums.push(n);
                continue;
            }

            // down row
            if y + 1 < lines.len() {
                let below_chars = lines[y + 1]
                    .chars()
                    .skip(start)
                    .take(end - start + 1)
                    .collect::<String>();
                if below_chars.chars().any(valid_symbol) {
                    nums.push(n);
                    continue;
                }
            }
        }
        index = (None, None);
    }
    nums.iter().sum()
}

pub fn solve2(input: &str) -> u32 {
    let mut nums: Vec<(u32, u32)> = Vec::new();
    let mut line_iter = input.lines().map(|s| Line::from(s));
    let mut prev: Option<Line> = None;
    let mut current = line_iter.next();
    let mut next = line_iter.next();
    while let Some(l) = &current {
        l.parts.iter().for_each(|p| {
            let temp = [&prev, &current, &next]
                .iter()
                .fold(Vec::new(), |mut acc, f| {
                    if let Some(l) = f {
                        acc.extend(l.nums.iter().filter_map(|n| {
                            let start = if n.start == 0 { n.start } else { n.start - 1 };
                            let end = if n.end == l.s.len() - 1 {
                                n.end
                            } else {
                                n.end + 1
                            };
                            if p.index >= start && p.index <= end {
                                Some(n.value)
                            } else {
                                None
                            }
                        }));
                    }
                    acc
                });
            if temp.len() == 2 {
                nums.push((temp[0], temp[1]));
            }
        });

        prev = current;
        current = next;
        next = line_iter.next();
    }

    nums.iter().map(|(a, b)| a * b).sum()
}

#[derive(Debug)]
struct Line<'a> {
    // original string
    s: &'a str,
    /// numbers in s and where they are
    nums: Vec<Num>,
    parts: Vec<Gear>,
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(s: &'a str) -> Self {
        let mut nums = Vec::new();
        let mut parts = Vec::new();
        let mut index: (Option<usize>, Option<usize>) = (None, None);
        for (i, c) in s.chars().enumerate() {
            if c.is_numeric() {
                match index {
                    (None, _) => {
                        index.0 = Some(i);
                        index.1 = Some(i);
                    }
                    _ => index.1 = Some(i),
                }

                if i != s.len() - 1 {
                    continue;
                }
            }

            if c.is_ascii_punctuation() && c != '.' {
                parts.push(Gear { char: c, index: i });
            }

            match index {
                (Some(start), Some(end)) => {
                    let n = s[start..end + 1].parse::<u32>().unwrap();
                    nums.push(Num {
                        value: n,
                        start,
                        end,
                    });
                    index = (None, None);
                }
                _ => {}
            }
        }

        if let (Some(start), Some(end)) = index {
            let n = s[start..end + 1].parse::<u32>().unwrap();
            nums.push(Num {
                value: n,
                start,
                end,
            });
        }

        Self { s, nums, parts }
    }
}

#[derive(Debug)]
struct Num {
    value: u32,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Gear {
    char: char,
    index: usize,
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    const TEST_DATA: &str = include_str!("../fixtures/test");
    const INPUT_DATA: &str = include_str!("../fixtures/input");

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(TEST_DATA), 4361);
        assert_eq!(solve1(INPUT_DATA), 506727);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(TEST_DATA), 467835);
        assert_eq!(solve2(INPUT_DATA), 75220503);
    }
}
