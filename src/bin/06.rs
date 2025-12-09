use pom::parser::*;
use std::iter::zip;
advent_of_code::solution!(6);

fn number<'a>() -> Parser<'a, u8, u64> {
    let digit = one_of(b"0123456789");
    digit.repeat(1..).collect().convert(str::from_utf8).convert(|s| u64::from_str_radix(&s, 10))
}

fn operator<'a>() -> Parser<'a, u8, char> {
    one_of(b"+*").map(|c| if c == b'*' {'*'} else {'+'})
}

fn numbers<'a>() -> Parser<'a, u8, Vec<u64>> {
    space().opt() * !one_of(b"+*") * list(number(), space()) - space().opt() - newline()
}

fn newline<'a>() -> Parser<'a, u8, ()> {
    one_of(b"\r\n").discard()
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t").repeat(1..).discard()
}

fn operations<'a>() -> Parser<'a, u8, Vec<char>> {
    space().opt() * -operator() * list(operator(), space()) - space().opt() - newline()
}

fn parser<'a>() -> Parser<'a, u8, (Vec<Vec<u64>>, Vec<char>)> {
    numbers().repeat(1..) + operations()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    if let Ok((numbers, operations)) = parser().parse(input.as_bytes()) {
        println!("{:?}, {:?}", numbers, operations);
        let t = transpose(numbers);
        for (operator, row) in zip(operations, t.iter()) {
            if operator == '*' {
                result += row.iter().fold(1, |acc, &x| acc * x);
            } else {
                result += row.iter().fold(0, |acc, &x| acc + x);
            }
        }
    } else {
        println!("lmao");
    }
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
