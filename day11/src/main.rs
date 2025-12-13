use std::{collections::{HashMap, HashSet, VecDeque}, ops::{AddAssign, SubAssign}};

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

fn parse(lines: &[String]) -> HashMap<String, HashSet<String>> {
    let mut devices: HashMap<String, HashSet<String>>  = lines.iter().map(|line| {
        let (device, attached) = line.split_once(": ").unwrap();
        let attached = attached.split(' ').map(|a| a.to_string()).collect();

        (device.to_string(), attached)
    })
    .collect();

    devices.insert(String::from("out"), HashSet::new());

    devices
}

fn part1(devices: &HashMap<String, HashSet<String>>) -> usize {
    count_paths(&String::from("you"), &String::from("out"), devices)
}

// Use topological sorting to find the number of paths from `start` to `end`
fn count_paths(start: &String, end: &String, devices: &HashMap<String, HashSet<String>>) -> usize {
    let mut indegree: HashMap<&String, usize> = HashMap::new();

    for (device, connected_to) in devices {
        indegree.entry(device).or_insert(0);

        for connection in connected_to {
            indegree.entry(connection)
                .and_modify(|count| {
                    count.add_assign(1);
                })
                .or_insert(1);
        }
    }

    let mut q = VecDeque::new();
    for (&device, count) in &indegree {
        if *count == 0 {
            q.push_back(device);
        }
    }

    let mut topological_order = vec![];
    while let Some(head) = q.pop_front() {
        topological_order.push(head);

        if let Some(connected) = devices.get(head) {
            for connection in connected {
                indegree.entry(connection).and_modify(|count| count.sub_assign(1));
                if *indegree.get(connection).unwrap() == 0 {
                    q.push_back(connection);
                }
            }
        }
    }

    let mut ways: HashMap<&String, usize> = HashMap::new();

    for device in devices.keys() {
        ways.entry(device).or_insert(0);
    }

    ways.entry(start).and_modify(|x| x.add_assign(1));

    for device in topological_order {
        if let Some(connected) = devices.get(device) {
            for connection in connected {
                let x = *ways.get(device).unwrap();
                ways.entry(connection).and_modify(|count| count.add_assign(x));
            }
        }
    }

    *ways.get(&end).unwrap()
}

fn part2(devices: &HashMap<String, HashSet<String>>) -> usize {
    let svr = String::from("svr");
    let fft = String::from("fft");
    let dac = String::from("dac");
    let out = String::from("out");

    let svr_to_ftt = count_paths(&svr, &fft, devices);
    let fft_to_dac = count_paths(&fft, &dac, devices);
    let dac_to_out = count_paths(&dac, &out, devices);

    println!("svr to fft: {}", svr_to_ftt);
    println!("fft to dac: {}", fft_to_dac);
    println!("dac to out: {}", dac_to_out);

    svr_to_ftt * fft_to_dac * dac_to_out
}
