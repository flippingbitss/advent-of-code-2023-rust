use itertools::Itertools;

const DECK_SIZE: usize = 13;
const HAND_SIZE: usize = 5;
const DECK_REGULAR: [char; DECK_SIZE] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const DECK_WEAK_JOKER: [char; DECK_SIZE] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

type CardSet = [char; HAND_SIZE];
type Deck = [char; DECK_SIZE];

#[derive(Debug)]
struct Hand {
    cards: CardSet,
    bid: usize,
    strength: usize,
}

impl Hand {
    fn score_regular(&self) -> (Category, usize) {
        let mut counts = [0_usize; DECK_SIZE];
        for c in self.cards {
            counts[card_value(c, &DECK_REGULAR)] += 1;
        }
        (categorize(&counts), self.strength)
    }

    fn score_with_joker(&self) -> (Category, usize) {
        let mut counts = [0_usize; DECK_SIZE];

        for c in self.cards {
            counts[card_value(c, &DECK_WEAK_JOKER)] += 1;
        }
        let joker_pos = card_value('J', &DECK_WEAK_JOKER);
        let jokers = counts[joker_pos];

        if jokers > 0 {
            if let Some(higher) = counts
                .iter()
                .enumerate()
                .filter(|(i, _)| i != &joker_pos)
                .max_by_key(|(_, &x)| x)
                .map(|(i, _)| i)
            {
                counts[higher] += jokers;
                counts[joker_pos] = 0;
            }
        }

        (categorize(&counts), self.strength)
    }
}

fn card_value(c: char, deck: &Deck) -> usize {
    deck.iter().position(|x| *x == c).unwrap()
}

fn categorize(counts: &[usize]) -> Category {
    let non_zero_counts = counts
        .into_iter()
        .cloned()
        .filter(|&x| x > 0)
        .sorted()
        .collect::<Vec<usize>>();
    match non_zero_counts[..] {
        [5] => Category::Five,
        [1, 4] => Category::Four,
        [2, 3] => Category::FullHouse,
        [1, 1, 3] => Category::Three,
        [1, 2, 2] => Category::TwoPair,
        [1, 1, 1, 2] => Category::OnePair,
        [1, 1, 1, 1, 1] => Category::High,
        _ => unreachable!(),
    }
}

fn parse_input(input: &str, deck: &Deck) -> Vec<Hand> {
    input
        .lines()
        .map(|l| {
            let (card_set, bid) = l.split_once(' ').unwrap();
            let mut cards = ['0'; HAND_SIZE];
            let mut strength = Vec::new();
            card_set.char_indices().for_each(|(i, c)| {
                cards[i] = c;
                let card_strength = card_value(c, deck) + (10 * (HAND_SIZE - i));
                strength.push(card_strength.to_string());
            });
            let bid = bid.parse::<usize>().unwrap();
            let strength = strength.into_iter().collect::<String>().parse().unwrap();
            Hand {
                cards,
                bid,
                strength,
            }
        })
        .collect()
}

#[derive(PartialOrd, PartialEq, Debug)]
enum Category {
    High,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

pub fn part_one(input: &str) -> usize {
    let hands = parse_input(input, &DECK_REGULAR);
    hands
        .iter()
        .sorted_by(|a, b| a.score_regular().partial_cmp(&b.score_regular()).unwrap())
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let hands = parse_input(input, &DECK_WEAK_JOKER);
    hands
        .iter()
        .sorted_by(|a, b| {
            a.score_with_joker()
                .partial_cmp(&b.score_with_joker())
                .unwrap()
        })
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum()
}
