advent_of_code::solution!(9);
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

pub fn part_one(input: &str) -> Option<u64> {
    let mut points: Vec<(u64, u64)> = Vec::new();
    let mut result: u64 = 0;
    for line in input.lines() {
        let t = line.split_once(",")?;
        let Ok(c1) = u64::from_str_radix(t.0, 10) else {
            todo!()
        };
        let Ok(c2) = u64::from_str_radix(t.1, 10) else {
            todo!()
        };

        let points_iter = points.iter();
        for p in points_iter {
            result = max(result, (p.0.abs_diff(c1) + 1) * (p.1.abs_diff(c2) + 1));
        }

        points.push((c1, c2));
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut points: Vec<(u64, u64)> = Vec::new();
    for line in input.lines() {
        let t = line.split_once(",")?;
        let Ok(c1) = u64::from_str_radix(t.0, 10) else {
            todo!()
        };
        let Ok(c2) = u64::from_str_radix(t.1, 10) else {
            todo!()
        };

        points.push((c1, c2));
    }
    let mut curr = points.last()?;
    let mut m: HashSet<(u64, u64)> = HashSet::new();
    for p in points.iter() {
        if curr.0 == p.0 {
            for i in curr.1..p.1 {
                m.insert((curr.0, i));
            }
        } else {
            for i in curr.0..p.0 {
                m.insert((i, curr.1));
            }
        }
    }

    //note to self, multiply graph by 2

    for (r, line) in input
        .lines()
        .enumerate()
        .map(|(size, str)| (size as u64, str))
    {
        let mut inside = false;
        for c in 0..line.len() as u64 {
            if m.contains(&(r, c)) && !m.contains(&(r, c - 1)) {
                inside = !inside;
            }
            if inside {}
        }
    }
    let mut result: u64 = 0;
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
