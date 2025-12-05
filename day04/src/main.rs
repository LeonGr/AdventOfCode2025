use std::collections::HashSet;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}


#[derive(Clone)]
struct Map {
    paper: HashSet<Coord>,
    max_x: i32,
    max_y: i32,
}

type Coord = (i32, i32);

fn parse(lines: &[String]) -> Map {
    let mut map = HashSet::new();

    let max_y = (lines.len() - 1) as i32;
    let max_x = (lines.first().unwrap().len() - 1) as i32;

    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '@' {
                map.insert((col as i32, row as i32));
            }
        }
    }

    Map {
        paper: map,
        max_x,
        max_y,
    }
}

fn get_neighbours(map: &Map, (x, y): &Coord) -> Vec<Coord> {
    let mut neighbours = vec![];

    let displacements = vec![ (0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, -1), (-1, 1), (1, -1) ];
    for (dx, dy) in displacements {
        let (nx, ny) = (x + dx, y + dy);

        if nx < 0 || ny < 0 || nx > map.max_x || ny > map.max_y {
            continue;
        }

        if map.paper.contains(&(nx, ny)) {
            neighbours.push((nx, ny));
        }
    }

    neighbours
}


fn part1(map: &Map) -> usize {
    let fewer_than_four_neighbours: Vec<_> =
        map.paper.iter()
            .filter(|pos| {
                get_neighbours(map, pos).len() < 4
            }).collect();

    // println!("{:?}", fewer_than_four_neighbours);

    fewer_than_four_neighbours.len()
}

fn remove_available_rolls(map: &mut Map) -> bool {
    let map_copy = map.clone();
    map.paper.retain(|pos| {
        get_neighbours(&map_copy, pos).len() >= 4
    });

    map_copy.paper.len() != map.paper.len()
}

fn part2(map: &Map) -> usize {
    let mut map_copy = map.clone();

    loop {
        if !remove_available_rolls(&mut map_copy) {
            break;
        }
    }

    map.paper.len() - map_copy.paper.len()
    // println!("{:?}", fewer_than_four_neighbours);
}

fn main() {
    let lines = read_input();
    let map = parse(&lines);

    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

