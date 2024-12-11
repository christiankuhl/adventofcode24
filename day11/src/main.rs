use std::{collections::HashMap, fs};

fn load(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .expect("File not found")
        .split(' ')
        .map(|s| s.parse().expect("Expected a usize"))
        .collect()
}

fn transform(value: usize) -> Vec<usize> {
    if value == 0 {
        return vec![1];
    }
    let l = value.ilog10() + 1;
    if l % 2 == 0 {
        vec![value / 10usize.pow(l/2), value % 10usize.pow(l/2)]
    } else {
        vec![2024 * value]
    }
}

fn iterate(input: &[usize], blinks: usize) -> usize {
    let mut stones: HashMap<usize, usize> = input.iter().map(|&s| (s, 1usize)).collect();
    for _ in 0..blinks {
        let mut new_stones = HashMap::new();
        for (stone, count) in stones.iter() {
            let result = transform(*stone);
            for s in result {
                *new_stones.entry(s).or_insert(0) += count;
            }
        }
        stones = new_stones;
    }
    stones.values().sum()
}

fn part1(input: &[usize]) -> usize {
    iterate(input, 25)
}

fn part2(input: &[usize]) -> usize {
    iterate(input, 75)
}

fn main() {
    let input = load("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = load("test.txt");
        assert_eq!(part1(&input), 55312);
    }

    #[test]
    fn test_part2() {
        let input = load("test.txt");
        assert_eq!(part2(&input), 65601038650482);
    }
}
