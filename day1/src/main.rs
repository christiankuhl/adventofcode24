use std::fs;
use std::collections::HashMap;

fn parse_input() -> (Vec<u32>, Vec<u32>) {
    let mut left = vec![];
    let mut right = vec![];
    fs::read_to_string("part1.txt").unwrap().split('\n').for_each(
        |s| {
            let pair: Vec<u32> = s.split("   ").map(|t| t.parse().expect("Expected an unsigned integer")).collect();
            assert!(pair.len() == 2);
            left.push(pair[0]);
            right.push(pair[1]);
        }
    );
    (left, right)
}

fn part1(left: &[u32], right: &[u32]) -> u32 {
    let mut left = left.to_owned();
    left.sort();
    let mut right = right.to_owned();
    right.sort();
    left.iter().zip(right).fold(0, |a, (&l, r)| a + (l as i32 - r as i32).unsigned_abs())
}

fn part2(left: &[u32], right: &[u32]) -> u32 {
    let mut tally: HashMap<u32, u32> = HashMap::new();
    for r in right {
        tally.entry(*r).and_modify(|e| *e += 1).or_insert(1);
    }
    left.iter().fold(0, |a, &l| a + tally.get(&l).unwrap_or(&0) * l)
}

fn main() {
    let (left, right) = parse_input();
    println!("Part 1: {}", part1(&left, &right));
    println!("Part 2: {}", part2(&left, &right));
}
