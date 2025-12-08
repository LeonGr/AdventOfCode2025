use std::collections::{HashMap, HashSet};

type Coord = (i64, i64, i64);

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

fn parse(lines: &[String]) -> HashSet<Coord> {
    lines.iter()
        .map(|line| {
            let split: Vec<&str> = line.split(',').collect();
            (split[0].parse().unwrap(), split[1].parse().unwrap(), split[2].parse().unwrap())
        })
        .collect()
}

fn get_distance(point_a: &Coord, point_b: &Coord) -> f64 {
    let x =
        (point_a.0 - point_b.0).pow(2) +
        (point_a.1 - point_b.1).pow(2) +
        (point_a.2 - point_b.2).pow(2);

    let y = x  as f64;

    (y).sqrt()
}

fn are_connected(point_a: &Coord, point_b: &Coord, connections: &HashMap<Coord, HashSet<Coord>>) -> bool {
    if let Some(connected_to_a) = connections.get(point_a) {
        connected_to_a.contains(point_b)
    } else {
        false
    }
}

fn get_circuit(key: &Coord, connections: &HashMap<Coord, HashSet<Coord>>) -> HashSet<Coord> {
    let mut stack = vec![key];
    let mut circuit = HashSet::from([*key]);

    while let Some(node) = stack.pop() {
        if let Some(connected) = connections.get(node) {
            for c in connected {
                if !circuit.contains(c) {
                    stack.push(c);
                }

                circuit.insert(*c);
            }
        }
    }

    circuit
}

fn find_circuits(connections: &mut HashMap<Coord, HashSet<Coord>>) -> Vec<HashSet<Coord>> {
    let keys: Vec<Coord> = connections.keys().cloned().collect();
    let mut seen = HashSet::new();

    let mut circuits = vec![];

    for key in keys {
        if seen.contains(&key) {
            continue;
        }

        let circuit = get_circuit(&key, connections);

        for c in circuit.clone() {
            connections.remove(&c);
            seen.insert(c);
        }

        circuits.push(circuit);
    }

    circuits
}

fn part1(points: &HashSet<Coord>) -> usize {
    // let max_connections = 10;
    let max_connections = 1000;

    let mut distances = vec![];
    for point_a in points {
        for point_b in points {
            if point_a == point_b {
                continue;
            }

            distances.push((*point_a, *point_b, get_distance(point_a, point_b)));
        }
    }

    distances.sort_by(|(_, _, d1), (_, _, d2)| {
        d2.total_cmp(d1)
    });

    let mut connection_count = 0;
    let mut connections = HashMap::new();

    while connection_count < max_connections {
        if let Some((point_a, point_b, _)) = distances.pop() {
            if are_connected(&point_a, &point_b, &connections) {
                continue;
            }
            connections
                .entry(point_a)
                .and_modify(|connected_to_a| { connected_to_a.insert(point_b); })
                .or_insert_with(|| HashSet::from([point_b]));

            connections
                .entry(point_b)
                .and_modify(|connected_to_b| { connected_to_b.insert(point_a); })
                .or_insert_with(|| HashSet::from([point_a]));

            connection_count += 1;
        } else {
            break;
        }
    }

    let mut circuits = find_circuits(&mut connections);

    circuits.sort_by(|c1, c2| {
        (c2.len()).cmp(&c1.len())
    });

    circuits.iter().take(3).map(|circuit| circuit.len()).product()
}

fn part2(points: &HashSet<Coord>) -> i64 {
    let mut connections = HashMap::new();

    let mut distances = vec![];
    for point_a in points {
        for point_b in points {
            if point_a == point_b {
                continue;
            }

            distances.push((*point_a, *point_b, get_distance(point_a, point_b)));
        }
    }

    distances.sort_by(|(_, _, d1), (_, _, d2)| {
        d2.total_cmp(d1)
    });

    while let Some((point_a, point_b, _)) = distances.pop() {
        if are_connected(&point_a, &point_b, &connections) {
            continue;
        }

        connections
            .entry(point_a)
            .and_modify(|connected_to_a| { connected_to_a.insert(point_b); })
            .or_insert_with(|| HashSet::from([point_b]));

        connections
            .entry(point_b)
            .and_modify(|connected_to_b| { connected_to_b.insert(point_a); })
            .or_insert_with(|| HashSet::from([point_a]));

        let circuit = get_circuit(&point_a, &connections);
        if circuit.len() == points.len() {
            return point_a.0 * point_b.0;
        };
    }

    unreachable!()
}
