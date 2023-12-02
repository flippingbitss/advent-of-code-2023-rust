use std::char;

const DIGITS_IN_WORDS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|l| (find_digit(l.chars()), find_digit(l.chars().rev())))
        .map(|(l, r)| l * 10 + r)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    input.lines()
        .map(find_word_or_digit)
        .map(|v| (*v.first().unwrap(), *v.last().unwrap()))
        .map(|(l, r)| l * 10 + r)
        .sum()
}

fn find_digit<I: Iterator<Item = char>>(input: I) -> usize {
    input.into_iter().find_map(|c| c.to_digit(10)).unwrap() as usize
}

fn find_word_or_digit(input: &str) -> Vec<usize> {
    let mut output = Vec::new();
    if input.is_empty() {
        return output;
    }
    if let Some(c) = input.chars().next().and_then(|c| c.to_digit(10)) {
        output.push(c as usize);
        output.append(&mut find_word_or_digit(&input[1..]))
    } else {
        for (i, &word) in DIGITS_IN_WORDS.iter().enumerate() {
            if input.starts_with(word) {
                output.push(i + 1);
                output.append(&mut find_word_or_digit(&input[(word.len() - 1)..]));
                return output;
            }
        }
        output.append(&mut find_word_or_digit(&input[1..]))
    }
    output
}
