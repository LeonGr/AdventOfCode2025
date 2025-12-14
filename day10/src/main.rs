// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
//
// indicator light diagram: [...]
// button wiring schematics: (...)
// - list which lights it toggles
// joltage requirements: {...}
//
// all lights initially off, must match diagram
// goal: find the fewest total presses required to correctly configure the lights

// Nom:
// delimited: for parsing stuff inside lists

use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::{
        complete::{u64, u8},
        one_of,
    },
    combinator::map,
    multi::{many0, separated_list0},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use highs::*;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

#[derive(Debug)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<u8>>,
    joltages: Vec<u16>,
}

fn state_to_bool(input: &str) -> IResult<&str, bool> {
    map(one_of(".#"), |c| c == '#').parse(input)
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let target_parser = delimited(tag("["), many0(state_to_bool), tag("]"));

        let joltages_parser = delimited(tag("{"), separated_list0(tag(","), u64), tag("}"));

        let buttons_parser = separated_list0(
            tag(" "),
            delimited(tag("("), separated_list0(tag(","), u8), tag(")")),
        );

        let buttons_and_joltages = separated_pair(buttons_parser, tag(" "), joltages_parser);

        let all_parsers = separated_pair(target_parser, tag(" "), buttons_and_joltages);

        let mut map_to_machine = map(all_parsers, |(x, (y, z))| {
            let target = x;
            let buttons = y;
            let joltages = z.iter().map(|jolt| *jolt as u16).collect();

            Machine {
                target,
                buttons,
                joltages,
            }
        });

        let parsed = map_to_machine.parse(input);

        match parsed {
            Ok((_, machine)) => Ok(machine),
            Err(err) => Err(err.to_string()),
        }
    }
}

fn parse(lines: &[String]) -> Vec<Machine> {
    lines
        .iter()
        .map(|line| Machine::from_str(line).unwrap())
        .collect()
}

fn example() {
    // Solves: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1)
    let mut pb = ColProblem::new();

    let nonneg_a = pb.add_row(0..); // later we use this in bounds only
    let nonneg_b = pb.add_row(0..);
    let nonneg_c = pb.add_row(0..); // later we use this in bounds only
    let nonneg_d = pb.add_row(0..);
    let nonneg_e = pb.add_row(0..); // later we use this in bounds only
    let nonneg_f = pb.add_row(0..);

    // e + f = 2k
    let row0 = pb.add_row(0..=0);
    // b + f = 2k + 1
    let row1 = pb.add_row(1..=1);
    // c + d + e = 2k + 1
    let row2 = pb.add_row(1..=1);
    // a + b + d = 2k
    let row3 = pb.add_row(0..=0);

    // a
    pb.add_integer_column(1.0, 0.., [(row3, 1.0), (nonneg_a, 1.0)]);

    // b
    pb.add_integer_column(1.0, 0.., [(row1, 1.0), (row3, 1.0), (nonneg_b, 1.0)]);

    // c
    pb.add_integer_column(1.0, 0.., [(row2, 1.0), (nonneg_c, 1.0)]);

    // d
    pb.add_integer_column(1.0, 0.., [(row2, 1.0), (row3, 1.0), (nonneg_d, 1.0)]);

    // e
    pb.add_integer_column(1.0, 0.., [(row0, 1.0), (row2, 1.0), (nonneg_e, 1.0)]);

    // f
    pb.add_integer_column(1.0, 0.., [(row0, 1.0), (row1, 1.0), (nonneg_f, 1.0)]);

    // rows
    pb.add_integer_column(0.0, 0.., [(row0, -2.0)]);
    pb.add_integer_column(0.0, 0.., [(row1, -2.0)]);
    pb.add_integer_column(0.0, 0.., [(row2, -2.0)]);
    pb.add_integer_column(0.0, 0.., [(row3, -2.0)]);

    let solution = pb.optimise(Sense::Minimise).solve().get_solution();

    let columns = solution.columns();
    // println!("columns: {:?}", columns);
    // println!("a: {}, b: {}, c: {}, d: {}, e: {}, f: {}", columns[0], columns[1], columns[2], columns[3], columns[4], columns[5]);
}

