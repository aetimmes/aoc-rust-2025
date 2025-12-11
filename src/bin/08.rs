use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use std::cmp::Ordering;

use itertools::Itertools;
use std::iter::zip;

advent_of_code::solution!(8);

fn dist(p: &Segment) -> u64 {
    zip(p.points[0].coords, p.points[1].coords)
        .map(|(i, j)| (i.abs_diff(j)).pow(2))
        .sum::<u64>()
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
struct Point<T = u64> {
    coords: [T; 3],
}

struct Segment<T = u64> {
    points: [Point<T>; 2],
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}

impl Eq for Segment {}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        dist(&self).cmp(&dist(other))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_inner(input, Some(1000))
}

fn parse(input: &str) -> Option<(HashSet<Point>, Vec<Segment>)> {
    let mut points: HashSet<Point> = HashSet::new();
    let mut segments: Vec<Segment> = Vec::new();
    for line in input.lines() {
        let p1: Point = Point {
            coords: line
                .split(",")
                .map(|x| u64::from_str_radix(x, 10).unwrap())
                .collect_array()?,
        };

        for p2 in &points {
            segments.push(Segment { points: [p1, *p2] });
        }
        points.insert(p1);
    }

    segments.sort();
    Some((points, segments))
}

pub fn part_one_inner(input: &str, n: Option<u64>) -> Option<u64> {
    let iters: u64 = match n {
        Some(inner) => inner,
        None => 1000,
    };

    let (_, segments) = parse(input)?;

    let mut g: HashMap<Point, u64> = HashMap::new();
    let mut i: u64 = 0;
    let mut segments_iter = segments.iter();
    for _ in 0..iters {
        let s = segments_iter.next()?;
        let p1 = s.points[0];
        let p2 = s.points[1];
        if g.contains_key(&p1) && g.contains_key(&p2) {
            if g[&p1] != g[&p2] {
                let old = g[&p2];
                for (k, v) in g.clone() {
                    if v == old {
                        g.insert(k, g[&p1]);
                    }
                }
            }
        } else if !g.contains_key(&p1) && !g.contains_key(&p2) {
            g.insert(p1, i);
            g.insert(p2, i);
            i += 1;
        } else if g.contains_key(&p1) {
            g.insert(p2, g[&p1]);
        } else if g.contains_key(&p2) {
            g.insert(p1, g[&p2]);
        }
    }
    let mut counter: HashMap<u64, u64> = HashMap::new();
    for (_, v) in g {
        *counter.entry(v).or_insert(0) += 1;
    }
    Some(
        counter
            .values()
            .sorted()
            .rev()
            .take(3)
            .collect::<Vec<_>>()
            .into_iter()
            .fold(1, |acc, x| acc * x),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (points, segments) = parse(input)?;

    let mut g: HashMap<Point, u64> = HashMap::new();
    let mut i: u64 = 0;
    let mut seen: HashSet<Point> = HashSet::new();
    for s in segments {
        let p1 = s.points[0];
        let p2 = s.points[1];
        if g.contains_key(&p1) && g.contains_key(&p2) {
            if g[&p1] != g[&p2] {
                let old = g[&p2];
                for (k, v) in g.clone() {
                    if v == old {
                        g.insert(k, g[&p1]);
                    }
                }
            }
        } else if !g.contains_key(&p1) && !g.contains_key(&p2) {
            g.insert(p1, i);
            g.insert(p2, i);
            i += 1;
        } else if g.contains_key(&p1) {
            g.insert(p2, g[&p1]);
        } else if g.contains_key(&p2) {
            g.insert(p1, g[&p2]);
        }
        seen.insert(p1);
        seen.insert(p2);
        if seen == points {
            if g.values().tuple_windows().all(|w: (&u64, &u64)| w.0 == w.1) {
                return Some(p1.coords[0] * p2.coords[0]);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dist() {
        let result = dist(&Segment::<u64> {
            points: [Point { coords: [1, 2, 3] }, Point { coords: [4, 6, 8] }],
        });
        assert_eq!(result, 3 * 3 + 4 * 4 + 5 * 5);
    }

    #[test]
    fn test_part_one() {
        let result = part_one_inner(
            &advent_of_code::template::read_file("examples", DAY),
            Some(10),
        );
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
