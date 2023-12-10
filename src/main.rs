mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let answer_one = day06::part_one(include_str!("../inputs/day06.in"));
    let answer_two = day06::part_two(include_str!("../inputs/day06.in"));
    println!("{}", answer_one);
    println!("{}", answer_two);
}
