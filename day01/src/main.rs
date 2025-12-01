// 0-99
// 11 + R8 = 19
// 19 + L19 = 0
// 0 + L1 = 99
// 99 + R1 = 0
// Start: 50
// Solution: the number of times the dial is left pointing at 0 after any rotation in the sequence

enum Direction {
    Left(u16),
    Right(u16),
}

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

fn parse(lines: &[String]) -> Vec<Direction> {
    lines
        .iter()
        .map(|line| {
            let number: u16 = line[1..]
                .parse()
                .expect("Line should contain L/R + a number");
            if line.starts_with('L') {
                Direction::Left(number)
            } else {
                Direction::Right(number)
            }
        })
        .collect()
}

fn get_updated_dial(current_dial: i32, direction: &Direction) -> i32 {
    let new_dial = match direction {
        Direction::Left(n) => current_dial - i32::from(*n),
        Direction::Right(n) => current_dial + i32::from(*n),
    };

    new_dial.rem_euclid(100)
}

fn part1(directions: &[Direction]) -> u32 {
    let mut dial: i32 = 50;
    let mut zeroes: u32 = 0;

    for direction in directions {
        dial = get_updated_dial(dial, direction);

        if dial == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

fn get_passes_by_zero(current_dial: i32, direction: &Direction) -> u32 {
    let dial = current_dial;
    let mut passes_by_zero: u32 = 0;

    match direction {
        Direction::Left(change) => {
            let full_cycles = change / 100;
            passes_by_zero += u32::from(full_cycles);
            let remaining_change = change % 100;

            if current_dial != 0 && (dial - i32::from(remaining_change)) <= 0 {
                passes_by_zero += 1;
            }
        }
        Direction::Right(change) => {
            let full_cycles = change / 100;
            passes_by_zero += u32::from(full_cycles);
            let remaining_change = change % 100;

            if current_dial != 0 && (dial + i32::from(remaining_change)) >= 100 {
                passes_by_zero += 1;
            }
        }
    };

    passes_by_zero
}

fn part2(directions: &[Direction]) -> u32 {
    let mut dial: i32 = 50;
    let mut zeroes: u32 = 0;

    for direction in directions {
        let passes_by_zero = get_passes_by_zero(dial, direction);

        dial = get_updated_dial(dial, direction);

        zeroes += passes_by_zero;
    }

    zeroes
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
