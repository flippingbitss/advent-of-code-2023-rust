use std::collections::HashMap;

const TARGET: &'static str = "ZZZ";

fn parse_input(input: &str) -> (Vec<usize>, HashMap<&str, [&str; 2]>) {
    let (dirs, nodes) = input.split_once("\n\n").unwrap();
    let dirs = dirs
        .chars()
        .map(|c| match c {
            'R' => 1,
            'L' => 0,
            _ => unreachable!(),
        })
        .collect();
    let mut map = HashMap::new();

    nodes.lines().for_each(|l| {
        let (key, values) = l.split_once(" = ").unwrap();
        let (left, right) = values[1..(values.len() - 1)].split_once(", ").unwrap();
        map.insert(key, [left, right]);
    });

    (dirs, map)
}

pub fn part_one(input: &str) -> usize {
    let (dirs, map) = parse_input(input);
    let mut curr_node = "AAA";
    let mut steps = 0;
    for dir in dirs.iter().cycle() {
        if curr_node == TARGET {
            break;
        }
        curr_node = map.get(curr_node).and_then(|v| v.get(*dir)).unwrap();
        steps += 1;
    }
    steps
}

pub fn part_two(input: &str) -> usize {
    let (dirs, map) = parse_input(input);

    let mut starting_nodes = map
        .keys()
        .filter(|k| k.chars().last().unwrap() == 'A')
        .collect::<Vec<_>>();

    let mut loop_sizes = vec![0; starting_nodes.len()];
    let mut paths = vec![vec![]; starting_nodes.len()];

    for (i, node) in starting_nodes.iter_mut().enumerate() {
        let mut first = false;
        for dir in dirs.iter().cycle() {
            if node.ends_with("Z") {
                if !first {
                    paths[i].push(*node);
                    first = true;
                } else {
                    break;
                }
            }
            if !first {
                paths[i].push(*node);
            } else {
                loop_sizes[i] += 1;
            }
            let next_node = map.get(*node).and_then(|v| v.get(*dir)).unwrap();
            *node = next_node;
        }
    }

    let min_loop = loop_sizes.iter().min().unwrap();
    let min_path_len = paths.iter().map(|p| p.len()).min().unwrap();
    let lcm_total = loop_sizes.iter().fold(1, |acc, &x| lcm(acc, x));
    
    lcm_total - (min_path_len - min_loop - 1)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    // lcm(a,b) * gcd(a,b) = a * b;
    (a * b) / gcd(a, b)
}
