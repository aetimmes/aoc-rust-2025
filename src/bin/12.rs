advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    for line in input.lines() {
        if !line.contains("x") {
            continue;
        }
        let Some((dim, counts)) = line.split_once(": ") else {todo!()};
        let (x,y) = dim.split_once("x")?;
        let area = u64::from_str_radix(x, 10).ok()? * u64::from_str_radix(y, 10).ok()?;
        let nums = counts.split(" ");
        let mut total = 0;
        for num in nums {
            total += 9 * u64::from_str_radix(num, 10).ok()?;
        }
        if total <= area {
            result += 1;
        }
    }
    Some(result)
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
