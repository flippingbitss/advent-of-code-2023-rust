mod day01;
mod day02;

fn main() {
    let answer_one = day02::part_one(include_str!("../inputs/day02.in"));
    let answer_two = day02::part_two(include_str!("../inputs/day02.in"));
    println!("{}", answer_one);
    println!("{}", answer_two);
}
