#![feature(is_sorted)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

fn parse(path: &str) -> (Vec<(isize, isize)>, Vec<Vec<isize>>) {
    let content = fs::read_to_string(path).expect("File not found");
    let (order_raw, updates_raw) = content.split_once("\n\n").expect("Expected two parts");
    let pairs: Vec<(isize, isize)> = order_raw
        .split("\n")
        .map(|s| s.split_once("|").expect("Expected a pair separated by |"))
        .map(|(l, r)| {
            (
                l.parse().expect("Expected integer"),
                r.parse().expect("Expected integer"),
            )
        })
        .collect();
    let updates: Vec<Vec<isize>> = updates_raw
        .split("\n")
        .map(|s| {
            s.split(",")
                .map(|v| v.parse().expect("Expected an integer"))
                .collect()
        })
        .collect();
    (pairs, updates)
}

fn is_ordered(update: &[isize], order: &HashSet<(isize, isize)>) -> bool {
    update.is_sorted_by(|&a, &b| order.contains(&(a, b)))
}

fn make_order(pairs: &[(isize, isize)]) -> HashSet<(isize, isize)> {
    let mut items: HashSet<isize> = HashSet::new();
    let mut order: HashSet<(isize, isize)> = HashSet::new();
    let mut less_than: HashMap<isize, HashSet<isize>> = HashMap::new();
    let mut greater_than: HashMap<isize, HashSet<isize>> = HashMap::new();
    for (l, r) in pairs {
        items.insert(*l);
        items.insert(*r);
        order.insert((*l, *r));
        less_than.entry(*l).or_insert(HashSet::new()).insert(*r);
        greater_than.entry(*r).or_insert(HashSet::new()).insert(*l);
    }
    let mut prev_len = 0;
    let mut new_pairs = HashSet::new();
    while order.len() > prev_len {
        prev_len = order.len();
        for (l, r) in order.iter() {
            for x in less_than.entry(*r).or_insert(HashSet::new()).iter() {
                new_pairs.insert((*r, *x));
                greater_than.entry(*x).or_insert(HashSet::new()).insert(*r);
            }
            for x in greater_than.entry(*l).or_insert(HashSet::new()).iter() {
                new_pairs.insert((*x, *l));
                less_than.entry(*x).or_insert(HashSet::new()).insert(*l);
            }
        }
        for pair in new_pairs.drain() {
            order.insert(pair);
        }
    }
    order
}

fn compare_pages(a: isize, b: isize, order: &HashSet<(isize, isize)>) -> Ordering {
    if a == b {
        return Ordering::Equal;
    } else if order.contains(&(a, b)) {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

fn part1(updates: &[Vec<isize>], order: &HashSet<(isize, isize)>) -> isize {
    updates
        .iter()
        .filter(|u| is_ordered(u, &order))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn part2(updates: &mut [Vec<isize>], order: &HashSet<(isize, isize)>) -> isize {
    let mut filtered: Vec<Vec<isize>> = updates
        .iter()
        .filter(|u| !is_ordered(u, &order))
        .cloned()
        .collect();
    filtered
        .iter_mut()
        .for_each(|u| u.sort_by(|&a, &b| compare_pages(a, b, &order)));
    filtered.iter().map(|u| u[u.len() / 2]).sum()
}

fn main() {
    let (pairs, mut updates) = parse("input.txt");
    let order = make_order(&pairs);
    println!("Part 1: {}", part1(&updates, &order));
    println!("Part 2: {}", part2(&mut updates, &order));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let (pairs, updates) = parse("test.txt");
        let order = make_order(&pairs);
        assert_eq!(part1(&updates, &order), 143)
    }

    #[test]
    fn test_part2() {
        let (pairs, mut updates) = parse("test.txt");
        let order = make_order(&pairs);
        assert_eq!(part2(&mut updates, &order), 123)
    }
}
