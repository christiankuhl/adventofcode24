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
    start: (isize, isize),
    guard_direction: Direction,
    guard_states: HashSet<((isize, isize), Direction)>,
    guard_trajectory: Vec<((isize, isize), Direction)>,
    grid: Vec<Vec<char>>,
    dont_track_guard: bool,
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
            if self.grid[next_pos.1 as usize][next_pos.0 as usize] == '#' {
                if !self.guard_states.insert((next_pos, self.guard_direction)) {
                    return GuardTrajectory::Loop;
                } 
                self.guard_direction.rotate();
                continue;
            }
            self.guard = next_pos;
            if !self.dont_track_guard {
                self.guard_trajectory.push((next_pos, self.guard_direction));
            }
        }
    }
    fn guard_positions(&self) -> HashSet<(isize, isize)> {
        let mut guard_positions = HashSet::new();
        for (pos, _) in &self.guard_trajectory {
            guard_positions.insert(*pos);
        }
        guard_positions
    }
    fn reset(&mut self) {
        self.guard_trajectory.clear();
        self.guard_states.clear();
        self.guard = self.start;
        self.guard_direction = Direction::North;
    }
}

fn part1(lab: &LabArea) -> usize {
    let mut lab = lab.clone();
    lab.dont_track_guard = false;
    lab.move_guard();
    lab.guard_positions().len()
}

fn part2(lab: &mut LabArea) -> usize {
    let mut res = 0;
    lab.move_guard();
    lab.dont_track_guard = true;
    let positions_and_directions = lab.guard_trajectory.clone();
    for ((x, y), _) in positions_and_directions.iter().skip(2) {
        lab.reset();
        lab.grid[*y as usize][*x as usize] = '#';
        match lab.move_guard() {
            GuardTrajectory::Loop => res += 1,
            GuardTrajectory::LeavesArea => {}
        }
        lab.grid[*y as usize][*x as usize] = '.';
    }
    res - 1
}

fn load(path: &str) -> LabArea {
    let mut result = LabArea::default();
    let raw = fs::read_to_string(path).expect("File not found");
    for (y, row) in raw.split('\n').enumerate() {
        result.width = row.len() as isize;
        result.height += 1;
        result.grid.push(vec![]);
        for (x, c) in row.chars().enumerate() {
            match c {
                '#' => {
                    // result.grid(x as isize, y as isize);
                }
                '^' => {
                    result.guard = (x as isize, y as isize);
                    result.start = (x as isize, y as isize);
                    result
                        .guard_states
                        .insert(((x as isize, y as isize), Direction::North));
                }
                '.' => {}
                _ => unreachable!(),
            }
            result.grid[y].push(c);
        }
    }
    result
}

fn main() {
    let mut lab = load("input.txt");
    println!("Part 1: {}", part1(&lab));
    println!("Part 2: {}", part2(&mut lab));
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
        let mut lab = load("test.txt");
        assert_eq!(part2(&mut lab), 6)
    }
}
