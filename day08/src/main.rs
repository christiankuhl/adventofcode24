use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
    mem::swap,
};

struct Grid {
    antennae: HashMap<char, HashSet<(isize, isize)>>,
    antinodes_pt1: HashSet<(isize, isize)>,
    antinodes_pt2: HashSet<(isize, isize)>,
    width: isize,
    height: isize,
}

impl Grid {
    fn in_grid(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
    fn find_antinodes_pt1(&mut self) {
        for antennae in self.antennae.values() {
            for pair in antennae.iter().combinations(2) {
                let (x1, y1) = pair[0];
                let (x2, y2) = pair[1];
                let nx1 = 2 * x2 - x1;
                let ny1 = 2 * y2 - y1;
                let nx2 = 2 * x1 - x2;
                let ny2 = 2 * y1 - y2;
                if self.in_grid(nx1, ny1) {
                    self.antinodes_pt1.insert((nx1, ny1));
                }
                if self.in_grid(nx2, ny2) {
                    self.antinodes_pt1.insert((nx2, ny2));
                }
            }
        }
    }

    fn find_antinodes_pt2(&mut self) {
        for antennae in self.antennae.values() {
            for pair in antennae.iter().combinations(2) {
                let (x1, y1) = *pair[0];
                let (x2, y2) = *pair[1];
                let mut dx = x2 - x1;
                let mut dy = y2 - y1;
                let m = gcd(dx.unsigned_abs(), dy.unsigned_abs()) as isize;
                dx /= m;
                dy /= m;
                let mut px = x1;
                let mut py = y1;
                let mut qx = x1;
                let mut qy = y1;
                while self.in_grid(px, py) || self.in_grid(qx, qy) {
                    if self.in_grid(px, py) {
                        self.antinodes_pt2.insert((px, py));
                    }
                    if self.in_grid(qx, qy) {
                        self.antinodes_pt2.insert((qx, qy));
                    }
                    px -= dx;
                    py -= dy;
                    qx += dx;
                    qy += dy;
                }
            }
        }
    }
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn load(path: &str) -> Grid {
    let mut width: isize = 0;
    let mut height = 0;
    let mut antennae = HashMap::new();
    let antinodes_pt1 = HashSet::new();
    let antinodes_pt2 = HashSet::new();
    fs::read_to_string(path)
        .expect("File not found")
        .split('\n')
        .enumerate()
        .for_each(|(y, r)| {
            height += 1;
            width = r.len() as isize;
            r.chars().enumerate().for_each(|(x, c)| {
                if c.is_alphanumeric() {
                    antennae
                        .entry(c)
                        .or_insert(HashSet::new())
                        .insert((x as isize, y as isize));
                } else {
                    assert_eq!(c, '.');
                }
            });
        });
    Grid {
        antennae,
        antinodes_pt1,
        antinodes_pt2,
        width,
        height,
    }
}

fn part1(grid: &mut Grid) -> usize {
    grid.find_antinodes_pt1();
    grid.antinodes_pt1.len()
}

fn part2(grid: &mut Grid) -> usize {
    grid.find_antinodes_pt2();
    grid.antinodes_pt2.len()
}

fn main() {
    let mut grid = load("input.txt");
    println!("Part 1: {}", part1(&mut grid));
    println!("Part 2: {}", part2(&mut grid));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let mut grid = load("test.txt");
        assert_eq!(part1(&mut grid), 14);
    }

    #[test]
    fn test_part2() {
        let mut grid = load("test.txt");
        assert_eq!(part2(&mut grid), 34);
    }
}
