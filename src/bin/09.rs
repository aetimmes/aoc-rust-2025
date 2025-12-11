advent_of_code::solution!(9);
use std::{
    cmp::{max, min},
    collections::HashSet,
};

use itertools::{Itertools};

pub fn part_one(input: &str) -> Option<u64> {
    let mut points: Vec<(u64, u64)> = Vec::new();
    let mut result: u64 = 0;
    for line in input.lines() {
        let t = line.split_once(",")?;
        let Ok(c1) = u64::from_str_radix(t.0, 10) else {
            todo!()
        };
        let Ok(c2) = u64::from_str_radix(t.1, 10) else {
            todo!()
        };

        let points_iter = points.iter();
        for p in points_iter {
            result = max(result, (p.0.abs_diff(c1) + 1) * (p.1.abs_diff(c2) + 1));
        }

        points.push((c1, c2));
    }
    Some(result)
}

fn print_map(m: HashSet<(u64, u64)>) {
    let Some(mr) = m.iter().map(|x| x.0).max() else {todo!()};
    let Some(mc) = m.iter().map(|x| x.1).max() else {todo!()};
    for r in 0..(mr+2) {
        let mut s: String = "".to_string();
        for c in 0..(mc+1) {
            if m.contains(&(r,c)) {
                s += "#"
            } else {
                s += "."
            }
        }
        println!("{:?}", s);
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut points: Vec<(u64, u64)> = Vec::new();
    for line in input.lines() {
        let t = line.split_once(",")?;
        let Ok(c1) = u64::from_str_radix(t.0, 10) else {
            todo!()
        };
        let Ok(c2) = u64::from_str_radix(t.1, 10) else {
            todo!()
        };

        points.push((c1, c2));
    }
    let mut curr = points.last()?;
    let mut m: HashSet<(u64, u64)> = HashSet::new();
    for p in points.iter() {
        if curr.0 == p.0 {
            let (c1, c2): (u64, u64);
            if curr.1 < p.1 {
                (c1, c2) = (curr.1, p.1);
            } else {
                (c1, c2 ) = (p.1, curr.1);
            }
            for i in c1*2..c2*2 {
                m.insert((curr.0*2, i));
            }
        } else {
            let (r1, r2): (u64, u64);
            if curr.0 < p.0 {
                (r1, r2) = (curr.0, p.0);
            } else {
                (r1, r2) = (p.0, curr.0)
            }
            for i in r1*2..r2*2 {
                m.insert((i, curr.1*2));
            }
        }
        curr = p;
    }

    //print_map(m.clone());

    let mut result: u64 = 0;
    for p in points.iter().combinations(2) {
        let mut valid = true;
        let r1 = min(p[0].0,p[1].0);
        let r2 = max(p[0].0,p[1].0);
        let c1 = min(p[0].1, p[1].1);
        let c2 = max(p[0].1, p[1].1);

        //println!("testing {:?}", ((r1,c1),(r2,c2)) );

        for i in ((c1*2)+1)..((c2*2)-1) {
            if m.contains(&((r1*2)+1, i)) || m.contains(&((r2*2)-1, i)) {
                valid = false;
                break;
            }
        }
        if !valid { continue };
        for i in ((r1*2)+1)..((r2*2)-1) {
            if m.contains(&(i, (c1*2)+1)) || m.contains(&(i, (c2*2)-1)) {
                valid = false;
                break;
            }
        }
        if valid {
            //println!("valid");
            let area  = (r1.abs_diff(r2)+1) * (c1.abs_diff(c2)+1);
            if area > result {
                //println!("new max: {:?}", area);
                result = area;
            }
        } else {
            //println!("invalid");
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
