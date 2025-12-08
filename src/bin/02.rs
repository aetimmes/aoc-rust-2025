use pom::parser::*;
use fancy_regex::Regex;
advent_of_code::solution!(2);


fn number<'a>() -> Parser<'a, u8, u64> {
    let digits = one_of(b"0123456789").repeat(1..);
    digits.collect().convert(str::from_utf8).convert(|s|u64::from_str_radix(&s, 10))
}

fn parser<'a>() -> Parser<'a, u8, Vec<(u64, u64)>> {
    let range = number() - sym(b'-') + number();
    list(range, sym(b','))
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed = parser().parse(input.as_bytes());
    let mut result: u64 = 0;
    let re = Regex::new(r"^(.+)\1$");
    if let Ok(ranges) = parsed && let Ok(r) = re {
        for (start, end) in ranges {
            for i in start..end+1 {
                if let Ok(b)= r.is_match(&i.to_string()) && b {
                    result += i;
                }
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = parser().parse(input.as_bytes());
    let mut result: u64 = 0;
    let re = Regex::new(r"^(.+)\1+$");
    if let Ok(ranges) = parsed && let Ok(r) = re {
        for (start, end) in ranges {
            for i in start..end+1 {
                if let Ok(b)= r.is_match(&i.to_string()) && b {
                    result += i;
                }
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
