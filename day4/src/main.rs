use std::{collections::HashMap, fs};

fn load(path: &str) -> HashMap<(isize, isize), char> {
    let mut res = HashMap::new();
    let raw = fs::read_to_string(path).expect("File not found");
    for (y, row) in raw.split('\n').enumerate() {
        for (x, c) in row.chars().enumerate() {
            res.insert((x as isize, y as isize), c);
        }
    }
    res
}

fn is_xmas(grid: &HashMap<(isize, isize), char>, x: isize, y: isize, dir: usize) -> bool {
    if *grid.get(&(x, y)).unwrap() != 'X' {
        return false;
    }
    for (dx, c) in ['M', 'A', 'S'].iter().enumerate() {
        let next_key = match dir {
            0 => (x + dx as isize + 1, y),
            1 => (x + dx as isize + 1, y - dx as isize - 1),
            2 => (x, y - dx as isize - 1),
            3 => (x - dx as isize - 1, y - dx as isize - 1),
            4 => (x - dx as isize - 1, y),
            5 => (x - dx as isize - 1, y + dx as isize + 1),
            6 => (x, y + dx as isize + 1),
            7 => (x + dx as isize + 1, y + dx as isize + 1),
            _ => unreachable!(),
        };
        match grid.get(&next_key) {
            Some(gc) => {
                if gc != c {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}


fn is_x_mas(grid: &HashMap<(isize, isize), char>, x: isize, y: isize) -> bool {
    if *grid.get(&(x, y)).unwrap() != 'A' {
        return false;
    }
    let diag1 = [grid.get(&(x + 1, y + 1)), grid.get(&(x - 1, y - 1))];
    let diag2 = [grid.get(&(x - 1, y + 1)), grid.get(&(x + 1, y - 1))];
    diag1.contains(&Some(&'M')) && diag1.contains(&Some(&'S')) && diag2.contains(&Some(&'M')) && diag2.contains(&Some(&'S'))
}


fn count_xmas(grid: &HashMap<(isize, isize), char>) -> usize {
    let mut xmas = 0usize;
    let dim = grid.keys().max().unwrap();
    for x in 0..dim.0 + 1 {
        for y in 0..dim.1 + 1 {
            for dir in 0..8 {
                if is_xmas(grid, x, y, dir) {
                    xmas += 1;
                }
            }
        }
    }
    xmas
}

fn count_x_mas(grid: &HashMap<(isize, isize), char>) -> usize {
    let mut xmas = 0usize;
    let dim = grid.keys().max().unwrap();
    for x in 0..dim.0 + 1 {
        for y in 0..dim.1 + 1 {
            if is_x_mas(grid, x, y) {
                xmas += 1;
            }
        }
    }
    xmas
}

fn main() {
    let grid = load("input.txt");
    println!("Part 1: {}", count_xmas(&grid));
    println!("Part 2: {}", count_x_mas(&grid));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let data = load("test.txt");
        assert_eq!(count_xmas(&data), 18);
    }

    #[test]
    fn part2_test() {
        let data = load("test.txt");
        assert_eq!(count_x_mas(&data), 9);
    }
}
