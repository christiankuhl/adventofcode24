use regex::Regex;
use std::{fs, isize};

const LIMIT_PART1: isize = 100;
const OFFSET_PART2: isize = 10000000000000;

#[derive(Debug, Clone, Copy)]
struct Prize {
    btn_a: (isize, isize),
    btn_b: (isize, isize),
    prize: (isize, isize),
}

impl Prize {
    fn cost(&self, limit: isize) -> Option<isize> {
        let det = self.btn_a.0 * self.btn_b.1 - self.btn_a.1 * self.btn_b.0;
        if det == 0 {
            return None;
        }
        let da = self.prize.0 * self.btn_b.1 - self.prize.1 * self.btn_b.0;
        let db = self.btn_a.0 * self.prize.1 - self.btn_a.1 * self.prize.0;
        let a = da / det;
        let b = db / det;
        if a * self.btn_a.0 + b * self.btn_b.0 == self.prize.0
            && a * self.btn_a.0 + b * self.btn_b.0 == self.prize.0
            && a >= 0
            && b >= 0
            && a <= limit
            && b <= limit
        {
            return Some(3 * a + b);
        }
        None
    }
}

fn load(path: &str) -> Vec<Prize> {
    let button_pat = Regex::new(r"^Button ([A|B]): X\+([0-9]+), Y\+([0-9]+)$").unwrap();
    let prize_pat = Regex::new(r"^Prize: X=([0-9]+), Y=([0-9]+)$").unwrap();
    let mut prizes = vec![];
    fs::read_to_string(path)
        .expect("File not found")
        .split("\n\n")
        .for_each(|block| {
            let rows: Vec<&str> = block.split('\n').collect();
            assert!(rows.len() == 3, "Malformed block");
            let btn_a = button_pat.captures(rows[0]).expect("Malformed row A");
            assert!(btn_a.get(1).unwrap().as_str() == "A", "Expected button A");
            let btn_a = (
                btn_a.get(2).unwrap().as_str().parse().unwrap(),
                btn_a.get(3).unwrap().as_str().parse().unwrap(),
            );
            let btn_b = button_pat.captures(rows[1]).expect("Malformed row B");
            assert!(btn_b.get(1).unwrap().as_str() == "B", "Expected button B");
            let btn_b = (
                btn_b.get(2).unwrap().as_str().parse().unwrap(),
                btn_b.get(3).unwrap().as_str().parse().unwrap(),
            );
            let prize = prize_pat
                .captures(rows[2])
                .expect("Expected prize location");
            let prize = (
                prize.get(1).unwrap().as_str().parse().unwrap(),
                prize.get(2).unwrap().as_str().parse().unwrap(),
            );
            prizes.push(Prize {
                btn_a,
                btn_b,
                prize,
            });
        });
    prizes
}

fn part1(prizes: &[Prize]) -> isize {
    prizes.iter().filter_map(|p| p.cost(LIMIT_PART1)).sum()
}

fn part2(prizes: &mut [Prize]) -> isize {
    prizes.iter_mut().for_each(|p| {
        p.prize.0 += OFFSET_PART2;
        p.prize.1 += OFFSET_PART2;
    });
    prizes.iter().filter_map(|p| p.cost(isize::MAX)).sum()
}

fn main() {
    let mut prizes = load("input.txt");
    println!("Part 1: {}", part1(&prizes));
    println!("Part 2: {}", part2(&mut prizes));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let prizes = load("test.txt");
        assert_eq!(part1(&prizes), 480);
    }

    #[test]
    fn test_part2() {
        let mut prizes = load("test.txt");
        assert_eq!(part2(&mut prizes), 875318608908);
    }
}
