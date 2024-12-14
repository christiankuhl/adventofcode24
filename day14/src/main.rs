use std::{collections::HashMap, fs};
use std::io;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Robot(isize, isize, isize, isize);

impl Robot {
    fn move_fwd(&mut self, dt: isize, width: isize, height: isize) {
        self.0 = (self.0 + dt * self.2).rem_euclid(width);
        self.1 = (self.1 + dt * self.3).rem_euclid(height);
    }
}

struct Bathroom {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl Bathroom {
    fn wait(&mut self, dt: isize) {
        for r in self.robots.iter_mut() {
            r.move_fwd(dt, self.width as isize, self.height as isize);
        }
    }
}

impl std::fmt::Display for Bathroom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = vec![vec!["\x1b[38;5;0m."; self.width]; self.height];
        for r in self.robots.iter() {
            grid[r.1 as usize][r.0 as usize] = "\x1b[1;32m*";
        }
        let rows: Vec<String> = grid.iter().map(|row| row.join("")).collect();
        write!(f, "\x1b[1;1H{}", rows.join("\n"))
    }
}

fn load(path: &str, width: usize, height: usize) -> Bathroom {
    let robots = fs::read_to_string(path)
        .expect("File not found")
        .split('\n')
        .map(|l| {
            let (p, v) = l.split_once(' ').expect("Malformed line");
            let (px, py) = p.split_once(',').expect("Malformed p");
            let (vx, vy) = v.split_once(',').expect("Malformed v");
            let (_, px) = px.split_once('=').expect("Malformed p");
            let (_, vx) = vx.split_once('=').expect("Malformed v");
            Robot(
                px.parse().expect("Expected an isize for px"),
                py.parse().expect("Expected an isize for py"),
                vx.parse().expect("Expected an isize for vx"),
                vy.parse().expect("Expected an isize for vy"),
            )
        })
        .collect();
    Bathroom { width, height, robots }
}

fn part1(bathroom: &mut Bathroom) -> usize {
    let mut quadrants: HashMap<(bool, bool, bool), usize> = HashMap::new();
    let width = bathroom.width as isize;
    let height = bathroom.height as isize;
    bathroom.wait(100);
    for r in bathroom.robots.iter() {
        let quad = (r.0 < width / 2, r.1 < height / 2, r.0 == width / 2 || r.1 == height / 2);
        if !quad.2 {
            *quadrants.entry(quad).or_default() += 1;
        }
    }
    quadrants.values().product()
}

#[allow(dead_code)]
fn look_through(bathroom: &mut Bathroom) {
    let mut stdin = io::stdin();
    let mut dt = 70;
    bathroom.wait(70);
    loop {
        println!("{bathroom}\n\x1b[0;37mdt = {dt}");
        bathroom.wait(101);
        dt += 101;
        let _ = stdin.read(&mut [0u8]).unwrap();
    }
}


fn part2() -> usize {
    // The robots' x position is periodic with length 101s. I manually stepped through the first
    // time steps to find a picture that is somewhat coherent in the x direction at dt=70.
    // Stepping in increments of 101 from there revealed the christmas tree at dt=7847
    7847
}

fn main() {
    let mut bathroom = load("input.txt", 101, 103);
    println!("Part 1: {}", part1(&mut bathroom));
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let mut bathroom = load("test.txt", 11, 7);
        assert_eq!(part1(&mut bathroom), 12);
    }

    #[test]
    fn test_part2() {
        let mut bathroom = load("input.txt", 101, 103);
        bathroom.wait(part2() as isize);
        let maybe_tree = format!("{}", bathroom);
        let tree = fs::read_to_string("tree.bin").expect("File not found");
        assert_eq!(maybe_tree, tree);
    }
}
