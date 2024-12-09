use std::fs;

fn is_safe(levels: &[i32]) -> bool {
    let mut next_lvl = levels.iter();
    let next = next_lvl.next();
    assert!(next.is_some());
    let mut sign: Option<i32> = None;
    for (prev, next) in levels.iter().zip(next_lvl) {
        if (next - prev).abs() < 1 || (next - prev).abs() > 3 {
            return false;
        }
        if sign.is_none() {
            sign = Some((next - prev).signum());
        } else {
            let direction = sign.unwrap();
            if direction != (next - prev).signum() {
                return false;
            }
        }
    }
    true
}

fn is_safe_pt2(levels: &[i32]) -> bool {
    if !is_safe(levels) {
        for lvl in 0..levels.len() {
            let mut new_lvl: Vec<i32> = levels.to_vec();
            new_lvl.remove(lvl);
            if is_safe(&new_lvl) {
                return true;
            }
        }
        return false;
    }
    true
}

fn part1(levels: &[Vec<i32>]) -> usize {
    levels.iter().filter(|&lvl| is_safe(lvl)).count()
}

fn part2(levels: &[Vec<i32>]) -> usize {
    levels.iter().filter(|&lvl| is_safe_pt2(lvl)).count()
}

fn load(path: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse().expect("Expected i32..."))
                .collect()
        })
        .collect()
}

fn main() {
    let levels = load("day2.txt");
    println!("Part 1: {}", part1(&levels));
    println!("Part 2: {}", part2(&levels));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2_test() {
        let data = load("test.txt");
        assert_eq!(part2(&data), 4);
    }
}