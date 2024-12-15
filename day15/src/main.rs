use std::{fs, marker::PhantomData};

trait Part {
    fn box_char() -> char;
}
enum Part1 {}
enum Part2 {}
impl Part for Part1 {
    fn box_char() -> char {
        'O'
    }
}
impl Part for Part2 {
    fn box_char() -> char {
        '['
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_vector(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!(),
        }
    }
}

struct Warehouse<T: Part> {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
    movements: Vec<char>,
    robot: (usize, usize),
    part: PhantomData<T>,
}

impl<T: Part> Warehouse<T> {
    fn new(warehouse: &str, movements: &str) -> Self {
        let movements = movements.replace('\n', "").chars().rev().collect();
        let mut grid = vec![];
        let mut robot = (0, 0);
        let mut found = 0;
        warehouse.split('\n').enumerate().for_each(|(y, line)| {
            let mut row = vec![];
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '@' {
                    robot = (x, y);
                    found += 1;
                }
                row.push(c);
            });
            grid.push(row);
        });
        let width = grid.last().expect("Empty grid").len();
        let height = grid.len();
        assert!(
            grid.iter().all(|r| r.len() == width),
            "Warehouse is ragged!"
        );
        assert!(found == 1, "Warehouse must contain exactly one robot!");
        Self {
            width,
            height,
            grid,
            movements,
            robot,
            part: PhantomData,
        }
    }
    fn next_cell(&self, pos: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        let (dx, dy) = direction.as_vector();
        let x = pos.0.checked_add_signed(dx)?;
        let y = pos.1.checked_add_signed(dy)?;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some((x, y))
    }
    fn move_robot_simple(&mut self, direction: Direction) {
        if let Some(mut tgt_cell) = self.next_empty_cell(direction) {
            loop {
                if tgt_cell == self.robot {
                    self.robot = self.next_cell(tgt_cell, direction).unwrap();
                    break;
                }
                let next_cell = self.next_cell(tgt_cell, direction.opposite()).unwrap();
                self.grid[tgt_cell.1][tgt_cell.0] = self.grid[next_cell.1][next_cell.0];
                self.grid[next_cell.1][next_cell.0] = '.';
                tgt_cell = next_cell;
            }
        }
    }
    fn next_empty_cell(&self, direction: Direction) -> Option<(usize, usize)> {
        let mut pos = self.robot;
        loop {
            pos = self.next_cell(pos, direction)?;
            if self.grid[pos.1][pos.0] == '.' {
                return Some(pos);
            } else if self.grid[pos.1][pos.0] == '#' {
                return None;
            }
        }
    }
    fn gps(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .map(|(x, &c)| if c == T::box_char() { 100 * y + x } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl<T: Part> std::fmt::Display for Warehouse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .grid
            .iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", s)
    }
}

impl Warehouse<Part1> {
    fn move_boxes(&mut self) {
        while !self.movements.is_empty() {
            let direction: Direction = self.movements.pop().unwrap().into();
            self.move_robot_simple(direction);
        }
    }
}

impl Warehouse<Part2> {
    fn move_boxes(&mut self) {
        while !self.movements.is_empty() {
            let direction: Direction = self.movements.pop().unwrap().into();
            match direction {
                Direction::Left | Direction::Right => self.move_robot_simple(direction),
                Direction::Up | Direction::Down => {
                    if !self.can_move(self.robot, direction) {
                        continue;
                    }
                    self.move_robot_complex(self.robot, '@', '.', direction);
                }
            }
        }
    }
    fn can_move(&self, pos: (usize, usize), direction: Direction) -> bool {
        let nxt = self.next_cell(pos, direction);
        if nxt.is_none() {
            return false
        }
        let nxt = nxt.unwrap();
        let c = self.grid[nxt.1][nxt.0];
        match c {
            '.' => true,
            '#' => false,
            '[' => {
                let right = (nxt.0 + 1, nxt.1);
                self.can_move(right, direction) && self.can_move(nxt, direction)
            }
            ']' => {
                let left = (nxt.0 - 1, nxt.1);
                self.can_move(left, direction) && self.can_move(nxt, direction)
            }
            _ => unreachable!("Encountered unexpected '{c}'!")
        }
    }
    fn move_robot_complex(&mut self, pos: (usize, usize), c: char, pc: char, direction: Direction) {
        let nxt = self.next_cell(pos, direction).unwrap();
        self.grid[pos.1][pos.0] = pc;
        let nc = &mut self.grid[nxt.1][nxt.0];
        if c == '@' {
            self.robot = nxt;
        }
        match nc {
            '.' => {
                *nc = c;
            },
            '#' => unreachable!(),
            '[' => {
                let right = (nxt.0 + 1, nxt.1);
                *nc = c;
                self.move_robot_complex(nxt, '[', c, direction);
                self.move_robot_complex(right, ']', '.', direction);
            }
            ']' => {
                let left = (nxt.0 - 1, nxt.1);
                *nc = c;
                self.move_robot_complex(nxt, ']', c, direction);
                self.move_robot_complex(left, '[', '.', direction);
            }
            _ => unreachable!("Unexpected '{nc}' encountered!")
        }
    }
}

fn load(path: &str) -> (String, String) {
    let raw = fs::read_to_string(path).expect("File not found");
    let (warehouse, movements) = raw.split_once("\n\n").expect("Malformed file");
    (warehouse.to_owned(), movements.to_owned())
}

fn part1(warehouse: &str, movements: &str) -> usize {
    let mut warehouse: Warehouse<Part1> = Warehouse::new(warehouse, movements);
    warehouse.move_boxes();
    warehouse.gps()
}

fn part2(warehouse: &str, movements: &str) -> usize {
    let warehouse = warehouse
        .replace('#', "##")
        .replace('.', "..")
        .replace('O', "[]")
        .replace('@', "@.");
    let mut warehouse: Warehouse<Part2> = Warehouse::new(&warehouse, movements);
    warehouse.move_boxes();
    warehouse.gps()
}

fn main() {
    let (warehouse, movements) = load("input.txt");
    println!("Part 1: {}", part1(&warehouse, &movements));
    println!("Part 2: {}", part2(&warehouse, &movements));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let (warehouse, movements) = load("test.txt");
        assert_eq!(part1(&warehouse, &movements), 10092);
    }

    #[test]
    fn test_part2() {
        let (warehouse, movements) = load("test.txt");
        assert_eq!(part2(&warehouse, &movements), 9021);
    }
}

// ####################
// ##[].......[].[][]##
// ##[]...........[].##
// ##[]........[][][]##
// ##[]......[]....[]##
// ##..##......[]....##
// ##..[]............##
// ##..@......[].[][]##
// ##......[][]..[]..##
// ####################

// ####################
// ##[]..[]......[][]##
// ##@....[]......[].##
// ##[.........[][][]##
// ##.[][].....[]..[]##
// ##..##......].....##
// ##................##
// ##[].....[]...[][]##
// ##[].....[].......##
// ####################