mod day01;

fn main() {
    let answer_one = day01::part_one(include_str!("../inputs/day01.in"));
    let answer_two = day01::part_two(include_str!("../inputs/day01.in"));
    println!("{}", answer_one);
    println!("{}", answer_two);
}
