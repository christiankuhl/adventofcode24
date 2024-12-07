use itertools::Itertools;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Concat,
}

fn load(path: &str) -> Vec<(usize, Vec<usize>)> {
    fs::read_to_string(path)
        .expect("File not found")
        .split("\n")
        .map(|r| {
            let (tgt, values) = r.split_once(": ").expect("Unexpected row format");
            (
                tgt.parse().expect("Expected usize"),
                values
                    .split(" ")
                    .map(|s| s.parse().expect("Expected usize"))
                    .collect(),
            )
        })
        .collect()
}

fn brute_force_solvable(lhs: usize, rhs: &[usize], op: Vec<Op>) -> bool {
    let n = rhs.len() - 1;
    let mut choices = Vec::new();
    for _ in 0..n {
        choices.push(op.clone());
    }
    eval(choices, lhs, rhs)
}

fn eval(choices: Vec<Vec<Op>>, lhs: usize, rhs: &[usize]) -> bool {
    choices
        .into_iter()
        .multi_cartesian_product()
        .map(|ops| {
            let mut rhs = rhs.iter();
            let start = *rhs.next().unwrap();
            let res = ops.iter().zip(rhs).fold(start, |acc, (op, x)| match op {
                Op::Add => acc + x,
                Op::Mul => acc * x,
                Op::Concat => 10usize.pow(x.checked_ilog10().unwrap_or(0) + 1) * acc + x,
            });
            res
        })
        .any(|res| res == lhs)
}

fn part1(eqns: &[(usize, Vec<usize>)]) -> usize {
    eqns.iter()
        .filter(|(l, r)| brute_force_solvable(*l, r, vec![Op::Add, Op::Mul]))
        .map(|(l, _)| l)
        .sum()
}

fn part2(eqns: &[(usize, Vec<usize>)]) -> usize {
    eqns.iter()
        .filter(|(l, r)| brute_force_solvable(*l, r, vec![Op::Add, Op::Mul, Op::Concat]))
        .map(|(l, _)| l)
        .sum()
}

fn main() {
    let eqns = load("input.txt");
    println!("Part 1: {}", part1(&eqns));
    println!("Part 2: {}", part2(&eqns));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let eqns = load("test.txt");
        assert_eq!(part1(&eqns), 3749);
    }

    #[test]
    fn test_part2() {
        let eqns = load("test.txt");
        assert_eq!(part2(&eqns), 11387);
    }
}
