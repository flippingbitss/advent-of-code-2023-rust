type Game = (usize, Vec<Set>);

#[derive(Default, PartialEq)]
struct Set([usize; 3]);

impl Set {
    fn rgb(r: usize, g: usize, b: usize) -> Self {
        Self([r, g, b])
    }

    fn iter(&self) -> impl Iterator<Item = usize> {
        self.0.into_iter()
    }
}

impl std::ops::Add for Set {
    type Output = Set;

    fn add(self, rhs: Self) -> Self::Output {
        let [r, g, b] = self.0;
        let [or, og, ob] = rhs.0;
        Set([r + or, g + og, b + ob])
    }
}

impl FromIterator<usize> for Set {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let r = iter.next().unwrap_or(0);
        let g = iter.next().unwrap_or(0);
        let b = iter.next().unwrap_or(0);
        Set::rgb(r, g, b)
    }
}

fn parse_line(s: &str) -> Game {
    let (game_prefix, rem) = s.split_once(": ").unwrap();
    let game_id = (game_prefix[5..]).parse::<usize>().unwrap();
    let mut sets = Vec::new();
    for set in rem.split("; ") {
        let cubes = set.split(", ");
        let mut set = Set::default();
        for c in cubes {
            let (count, color) = c.split_once(' ').unwrap();
            let count = count.parse::<usize>().unwrap();
            let index = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => unreachable!(),
            };
            set.0[index] = count;
        }
        sets.push(set);
    }
    (game_id, sets)
}

pub fn part_one(input: &str) -> usize {
    let max: Set = Set::rgb(12, 13, 14);
    input
        .lines()
        .map(parse_line)
        .filter(|(_, sets)| {
            sets.iter()
                .all(|set| set.iter().zip(max.iter()).all(|(a, b)| a <= b))
        })
        .map(|v| v.0)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .map(|(_, sets)| {
            sets.iter().fold(Set::default(), |acc, s| {
                Set::from_iter(acc.iter().zip(s.iter()).map(|(a, b)| a.max(b)))
            })
        })
        .map(|s| s.iter().product::<usize>())
        .sum()
}
