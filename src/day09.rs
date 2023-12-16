use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|d| d.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let mut total = 0; 
    for nums in parse_input(input) {
        let mut curr = nums.clone(); 
        loop {
            total += curr.last().unwrap().clone();
            let next = curr.iter()
                    .tuple_windows::<(_, _)>()
                    .map(|(a, b)| b - a)
                    .collect::<Vec<_>>(); 
            if next.iter().all(|&x| x == 0_isize) {
                break;
            }
            curr = next;
        }
    }
    total as usize
}

pub fn part_two(input: &str) -> usize {
    let mut total = 0; 
    for nums in parse_input(input) {
        let mut curr = nums.clone(); 
        for i in 0.. {
            let next = curr.iter()
                    .tuple_windows::<(_, _)>()
                    .map(|(a, b)| b - a)
                    .collect::<Vec<_>>(); 
            let curr_head = curr.first().unwrap().clone();
            if i % 2 == 0 {
                total += curr_head;
            } else {
                total -= curr_head;
            }
            if next.iter().all(|&x| x == 0_isize) {
                break;
            } 
            curr = next;
        }
    }
    total as usize
}
