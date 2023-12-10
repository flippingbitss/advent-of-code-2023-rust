mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let answer_one = day07::part_one(include_str!("../inputs/day07.in"));
    println!("{}", answer_one);
        let answer_two = day07::part_two(include_str!("../inputs/day07.in"));
    println!("{}", answer_two);
}
