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

pub fn part_one(input: &str) -> usize {
    let grid = parse_grid(input);

    let (width, height) = grid.size();

    let mut curr_low = 0;
    let mut curr_high = 0;
    let mut track = false;
    let mut attached = false;
    let mut total = 0;

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
                            attached = true;
                        }
                    }
                }
                if c == width - 1 {
                    if attached {
                        total += grid.parse_num(r, curr_low, curr_high);
                        attached = false;
                    }
                    track = false;
                }
            } else {
                if attached {
                    total += grid.parse_num(r, curr_low, curr_high);
                    attached = false;
                }
                track = false;
            }
        }
    }
    total
}
