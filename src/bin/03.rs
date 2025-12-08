use pom::parser::*;
use std::cmp::max;
advent_of_code::solution!(3);

fn parser<'a>() -> Parser<'a, u8, Vec<Vec<u64>>> {
    let digit = one_of(b"0123456789");

    let number = digit.repeat(1..).map(|n| n.into_iter().map(|x| (x - b'0') as u64).collect());
    let newline = one_of(b"\r\n");
    (number - newline.discard()).repeat(1..)
}

pub fn part_one(input: &str) -> Option<u64> {
    let p = parser();
    let mut result = 0;
    if let Ok(lines) = p.parse(input.as_bytes()) {
        for line in lines {
            let mut i = line.iter();
            let mut b = i.next_back().copied()?;
            let mut a = i.next_back().copied()?;
                for d in i.map(|d| *d).rev() {
                    if d >= a {
                        b = max(a,b);
                        a = d;
                    }
                }
            result += 10*a + b;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let p = parser();
    let mut result = 0;
    let range = 12;
    if let Ok(lines) = p.parse(input.as_bytes()) {
        for line in lines {
            let mut best = if line.len() > range { line[line.len() - range ..].to_vec() } else { line.clone()};
            let iterator = line[..line.len() - 12].iter();
            for mut d in iterator.map(|d| *d).rev() {
                for i in 0..12  {
                    if d < best[i] {
                        break;
                    }
                    let tmp = best[i];
                    best[i] = d;
                    d = tmp;
                }
            }
            let n = best.iter().fold(0,|acc, e| acc * 10 + e);
            result += n;
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
