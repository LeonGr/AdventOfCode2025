// Input:
// - Ranges, separated by commas: first ID & last ID
// - Invalid: any ID which is some sequence of digits repeated twice
// - No leading zeroes

use std::str::FromStr;

#[derive(Debug)]
struct ID {
    number: usize,
    original: String,
}

#[derive(Debug)]
struct Range {
    start: ID,
    end: ID,
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_string, end_string) = s
            .split_once('-')
            .ok_or_else(|| format!("Expected valid range, got '{s}'"))?;

        let start_number: usize = start_string
            .parse()
            .map_err(|_| format!("Expected start of range to be a number, got '{start_string}'"))?;
        let end_number: usize = end_string
            .parse()
            .map_err(|_| format!("Expected end of range to be a number, got '{end_string}'"))?;

        let start = ID {
            number: start_number,
            original: start_string.to_string(),
        };

        let end = ID {
            number: end_number,
            original: end_string.to_string(),
        };

        Ok(Range { start, end })
    }
}

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .trim()
        .split(',')
        .map(std::string::ToString::to_string)
        .collect()
}

fn parse(lines: &[String]) -> Vec<Range> {
    lines
        .iter()
        .map(|line| line.parse().unwrap_or_else(|error| panic!("{}", error)))
        .collect()
}

impl ID {
    fn is_valid(&self) -> bool {
        let length = self.original.len();

        // If the length is not even, it's always valid
        if length % 2 != 0 {
            return true;
        }

        let half = length / 2;

        let start = &self.original[..half];
        let end = &self.original[half..];

        start != end
    }

    fn is_valid_2(&self) -> bool {
        let length = self.original.len();
        let half = length / 2;

        for sequence_length in 1..=half {
            let fits_n_times = length / sequence_length;

            let sequence = &self.original[..sequence_length];
            let repeated = sequence.repeat(fits_n_times);

            if self.original == repeated {
                return false;
            }
        }

        true
    }
}

impl Range {
    fn value(&self) -> usize {
        let mut total = 0;
        for number in self.start.number..=self.end.number {
            let id = ID {
                number,
                original: number.to_string(),
            };

            if !id.is_valid() {
                total += number;
            }
        }

        total
    }

    fn value_2(&self) -> usize {
        let mut total = 0;
        for number in self.start.number..=self.end.number {
            let id = ID {
                number,
                original: number.to_string(),
            };

            if !id.is_valid_2() {
                total += number;
            }
        }

        total
    }
}

fn part1(ranges: &[Range]) -> usize {
    ranges.iter().map(Range::value).sum()
}

fn part2(ranges: &[Range]) -> usize {
    ranges.iter().map(Range::value_2).sum()
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
