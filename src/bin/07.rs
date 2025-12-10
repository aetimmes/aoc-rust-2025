use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut splitters: HashSet<(u64, u64)> = HashSet::new();
    let mut start: (u64, u64) = (0, 0);
    for (r, line) in input.lines().enumerate() {
        let line_bytes = line.as_bytes();
        for c in 0..line_bytes.len() {
            match line_bytes[c] {
                b'S' => {
                    start = (r as u64, c as u64);
                }
                b'^' => {
                    splitters.insert((r as u64, c as u64));
                }
                _ => {}
            }
        }
    }
    let mut current: HashSet<(u64, u64)> = HashSet::new();
    current.insert(start);
    let mut result: u64 = 0;
    for _ in input.lines() {
        let mut nextset: HashSet<(u64, u64)> = HashSet::new();
        for (r, c) in current.iter() {
            if splitters.contains(&(r + 1, *c)) {
                result += 1;
                nextset.insert((r + 1, c + 1));
                nextset.insert((r + 1, c - 1));
            } else {
                nextset.insert((r + 1, *c));
            }
        }
        current = nextset;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut splitters: HashSet<(u64, u64)> = HashSet::new();
    let mut start: (u64, u64) = (0, 0);
    for (r, line) in input.lines().enumerate() {
        let line_bytes = line.as_bytes();
        for c in 0..line_bytes.len() {
            match line_bytes[c] {
                b'S' => {
                    start = (r as u64, c as u64);
                }
                b'^' => {
                    splitters.insert((r as u64, c as u64));
                }
                _ => {}
            }
        }
    }
    let mut current: HashMap<(u64, u64), u64> = HashMap::new();
    current.insert(start, 1);
    for _ in input.lines() {
        let mut nextmap: HashMap<(u64, u64), u64> = HashMap::new();
        for (&(r, c), &v) in current.iter() {
            if splitters.contains(&(r + 1, c)) {
                *nextmap.entry((r + 1, c + 1)).or_insert(0) += v;
                *nextmap.entry((r + 1, c - 1)).or_insert(0) += v;
            } else {
                *nextmap.entry((r + 1, c)).or_insert(0) += v;
            }
        }
        current = nextmap;
    }

    Some(current.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
