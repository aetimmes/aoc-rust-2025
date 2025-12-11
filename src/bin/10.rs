advent_of_code::solution!(10);

use pom::parser::*;
use std::collections::HashSet;

fn two_bits(mut v: Vec<u64>) -> u64 {
    v.reverse();
    v.iter().fold(0, |acc, x| acc * 2 + x)
}

fn two_pos(v: Vec<u64>) -> u64 {
    v.iter().fold(0, |acc, &x| acc + 2_u64.pow(x as u32))
}

fn light<'a>() -> Parser<'a, u8, u64> {
    sym(b'.').map(|_| 0 as u64) | sym(b'#').map(|_| 1 as u64)
}

fn lights<'a>() -> Parser<'a, u8, (u64, u64)> {
    let bits = light().repeat(1..);
    bits.map(|v| (2_u64.pow((v.len()) as u32) - 1, two_bits(v)))
}

fn initial_state<'a>() -> Parser<'a, u8, (u64, u64)> {
    sym(b'[').discard() * lights() - sym(b']').discard()
}

fn space<'a>() -> Parser<'a, u8, ()> {
    sym(b' ').repeat(1..).discard()
}

fn number<'a>() -> Parser<'a, u8, u64> {
    let digits = one_of(b"0123456789").repeat(1..);
    digits
        .collect()
        .convert(str::from_utf8)
        .convert(|s| u64::from_str_radix(&s, 10))
}

fn buttons<'a>() -> Parser<'a, u8, Vec<u64>> {
    //sym(b'(') * list(number(), sym(b',')).map(|s| two_pos(s.to_vec())) - sym(b')').discard()
    sym(b'(') * list(number(), sym(b',')) - sym(b')').discard()
}

fn reqs<'a>() -> Parser<'a, u8, Vec<u64>> {
    sym(b'{') * list(number(), sym(b',')) - sym(b'}').discard()
}

fn parser<'a>() -> Parser<'a, u8, ((u64, u64), Vec<Vec<u64>>, Vec<u64>)> {
    (initial_state().name("initial state")
        + (space() * list(buttons(), space())).name("buttons")
        + (space() * reqs()).name("reqs")
        - one_of(b"\n\r").repeat(0..).discard())
    .map(|x| (x.0.0, x.0.1, x.1))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    for line in input.lines() {
        let Ok(((_, goal), raw_buttons, _)) = parser().parse(line.as_bytes()) else {
            todo!()
        };
        let buttons: Vec<u64> = raw_buttons.iter().map(|x| two_pos(x.to_vec())).collect();
        let mut curr = HashSet::new();
        let mut seen = HashSet::new();
        curr.insert(0);
        seen.insert(0);
        let mut i = 0_u64;
        let mut found = false;
        while !found {
            i += 1;
            let mut next = HashSet::new();
            for s in &curr {
                for b in &buttons {
                    if !seen.contains(&(s ^ b)) {
                        next.insert(s ^ b);
                        seen.insert(s ^ b);
                    }
                    if s ^ b == goal {
                        found = true;
                    }
                }
            }
            curr = next;
        }
        result += i;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0_u64;
    for line in input.lines() {
        result += part_two_inner(line);
        println!("new result: {:?}", result);
    }
    Some(result)
}

async fn part_two_inner(input: &str) -> u64 {
    let Ok(((_, _), raw_buttons, goal)) = parser().parse(line.as_bytes()) else {
        todo!()
    };
    let buttons: Vec<Vec<u64>> = raw_buttons
        .iter()
        .map(|s| {
            s.iter().fold(vec![0; goal.len()], |acc, &x| {
                let mut nacc = acc.clone();
                nacc[x as usize] += 1;
                nacc
            })
        })
        .collect();
    let mut curr: HashSet<Vec<u64>> = HashSet::new();
    let mut seen: HashSet<Vec<u64>> = HashSet::new();
    curr.insert(vec![0; goal.len()]);
    seen.insert(vec![0; goal.len()]);
    let mut i = 0_u64;
    let mut found = false;
    while !found {
        i += 1;
        let mut next = HashSet::new();
        for s in &curr {
            for b in buttons.as_slice() {
                let candidate: Vec<u64> = s.iter().zip(b).map(|(x, y)| x + y).collect();
                if candidate == goal {
                    found = true;
                }
                if !seen.contains(&candidate)
                    && !candidate.iter().zip(goal.iter()).any(|(x, y)| x > y)
                {
                    next.insert(candidate.clone());
                    seen.insert(candidate);
                }
            }
        }
        curr = next;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let output: ((u64, u64), Vec<Vec<u64>>, Vec<u64>) = (
            (63, 46),
            vec![
                vec![0, 1, 2, 3, 4],
                vec![0, 3, 4],
                vec![0, 1, 2, 4, 5],
                vec![1, 2],
            ],
            vec![10, 11, 11, 5, 10, 5],
        );
        let result = parser().parse(input.as_bytes());
        match result {
            Ok(result) => {
                assert_eq!(result, output);
            }
            Err(result) => {
                println!("{:?}", result);
                panic!("oops");
            }
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
