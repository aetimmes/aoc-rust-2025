use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let lines: Vec<&str> = input.split("\n").collect();
    let mut m: HashMap<(i32,i32), u8>  = HashMap::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, e) in line.chars().enumerate() {
            if e == '@' {
                for dr in -1..2 as isize {
                    for dc in -1..2 as isize {
                        let nr = r as i32 + dr as i32;
                        let nc = c as i32 + dc as i32;
                        match m.get(&(nr,nc)) {
                            Some(i) => { m.insert((nr,nc),i+1);}
                            None => { m.insert((nr,nc), 1); }
                        }
                    }
                }
            }
        }
    }
    for (r, line) in lines.iter().enumerate() {
        for (c, e) in line.chars().enumerate() {
            if e == '@' {
                if let Some(x) = m.get(&(r as i32, c as i32)) {
                    if *x <= 4 {
                        result += 1;
                    }
                }
            }
        }
    }
    Some(result)
}

fn unload_rolls(s: &HashSet<(isize,isize)>) -> Option<(HashSet<(isize, isize)>, u64)> {
    let mut m: HashMap<(isize, isize), u8>  = HashMap::new();
    let set = s.clone();
    let mut result = 0;
    for (r, c) in s {
        for dr in -1..2 as isize {
            for dc in -1..2 as isize {
                let nr = r+dr;
                let nc = c+dc;
                match m.get(&(nr,nc)) {
                    Some(i) => { m.insert((nr,nc),i+1);}
                    None => { m.insert((nr,nc), 1); }
                }
            }
        }
    }
    let mut next_set = HashSet::new();
    for (r, c) in set {
        if let Some(x) = m.get(&(r, c)) {
            if *x <= 4 {
                result += 1;
            } else {
                next_set.insert((r,c));
            }
        }
    }
    if result > 0 {
        return Some((next_set, result))
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let lines: Vec<&str> = input.split("\n").collect();
    let mut s: HashSet<(isize, isize)> = HashSet::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, e) in line.chars().enumerate() {
            if e == '@' {
                s.insert((r as isize, c as isize));
            }
        }
    }

    while let Some((set, curr)) = unload_rolls(&s) {
        s = set;
        result += curr;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
