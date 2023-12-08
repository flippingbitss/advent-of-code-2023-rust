use std::collections::HashSet;

type CardSet = HashSet<usize>;

fn parse_card(input: &str) -> (CardSet, CardSet) {
    let (_, all_nums) = input.split_once(':').unwrap();
    let (winning, holding) = all_nums.split_once('|').unwrap();
    let winning = winning
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let holding = holding
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    (winning, holding)
}

fn matches(w: &CardSet, h: &CardSet) -> usize {
    w.intersection(h).count()
}

fn evaluate_points(matches: usize) -> usize {
    if matches == 0 {
        0
    } else {
        1 << (matches - 1)
    }
}

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(parse_card)
        .map(|(w, h)| matches(&w, &h))
        .map(evaluate_points)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let cards = input
        .lines()
        .map(parse_card)
        .map(|(w, h)| matches(&w, &h))
        .collect::<Vec<usize>>();

    let mut total = 0;
    let mut queue = Vec::from_iter(0..cards.len());

    while let Some(card_id) = queue.pop() {
        total += 1;
        let matches = &cards[card_id];
        queue.extend((card_id + 1)..(card_id + 1 + *matches));
    }

    total
}
