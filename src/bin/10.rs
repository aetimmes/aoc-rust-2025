advent_of_code::solution!(10);
use std::iter::Rev;

use pom::parser::*;

fn two_bits(mut v: Vec<u64>) -> u64 {
    v.reverse();
    v.iter().fold(0, |acc, x| acc*2 + x)
}

fn light<'a>() -> Parser<'a, u8, u64> {
    sym(b'.').map(|_| 0 as u64) | sym(b'#').map(|_| 1 as u64)
}

fn lights<'a>() -> Parser<'a, u8, u64> {
    light().repeat(1..).map(|v| two_bits(v))
}

fn initial_state<'a>() -> Parser<'a, u8, u64> {
    sym(b'[').discard() * lights() - sym(b']').discard()
}

fn space<'a>() -> Parser<'a, u8, ()> {
    sym(b' ').repeat(1..).discard()
}

fn number<'a>() -> Parser<'a, u8, u64> {
    let digits = one_of(b"0123456789").repeat(1..);
    digits.collect().convert(str::from_utf8).convert(|s|u64::from_str_radix(&s, 10))
}
fn buttons<'a>() -> Parser<'a, u8, u64> {
    sym(b'(') * list(number(), sym(b',')).collect().map() - sym(b')').discard()
}

fn parser<'a>() -> Parser<'a, u8, (u64, Vec<u64>, u64)> {
    initial_state() + space() +
}

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
