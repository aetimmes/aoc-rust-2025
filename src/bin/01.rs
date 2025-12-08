use pom::parser::*;
advent_of_code::solution!(1);

fn parser<'a>() -> Parser<'a, u8, Vec<i32>> {
    let digit = one_of(b"1234567890");
    let number = digit.repeat(1..).collect().convert(str::from_utf8).convert(|s|i32::from_str_radix(&s, 10))
    ;
    let direction = sym(b'L').map(|_| -1) | sym(b'R').map(|_| 1);
    let line = (direction + number).map(|(dir, dist)| (dir, dist));
    let newline = one_of(b"\r\n");
    ((line - newline).map(|(dir, dist)| dir * dist)).repeat(1..)
}

pub fn part_one(input: &str) -> Option<i32> {
   let res = parser().parse(input.as_bytes());
   let mut pos = 50;
   let mut result= 0;
   if let Ok(turns) = res {
        for turn in turns {
            pos = (pos + turn).rem_euclid(100);
            if pos == 0 {
                result += 1
            }
        }
   }
   println!("result: {:?}", result);
   Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
   let res = parser().parse(input.as_bytes());
   let mut pos = 50;
   let mut result= 0;
   if let Ok(turns) = res {
        for mut turn in turns {
            result += (turn / 100).abs();
            turn %= 100;
            pos += turn;
            if turn != 0 && !(pos == turn || pos == -turn) && (pos <= 0 || pos >= 100) {
                result += 1;
            }
            pos = pos.rem_euclid(100);
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
