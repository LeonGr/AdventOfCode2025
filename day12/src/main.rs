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
    // println!("part2: {}", part2(&parsed));
}

#[derive(Debug)]
struct Problem {
    width: usize,
    height: usize,
    shapes_required: Vec<usize>,
}

#[derive(Debug)]
struct Parsed {
    shapes: Vec<usize>,
    problems: Vec<Problem>,
}

fn parse(lines: &[String]) -> Parsed {
    let non_empty_lines: Vec<&String> = lines.iter()
        .filter(|line| !line.is_empty())
        .collect();

    let mut shapes = vec![];
    let mut problems = vec![];
    let mut parsing_shapes = true;

    let mut line_index = 0;

    let mut seen_blocks = 0;

    while let Some(line) = non_empty_lines.get(line_index) {
        // println!("line: '{line}'");
        if parsing_shapes {
            // We finishes counting all blocks in a shape:
            if line.chars().nth(1).unwrap() == ':' {
                if seen_blocks > 0 {
                    shapes.push(seen_blocks);
                    seen_blocks = 0;
                }
            }
            // Contains a packing problem line:
            else if line.contains(":") {
                if seen_blocks > 0 {
                    shapes.push(seen_blocks);
                    seen_blocks = 0;
                }
                parsing_shapes = false;
                continue;
            }
            // We're counting the blocks in the line of a shape
            else {
                seen_blocks += line.chars().filter(|&char| char == '#').count();
            }
        } else {
            let (area_part, shapes_required_part) = line.split_once(": ").unwrap();
            let (width, height) = area_part.split_once('x').unwrap();
            let width = width.parse().unwrap();
            let height = height.parse().unwrap();
            let shapes_required = shapes_required_part.split(' ')
                .map(|n| n.parse().unwrap()).collect();

            let problem = Problem {
                shapes_required,
                width,
                height
            };

            problems.push(problem);
        }

        line_index += 1;
    }

    Parsed {
        problems,
        shapes,
    }
}

fn is_possible(shapes: &[usize], problem: &Problem) -> bool {
    let total_blocks: usize = problem.shapes_required.iter().enumerate()
        .map(|(index, shape_required)| {
            shapes[index] * shape_required
        })
        .sum();

    total_blocks <= problem.width * problem.height
}

fn part1(parsed: &Parsed) -> usize {
    println!("{:?}", parsed);

    parsed.problems.iter()
        .filter(|problem| is_possible(&parsed.shapes, problem))
        .count()
}
