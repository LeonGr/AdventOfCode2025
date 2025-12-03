fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

// Get first index of max
fn get_max_with_index(values: &Vec<(usize, &u32)>) -> (usize, u32) {
    let mut max: i32 = -1;
    let mut max_index: usize = 0;
    for (i, battery) in values {
        let battery_value = (**battery) as i32;
        if battery_value > max {
            max = battery_value;
            max_index = *i;
        }
    }

    (max_index, max as u32)
}

fn get_max_joltage(line: &str) -> usize {
    let battery_values: Vec<_> = line
        .chars()
        .map(|battery| battery.to_digit(10).unwrap())
        .collect();

    let length = line.len();

    let without_last = battery_values.iter().take(length - 1).enumerate().collect();
    let (max_index, max) = get_max_with_index(&without_last);

    let from_previous_max = battery_values
        .iter()
        .enumerate()
        .skip(max_index + 1)
        .collect();
    let (_, second_max) = get_max_with_index(&from_previous_max);

    (max * 10 + second_max) as usize
}

fn part1(lines: &[String]) -> usize {
    lines.iter().map(|line| get_max_joltage(line)).sum()
}

fn get_max_joltage2(line: &str) -> u64 {
    let battery_values: Vec<_> = line
        .chars()
        .map(|battery| battery.to_digit(10).unwrap())
        .collect();
    let length = battery_values.len();
    let mut values: Vec<_> = vec![];
    let mut value = 0;
    let mut start_index = 0;

    for remaining_batteries in (0..12).rev() {
        let numbers = battery_values
            .iter()
            .enumerate()
            .take(length - remaining_batteries)
            .skip(start_index)
            .collect();
        let (index, max) = get_max_with_index(&numbers);

        value += (max as u64) * 10_u64.pow(remaining_batteries as u32);
        values.push(max);
        start_index = index + 1;
    }

    value
}

fn part2(lines: &[String]) -> u64 {
    lines.iter().map(|line| get_max_joltage2(line)).sum()
}

fn main() {
    let lines = read_input();

    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));
}