impl Machine {
    // We convert the machine to a linear equation so we can solve it with integer programming:
    // Example: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1)
    // buttons:
    // a (3)
    // b (1,3)
    // c (2)
    // d (2,3)
    // e (0,2)
    // f (0,1)
    // We can convert to these rows (even or odd presses for on/off):
    // e + f - 2w = 0
    // b + f - 2x = 1
    // c + d + e - 2y = 1
    // a + b + d - 2y = 0
    //
    // Alternative:
    // Since pressing a button twice resets leaves the state back in the previous state, we know we
    // only have to press each button at most once. So we can pretty easily try all permutations
    fn solve1(&self) -> usize {
        let buttons = &self.buttons;
        let target = &self.target;

        let mut pb = ColProblem::new();

        // Prevent negative number of key presses
        let above_zero_bounds: Vec<Row> = (0..(buttons.len())).map(|_| pb.add_row(0..)).collect();

        let even_odd_rows: Vec<Row> = target
            .iter()
            .map(|&bool| {
                if bool {
                    pb.add_row(1..=1)
                } else {
                    pb.add_row(0..=0)
                }
            })
            .collect();

        for (i, button) in buttons.iter().enumerate() {
            let mut row_factors = vec![(above_zero_bounds[i], 1.0)];
            for &n in button {
                row_factors.push((even_odd_rows[n as usize], 1.0));
            }

            // col_factor 1, since it does contribute to the presses
            pb.add_integer_column(1.0, 0.., row_factors);
        }

        // Add the even/odd target, with col_factor 0, since they don't contribute to the presses
        for row in even_odd_rows {
            pb.add_integer_column(0.0, 0.., [(row, -2.0)]);
        }

        // We want to minimize the sum of a + b ... + f
        let solution = pb.optimise(Sense::Minimise).solve().get_solution();

        let columns = solution.columns();

        // Floating point math is wack, so we we may get 0.999... answers,  so we round them
        let answer = columns
            .iter()
            .take(buttons.len())
            .map(|c| (c.round()) as usize)
            .sum();

        answer
    }

    // Here we simply use the joltage as the target, so we don't need 2k, for k is integer, like in part1
    //
    // Alternative:
    // Gaussian elimination
    // b = A*x + p
    // where
    // b is the target vector
    // A is a matrix for showing presses
    // - each column is a button
    // - each rows is which light changes when the button gets pressed
    // x is the number of presses
    // p is the initial vector
    //
    // Solve by
    // x = (b - p ) * A^-1
    fn solve2(&self) -> usize {
        let buttons = &self.buttons;
        let joltages = &self.joltages;

        let mut pb = ColProblem::new();

        let above_zero_bounds: Vec<Row> = (0..(buttons.len())).map(|_| pb.add_row(0..)).collect();

        let joltage_rows: Vec<Row> = joltages
            .iter()
            .map(|&value| pb.add_row(value..=value))
            .collect();

        for (i, button) in buttons.iter().enumerate() {
            let mut row_factors = vec![(above_zero_bounds[i], 1.0)];
            for &n in button {
                row_factors.push((joltage_rows[n as usize], 1.0));
            }

            pb.add_integer_column(1.0, 0.., row_factors);
        }

        let solution = pb.optimise(Sense::Minimise).solve().get_solution();

        let columns = solution.columns();

        let answer = columns
            .iter()
            .take(buttons.len())
            .map(|c| (c.round()) as usize)
            .sum();

        answer
    }
}

fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(|machine| machine.solve1()).sum()
}

fn part2(machines: &[Machine]) -> usize {
    machines.iter().map(|machine| machine.solve2()).sum()
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
