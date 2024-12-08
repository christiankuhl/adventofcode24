use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn rotate(&mut self) {
        *self = Self::from(((*self as usize) + 1) % 4);
    }
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => unreachable!(),
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}

enum GuardTrajectory {
    LeavesArea,
    Loop,
}

#[derive(Debug, Clone, Default)]
struct LabArea {
    width: isize,
    height: isize,
    guard: (isize, isize),
    guard_direction: Direction,
    guard_states: HashSet<((isize, isize), Direction)>,
    blocks: HashSet<(isize, isize)>,
}

impl LabArea {
    fn move_guard(&mut self) -> GuardTrajectory {
        let mut next_pos;
        loop {
            next_pos = match self.guard_direction {
                Direction::North => (self.guard.0, self.guard.1 - 1),
                Direction::East => (self.guard.0 + 1, self.guard.1),
                Direction::South => (self.guard.0, self.guard.1 + 1),
                Direction::West => (self.guard.0 - 1, self.guard.1),
            };
            if next_pos.0 < 0
                || next_pos.0 >= self.width
                || next_pos.1 < 0
                || next_pos.1 >= self.height
            {
                return GuardTrajectory::LeavesArea;
            }
            if self.blocks.contains(&next_pos) {
                self.guard_direction.rotate();
                continue;
            }
            self.guard = next_pos;
            if !self.guard_states.insert((next_pos, self.guard_direction)) {
                return GuardTrajectory::Loop;
            }
        }
    }
    fn guard_positions(&self) -> HashSet<(isize, isize)> {
        let mut guard_positions = HashSet::new();
        for (pos, _) in &self.guard_states {
            guard_positions.insert(*pos);
        }
        guard_positions
    }
}

fn part1(lab: &LabArea) -> usize {
    let mut lab = lab.clone();
    lab.move_guard();
    lab.guard_positions().len()
}

fn part2(lab: &LabArea) -> usize {
    let mut res = 0;
    let mut lab = lab.clone();
    let mut new_lab = lab.clone();
    let guard_pos = lab.guard;
    new_lab.move_guard();
    for (x, y) in new_lab.guard_positions() {
        if lab.blocks.contains(&(x, y)) || lab.guard == (x, y - 1) {
            continue;
        }
        let mut new_lab = lab.clone();
        new_lab.blocks.insert((x, y));
        match new_lab.move_guard() {
            GuardTrajectory::Loop => res += 1,
            GuardTrajectory::LeavesArea => {}
        }
    }
    res
}

fn load(path: &str) -> LabArea {
    let mut result = LabArea::default();
    let raw = fs::read_to_string(path).expect("File not found");
    for (y, row) in raw.split("\n").enumerate() {
        result.width = row.len() as isize;
        result.height += 1;
        for (x, c) in row.chars().enumerate() {
            match c {
                '#' => {
                    result.blocks.insert((x as isize, y as isize));
                }
                '^' => {
                    result.guard = (x as isize, y as isize);
                    result
                        .guard_states
                        .insert(((x as isize, y as isize), Direction::North));
                }
                '.' => {}
                _ => unreachable!(),
            }
        }
    }
    result
}

fn main() {
    let lab = load("input.txt");
    println!("Part 1: {}", part1(&lab));
    println!("Part 2: {}", part2(&lab));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let lab = load("test.txt");
        assert_eq!(part1(&lab), 41)
    }

    #[test]
    fn test_part2() {
        let lab = load("test.txt");
        assert_eq!(part2(&lab), 6)
    }
}
