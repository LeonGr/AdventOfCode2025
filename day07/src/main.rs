use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}

type Coord = (i64, i64);

#[derive(Debug)]
struct Diagram {
    start: Coord,
    splitters: HashSet<Coord>,
    height: usize,
}

fn parse(lines: &[String]) -> Diagram {
    let start = lines
        .first()
        .and_then(|line| line.find('S'))
        .expect("Expected start in first line");
    let start = (start as i64, 0);

    let height = lines.len();

    let splitters = lines
        .iter()
        .enumerate()
        .skip(1)
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, char)| {
                    if char == '^' {
                        Some((x as i64, y as i64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Coord>>()
        })
        .collect();

    Diagram {
        start,
        splitters,
        height,
    }
}

fn part1(diagram: &Diagram) -> usize {
    let mut splits = 0;
    let mut beams = HashSet::new();
    beams.insert(diagram.start);
    let mut rounds = 0;

    while rounds < diagram.height {
        let mut new_beams = HashSet::new();

        for (beam_x, beam_y) in &beams {
            if diagram.splitters.contains(&(*beam_x, beam_y + 1)) {
                splits += 1;
                new_beams.insert((beam_x - 1, beam_y + 1));
                new_beams.insert((beam_x + 1, beam_y + 1));
            } else {
                new_beams.insert((*beam_x, beam_y + 1));
            }
        }

        beams = new_beams;

        rounds += 1;
    }

    splits
}

fn part2(diagram: &Diagram) -> i64 {
    let mut beams = HashMap::new();
    beams.insert(diagram.start, 1);
    let mut rounds = 0;

    while rounds < diagram.height {
        let mut new_beams: HashMap<(i64, i64), i64> = HashMap::new();

        for ((beam_x, beam_y), count) in &beams {
            if diagram.splitters.contains(&(*beam_x, beam_y + 1)) {
                new_beams
                    .entry((beam_x - 1, beam_y + 1))
                    .and_modify(|current_count| current_count.add_assign(count))
                    .or_insert(*count);
                new_beams
                    .entry((beam_x + 1, beam_y + 1))
                    .and_modify(|current_count| current_count.add_assign(count))
                    .or_insert(*count);
            } else {
                new_beams
                    .entry((*beam_x, beam_y + 1))
                    .and_modify(|current_count| current_count.add_assign(count))
                    .or_insert(*count);
            }
        }

        beams = new_beams;

        rounds += 1;
    }

    beams.values().sum()
}
