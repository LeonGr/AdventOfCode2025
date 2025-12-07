use std::{cmp::Ordering, collections::HashSet, future::poll_fn, ops::RangeInclusive};

fn read_input() -> (String, String) {
    let input = include_str!("../input");
    input.split_once("\n\n").map(|(ranges, ids)| (ranges.to_string(), ids.to_string())).expect("Expected ranges and IDs")
}

struct Problem {
    ranges: Vec<RangeInclusive<u64>>,
    ids: HashSet<u64>,
}

fn parse((ranges, ids): &(String, String)) -> Problem {
    let ranges = ranges.lines()
        .map(|line| {
            let (start, end) = line.split_once('-').expect("Expected 'start-end'");
           start.parse().expect("Expected start of range number")..=end.parse().expect("Expected end of range number") 

        })
        .collect();

    let ids = ids.lines()
        .map(|line| {
            line.parse().expect("Expected id number")
        })
        .collect();

    Problem { ranges, ids }
}

fn part1(problem: &Problem) -> usize {
    let mut remaining_ids = problem.ids.clone();
    let mut fresh_ids = vec![];
    let mut new_fresh_id_index = 0;

    for range in &problem.ranges {
        for id in &remaining_ids {
            if range.contains(id) {
                fresh_ids.push(*id);
            }
        }

        for seen_id in &fresh_ids[new_fresh_id_index..] {
            remaining_ids.remove(seen_id);
        }

        new_fresh_id_index = fresh_ids.len();
    }

    fresh_ids.len()
}

fn range_contains(first: &RangeInclusive<u64>, second: &RangeInclusive<u64>) -> bool {
    first.contains(second.start()) && first.contains(second.end())
}

fn part2(problem: &Problem) -> usize {
    // Remove all ranges that are contained inside another range
    let mut ranges = problem.ranges.clone();

    ranges.sort_by(|first, second| {
        let start_cmp = first.start().cmp(second.start());

        if start_cmp == Ordering::Equal {
            first.end().cmp(second.end())
        } else {
            start_cmp
        }
    });

    loop {
        let mut new_ranges: Vec<RangeInclusive<u64>> = vec![];

        let mut first_index = 0;

        while first_index + 1 < ranges.len() {
            let first = &ranges[first_index];
            let second = &ranges[first_index + 1];

            if range_contains(first, second) {
                new_ranges.push(first.clone());
                first_index += 2;
                continue;
            }

            if first.contains(second.start()) {
                let merged_range = (*first.start())..=(*second.end());
                new_ranges.push(merged_range);
                first_index += 2;
                continue;
            }

            new_ranges.push(first.clone());
            first_index += 1;
        }

        if first_index + 1 == ranges.len() {
            new_ranges.push(ranges.last().unwrap().clone());
        }

        if new_ranges.len() == ranges.len() {
            break;
        }

        ranges = new_ranges;
    }

    ranges.iter()
        .map(|range| range.clone().count())
        .sum()
}

fn main() {
    let input = read_input();
    let parsed = parse(&input);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
