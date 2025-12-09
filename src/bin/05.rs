use pom::parser::*;
use std::cmp::max;
use itertools::any;

advent_of_code::solution!(5);

fn number<'a>() -> Parser<'a, u8, u64> {
    let digit = one_of(b"0123456789");
    digit.repeat(1..).collect().convert(str::from_utf8).convert(|s| u64::from_str_radix(&s, 10))
}

fn newline<'a>() -> Parser<'a, u8, u8> {
    one_of(b"\r\n")
}

fn parser<'a>() -> Parser<'a, u8, (Vec<(u64,u64)>,Vec<u64>)> {
    let range = number() - sym(b'-').discard() + number();
    list(range, newline()) - newline().repeat(1..).discard() + list(number(), newline())
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    if let Ok((ranges, ids)) = parser().parse(input.as_bytes()){
        for id in ids {
            let r = ranges.clone();
            if any(r, |(x,y)| (x..(y+1)).contains(&id)) {
                result += 1;
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    if let Ok((mut ranges, _ids)) = parser().parse(input.as_bytes()){
        ranges.sort();
        let mut curr: (u64, u64) = (0,0);
        for range in ranges {
            if curr == (0,0) {
                curr = range
            } else if range.0 <= curr.1 {
                curr = (curr.0, max(range.1, curr.1));
            } else {
                result += curr.1 - curr.0 + 1;
                curr = range;
            }
        }
        result += curr.1 - curr.0 + 1;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}

// 347444809832352 TOO LOW
// 347444809832329
// 34744480983235200 TOO HIGH
