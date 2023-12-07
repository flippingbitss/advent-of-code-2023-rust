use std::collections::HashMap;

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn parse_num(&self, row: usize, start: usize, end: usize) -> usize {
        self.0[row][start..end]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.0[0].len() as isize && y >= 0 && y < self.0.len() as isize
    }

    fn size(&self) -> (usize, usize) {
        (self.0[0].len(), self.0.len())
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        if self.in_bounds(x, y) {
            return Some(self.0[y as usize][x as usize]);
        }
        None
    }
}

fn parse_grid(input: &str) -> Grid {
    Grid(input.lines().map(|l| l.chars().collect()).collect())
}

const DIRS: [(isize, isize); 8] = [
    (1, 0),
    (0, 1),
    (1, 1),
    (-1, -1),
    (-1, 0),
    (0, -1),
    (1, -1),
    (-1, 1),
];

type GearPos = (usize, usize);
type PartNum = usize;

fn find_attached_numbers(input: &str) -> Vec<(PartNum, GearPos)> {
    let grid = parse_grid(input);

    let (width, height) = grid.size();

    let mut curr_low = 0;
    let mut curr_high = 0;
    let mut track = false;
    let mut attached = false;
    let mut gear_pos = (0, 0);
    let mut results = Vec::new();
    for r in 0..height {
        for c in 0..width {
            let ch = grid.get(c as isize, r as isize).unwrap();
            if ch.is_digit(10) {
                if !track {
                    curr_low = c;
                    curr_high = c + 1;
                    track = true;
                } else {
                    curr_high += 1;
                }
                for dir in DIRS {
                    let cn = c as isize + dir.0;
                    let rn = r as isize + dir.1;
                    if let Some(nh) = grid.get(cn, rn) {
                        if nh != '.' && !nh.is_digit(10) {
                            gear_pos = (cn as usize, rn as usize);
                            attached = true;
                        }
                    }
                }
                if c == width - 1 {
                    if attached {
                        let num = grid.parse_num(r, curr_low, curr_high);
                        results.push((num, gear_pos));
                        attached = false;
                    }
                    track = false;
                }
            } else {
                if attached {
                    let num = grid.parse_num(r, curr_low, curr_high);
                    results.push((num, gear_pos));
                    attached = false;
                }
                track = false;
            }
        }
    }
    results
}

pub fn part_one(input: &str) -> usize {
    let attachments = find_attached_numbers(input);
    attachments.iter().map(|x| x.0).sum()
}

pub fn part_two(input: &str) -> usize {
    let attachments = find_attached_numbers(input);

    let mut gears_to_parts = attachments
        .iter()
        .map(|x| (x.1, Vec::new()))
        .collect::<HashMap<_, _>>();

    attachments.into_iter().for_each(|(pn, gear_pos)| {
        gears_to_parts.entry(gear_pos).or_default().push(pn);
    });

    gears_to_parts
        .values()
        .filter(|pns| pns.len() == 2)
        .map(|pns| pns.iter().product::<usize>())
        .sum()
}
