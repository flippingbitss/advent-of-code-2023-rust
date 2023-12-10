fn parse_entries(input: &str) -> Vec<&str> {
    input.split_whitespace().skip(1).collect()
}

fn parse_num(input: &str) -> usize {
    input.parse().unwrap()
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let times = parse_entries(lines.next().unwrap());
    let distances = parse_entries(lines.next().unwrap());
    (times, distances)
}

fn ways_to_beat(time: usize, dist: usize) -> usize {
    (1..time)
        .map(|t| t * (time - t))
        .filter(|d| d > &dist)
        .count()
}

pub fn part_one(input: &str) -> usize {
    let (times, distances) = parse_input(input);
    times
        .into_iter()
        .map(parse_num)
        .zip(distances.into_iter().map(parse_num))
        .map(|(time, dist)| ways_to_beat(time, dist))
        .product()
}

pub fn part_two(input: &str) -> usize {
    let (upper, lower) = parse_input(input);
    let time: usize = upper.into_iter().collect::<String>().parse().unwrap();
    let dist: usize = lower.into_iter().collect::<String>().parse().unwrap();
    ways_to_beat(time, dist)
}
