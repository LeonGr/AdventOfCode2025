fn read_input() -> String {
    let input = include_str!("../input");
    input.to_string()
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.numbers.iter().sum::<u64>(),
            Operation::Multiply => self.numbers.iter().product::<u64>(),
        }
    }
}

fn parse1(input: &str) -> Vec<Problem> {
    let mut problems = vec![];

    let lines = input.lines();

    let last = lines
        .clone()
        .last()
        .expect("Expected last line to contains operations");

    for operation in last.split_whitespace() {
        let problem = Problem {
            operation: match operation {
                "*" => Operation::Multiply,
                "+" => Operation::Add,
                _ => unreachable!(),
            },
            numbers: vec![],
        };

        problems.push(problem);
    }

    let operation_lines = lines.clone().count() - 1;
    for line in lines.take(operation_lines) {
        let split = line.split_whitespace();
        for (i, number_string) in split.enumerate() {
            problems[i]
                .numbers
                .push(number_string.parse().expect("Expected a number"));
        }
    }

    problems
}

fn solve(problems: &[Problem]) -> u64 {
    problems.iter().map(Problem::solve).sum()
}

fn parse2(input: &str) -> Vec<Problem> {
    let mut problems = vec![];

    let lines = input.lines();
    let last = lines
        .clone()
        .last()
        .expect("Expected last line to contains operations");
    let max_len = last.len();

    for operation in last.split_whitespace() {
        let problem = Problem {
            operation: match operation {
                "*" => Operation::Multiply,
                "+" => Operation::Add,
                _ => unreachable!(),
            },
            numbers: vec![],
        };

        problems.push(problem);
    }

    let operation_lines = lines.clone().count() - 1;
    let char_matrix: Vec<Vec<char>> = lines
        .take(operation_lines)
        .map(|line| line.chars().collect())
        .collect();

    let mut result = Vec::new();
    for col in 0..max_len {
        let mut row = String::new();
        for r in &char_matrix {
            row.push(r[col]);
        }
        result.push(row.trim().to_string());
    }

    let transposed_string = result.join("\n");
    println!("'{transposed_string}'");

    for (i, problem_numbers) in transposed_string.split("\n\n").enumerate() {
        println!("{i}: '{problem_numbers}'");

        let numbers = problem_numbers.lines().map(|number_string| {
            println!("number: {number_string}");

            number_string.trim().parse().expect("Expected a number")
        });

        for number in numbers {
            problems[i].numbers.push(number);
        }
    }

    problems
}

fn main() {
    let input = read_input();

    let parsed1 = parse1(&input);
    let parsed2 = parse2(&input);

    println!("part1: {}", solve(&parsed1));
    println!("part2: {}", solve(&parsed2));
}
