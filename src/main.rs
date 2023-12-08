mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let answer_one = day04::part_one(include_str!("../inputs/day04.in"));
    let answer_two = day04::part_two(include_str!("../inputs/day04.in"));
    println!("{}", answer_one);
    println!("{}", answer_two);
}
