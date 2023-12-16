mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {
    let answer_one = day09::part_one(include_str!("../inputs/day09.in"));
    println!("{}", answer_one);
    let answer_two = day09::part_two(include_str!("../inputs/day09.in"));
    println!("{}", answer_two);
}
