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

    let mut steps = 0;

    let mut paths = vec![(0, vec![]); starting_nodes.len()];

    for (i, node) in starting_nodes.iter_mut().enumerate() {
        let mut first = false;
        for dir in dirs.iter().cycle() {
            if node.ends_with("Z") {
                if !first {
                    paths[i].1.push(*node);
                    first = true;
                } else {
                    break;
                }
            }
            if !first {
                paths[i].1.push(*node);
            } else {
                paths[i].0 += 1;
            }
            let next_node = map.get(*node).and_then(|v| v.get(*dir)).unwrap();
            *node = next_node;
        }
    }
    let mut steps = paths.iter().map(|(_, p)| p.len()).min().unwrap();
    let mut step_size = paths.iter().map(|x| x.0).min().unwrap();

 //   println!("{paths:?}");
    let mut nodes = vec![""; paths.len()];
    dbg!(steps);
    dbg!(step_size);
    println!();

    // dbg!((steps+step_size-1));
    for step in ((steps - 2)..).step_by(step_size) {
        for (i, (loop_size, path)) in paths.iter().enumerate() {
            let looped_path = &path[(path.len() - loop_size)..];
//            println!("{looped_path:?}");
            let index = step % (looped_path.len());
            let next = looped_path[index];
            //dbg!(step);
            //dbg!(loop_size);
            //dbg!(index);
            //dbg!(next);
            nodes[i] = looped_path[index];

            //println!("-----");
        }

        steps += (step_size - 1);
        if nodes.iter().all(|n| n.ends_with("Z")) {
            break;
        }
    }
    steps
}
