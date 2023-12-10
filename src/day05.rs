use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }

    fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

struct Mapping(Vec<(Range, Range)>);

impl Mapping {
    fn get(&self, v: usize) -> usize {
        self.0
            .iter()
            .find(|(sr, _)| sr.start <= v && sr.end > v)
            .map(|(sr, dr)| v - sr.start + dr.start)
            .unwrap_or(v)
    }

    fn trimmed_ranges(&self, bound: Range) -> impl Iterator<Item = (Range, Range)> + '_ {
        self.0
            .iter()
            .map(move |(src, dest)| {
                Self::trim_range(&bound, src).map(|trim_src| {
                    let mut dest = *dest;
                    if bound.start > src.start {
                        let start_delta = bound.start - src.start;
                        dest.start = dest.start + start_delta;
                    }
                    if bound.end < src.end {
                        let end_delta = src.end - bound.end;
                        dest.end = dest.end - end_delta;
                    }
                    (trim_src, dest)
                })
            })
            .filter_map(|v| v)
    }

    fn trim_range(bound: &Range, input: &Range) -> Option<Range> {
        let ostart = usize::max(bound.start, input.start);
        let oend = usize::min(bound.end, input.end);

        let overlapping = ostart < oend;
        if !overlapping {
            None
        } else {
            Some(Range::new(ostart, oend))
        }
    }
}

fn parse_mapping(input: &str) -> Mapping {
    Mapping(
        input
            .lines()
            .skip(1)
            .map(|s| {
                let nums = s
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                match nums[..] {
                    [drs, srs, len] => (Range::new(srs, srs + len), Range::new(drs, drs + len)),
                    _ => unreachable!(),
                }
            })
            .collect(),
    )
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Mapping>) {
    let mut groups = input.split("\n\n");
    let seeds = groups.next().unwrap();
    let seeds = seeds
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mappings = groups.map(parse_mapping).collect();

    (seeds, mappings)
}

pub fn part_one(input: &str) -> usize {
    let (seeds, mappings) = parse_input(input);
    seeds
        .iter()
        .map(|&s| mappings.iter().fold(s, |v, mp| mp.get(v)))
        .min()
        .unwrap()
}

pub fn part_two(input: &str) -> usize {
    part_two_with_sinking_ranges(input)
}

pub fn part_two_brute(input: &str) -> usize {
    let (seeds, mappings) = parse_input(input);
    seeds
        .into_iter()
        .tuples::<(_, _)>()
        .flat_map(|(s, r)| (s..(s + r)).into_iter())
        .map(|s| mappings.iter().fold(s, |v, mp| mp.get(v)))
        .min()
        .unwrap()
}

pub fn part_two_with_sinking_ranges(input: &str) -> usize {
    let (seeds, mappings) = parse_input(input);
    let mut seed_ranges = seeds
        .into_iter()
        .tuples::<(_, _)>()
        .map(|(s, l)| Range::new(s, s + l))
        .collect::<Vec<_>>();

    for mapping in mappings.iter() {
        let mut mapped = Vec::new();
        while let Some(seed_range) = seed_ranges.pop() {
            let mut overlapped = false;
            for (tsrc, tdest) in mapping.trimmed_ranges(seed_range) {
                let head = Range::new(seed_range.start, tsrc.start);
                let tail = Range::new(tsrc.end, seed_range.end);
                if !head.is_empty() {
                    seed_ranges.push(head);
                }
                if !tail.is_empty() {
                    seed_ranges.push(tail);
                }
                if !tdest.is_empty() {
                    mapped.push(tdest);
                    overlapped = true;
                    break;
                }
            }
            if !overlapped {
                mapped.push(seed_range);
            }
        }
        seed_ranges = mapped;
    }

    seed_ranges.iter().min_by_key(|r| r.start).unwrap().start
}
