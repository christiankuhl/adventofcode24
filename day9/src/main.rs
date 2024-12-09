use std::cell::Cell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy)]
enum Block {
    Free(usize),
    File(usize, usize, bool),
}

fn load(path: &str) -> String {
    fs::read_to_string(path).expect("File not found")
}

fn part1(map: &str) -> usize {
    let mut free: Vec<usize> = Vec::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut num_blocks = 0;
    for (j, c) in map.chars().enumerate() {
        let n = c.to_digit(10).unwrap() as usize;
        let id = j / 2;
        if j % 2 == 0 {
            for _ in 0..n {
                blocks.push(Block::File(id, 1, false));
                num_blocks += 1;
            }
        } else {
            for _ in 0..n {
                free.push(blocks.len());
                blocks.push(Block::Free(1));
            }
        }
    }
    free.reverse();
    while blocks.len() > num_blocks {
        match blocks.pop() {
            Some(Block::Free(_)) => continue,
            Some(Block::File(id, _, _)) => {
                let idx = free.pop().unwrap();
                blocks[idx] = Block::File(id, 1, false);
            }
            None => unreachable!(),
        }
    }
    blocks
        .iter()
        .enumerate()
        .map(|(j, b)| {
            j * match b {
                Block::File(id, _, _) => *id,
                Block::Free(_) => 0,
            }
        })
        .sum()
}

fn part2(map: &str) -> usize {
    let mut free: Vec<BinaryHeap<Reverse<Cell<usize>>>> = vec![BinaryHeap::new(); 10];
    let mut blocks: Vec<Block> = Vec::new();
    for (j, c) in map.chars().enumerate() {
        let n = c.to_digit(10).unwrap() as usize;
        let id = j / 2;
        if j % 2 == 0 {
            blocks.push(Block::File(id, n, false));
        } else {
            free[n].push(Reverse(Cell::new(j)));
            blocks.push(Block::Free(n));
        }
    }
    let mut reordered_blocks = blocks.clone();
    let mut block_placement: HashMap<usize, usize> = HashMap::new();
    for (j, b) in blocks.iter().enumerate() {
        if let Block::File(id, _, _) = b {
            block_placement.insert(*id, j);
        }
    }
    for b in blocks.iter().rev() {
        if let Block::File(id, n, false) = b {
            let current_spot = block_placement.get(id).unwrap();
            let available_len: Vec<usize> = (*n..10).filter(|&k| !free[k].is_empty()).collect();
            if let Some((k, new_spot)) = available_len
                .iter()
                .map(|&k| (k, free[k].peek().unwrap().0.get()))
                .filter(|(_, v)| *v < *current_spot)
                .min_by_key(|&(_, v)| v)
            {
                reordered_blocks[new_spot] = Block::File(*id, *n, true);
                reordered_blocks[*current_spot] = Block::Free(*n);
                free[*n].push(Reverse(Cell::new(*current_spot)));
                if k > *n {
                    reordered_blocks.insert(new_spot + 1, Block::Free(k - *n));
                    for slots in free.iter() {
                        for sp in slots.iter() {
                            if sp.0.get() >= new_spot {
                                sp.0.set(sp.0.get() + 1);
                            }
                        }
                    }
                    for k in block_placement.values_mut() {
                        if *k > new_spot {
                            *k += 1;
                        }
                    }
                    free[k - *n].push(Reverse(Cell::new(new_spot + 1)));
                }
                free[k].pop();
            }
        }
    }
    let mut idx = 0;
    let mut res = 0;
    for b in reordered_blocks {
        match b {
            Block::Free(n) => {
                idx += n;
            }
            Block::File(id, n, _) => {
                res += id * (n * idx + n * (n - 1) / 2);
                idx += n;
            }
        }
    }
    res
}

fn main() {
    let map = load("input.txt");
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let map = load("test.txt");
        assert_eq!(part1(&map), 1928)
    }

    #[test]
    fn test_part2() {
        let map = load("test.txt");
        assert_eq!(part2(&map), 2858)
    }
}
