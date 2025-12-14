advent_of_code::solution!(11);
use std::collections::HashMap;
use memoize::memoize;

use pom::{parser::*, parser::Parser};

fn word<'a>() -> Parser<'a, u8, String> {
    let letter = one_of(b"abcdefghijklmnopqrstuvwxyz");
    letter.repeat(1..).collect().convert(|x| String::from_utf8(x.to_vec()))
}

fn space<'a>() -> Parser<'a, u8, ()> {
    sym(b' ').repeat(1..).discard()
}

fn parser<'a>() -> Parser<'a, u8, (String, Vec<String>)> {

    word() - (sym(b':').discard() - space()) + list(word(), space())
}

#[memoize(Ignore: m)]
fn dfs(m: &HashMap<String,Vec<String>>, k: String, goal: String) -> u64 {
    if k == *goal {
        return 1_u64;
    }
    if !m.contains_key(&k) {
        return 0_u64;
    }
    m[&k].iter().map(|j| dfs(m, j.to_string(), goal.clone())).sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let p = parser();
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        if let Ok((k, v)) = p.parse(line.as_bytes()) {
            m.insert(k, v);
        }
    }

    Some(dfs(&m,"you".to_string(), "out".to_string()))
}

pub fn part_two(input: &str) -> Option<u64> {
    let p = parser();
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        if let Ok((k, v)) = p.parse(line.as_bytes()) {
            m.insert(k, v);
        }
    }
    let sd = dfs(&m, "svr".to_string(), "dac".to_string());
    let sf = dfs(&m, "svr".to_string(), "fft".to_string());
    let df = dfs(&m, "dac".to_string(), "fft".to_string());
    let fd = dfs(&m, "fft".to_string(), "dac".to_string());
    let dout = dfs(&m, "dac".to_string(),"out".to_string());
    let fout = dfs(&m, "fft".to_string(), "out".to_string());

    Some(
        (sd*df*fout) + (sf*fd*dout)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
