mod day01;
mod day02;
mod day03;

fn main() {
    let answer_one = day03::part_one(include_str!("../inputs/day03.in"));
    //let answer_two = day03::part_two(include_str!("../inputs/day02.in"));
    println!("{}", answer_one);
    //println!("{}", answer_two);
}
