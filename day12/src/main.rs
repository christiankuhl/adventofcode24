use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

struct Garden {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Garden {
    fn get(&self, cell: GardenCell) -> char {
        self.grid[cell.1][cell.0]
    }
}

#[derive(Debug)]
struct Patch {
    vegetable: char,
    cells: Vec<GardenCell>,
}

impl Patch {
    fn area(&self) -> usize {
        self.cells.len()
    }
    fn perimeter(&self) -> usize {
        self.boundary_cells()
            .iter()
            .map(|(_, _, v0, v1)| (v0.abs() + v1.abs()) as usize)
            .sum()
    }
    fn boundary_cells(&self) -> Vec<(usize, usize, isize, isize)> {
        let mut bd: HashMap<(usize, usize), (isize, isize)> = HashMap::new();
        self.cells.iter().flat_map(|c| c.boundary()).for_each(|b| {
            let e = bd.entry((b.0, b.1)).or_insert((0, 0));
            e.0 += b.2;
            e.1 += b.3;
        });
        bd.iter()
            .filter(|&(_, v)| v.0 != 0 || v.1 != 0)
            .map(|((x0, x1), (v0, v1))| (*x0, *x1, *v0, *v1))
            .collect()
    }
    fn cost(&self) -> usize {
        self.area() * self.perimeter()
    }
    fn cost_after_discount(&self) -> usize {
        self.area() * self.boundary_sides()
    }
    fn boundary_sides(&self) -> usize {
        let mut horizontal: HashMap<usize, HashMap<usize, isize>> = HashMap::new();
        let mut vertical: HashMap<usize, HashMap<usize, isize>> = HashMap::new();
        let bdry = self.boundary_cells();
        for b in bdry.iter() {
            if b.2 != 0 {
                *horizontal.entry(b.1).or_default().entry(b.0).or_default() += b.2;
                *horizontal
                    .entry(b.1)
                    .or_default()
                    .entry(b.0 + 1)
                    .or_default() += -b.2;
            }
            if b.3 != 0 {
                *vertical.entry(b.0).or_default().entry(b.1).or_default() += b.3;
                *vertical.entry(b.0).or_default().entry(b.1 + 1).or_default() += -b.3;
            }
        }
        let sum_x: usize = horizontal
            .values()
            .map(|s| s.values().map(|&s| s.unsigned_abs()).sum::<usize>())
            .sum();
        let sum_y: usize = vertical
            .values()
            .map(|s| s.values().map(|&s| s.unsigned_abs()).sum::<usize>())
            .sum();
        (sum_x + sum_y) / 2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GardenCell(usize, usize);

impl GardenCell {
    fn neighbours(&self, garden: &Garden) -> Vec<Self> {
        let mut res = vec![];
        for dx in -1..2 {
            for dy in -1..2 {
                if dx == 0 && dy == 0 || dx != 0 && dy != 0 {
                    continue;
                }
                if let (Some(x), Some(y)) =
                    (self.0.checked_add_signed(dx), self.1.checked_add_signed(dy))
                {
                    if x < garden.width && y < garden.height {
                        res.push(GardenCell(x, y));
                    }
                }
            }
        }
        res
    }
    fn boundary(&self) -> Vec<(usize, usize, isize, isize)> {
        vec![
            (self.0, self.1, 0, 1),
            (self.0 + 1, self.1, 0, -1),
            (self.0, self.1, -1, 0),
            (self.0, self.1 + 1, 1, 0),
        ]
    }
}

fn load(path: &str) -> Garden {
    let mut grid = vec![];
    let mut height = 0;
    let mut width = 0;
    fs::read_to_string(path)
        .expect("File not found")
        .split('\n')
        .enumerate()
        .for_each(|(y, row)| {
            grid.push(vec![]);
            height += 1;
            row.chars().for_each(|c| {
                grid[y].push(c);
            });
            width = grid[y].len();
        });
    Garden {
        grid,
        width,
        height,
    }
}

fn fill_patch(start: GardenCell, remaining: &mut HashSet<GardenCell>, garden: &Garden) -> Patch {
    let mut current = start;
    let mut patch = Patch {
        vegetable: garden.get(current),
        cells: vec![current],
    };
    let mut visited = HashSet::new();
    let mut neighbours: Vec<GardenCell> = current
        .neighbours(garden)
        .iter()
        .filter(|&c| {
            garden.get(*c) == patch.vegetable && visited.insert(*c) && remaining.contains(c)
        })
        .cloned()
        .collect();
    visited.insert(current);
    remaining.remove(&current);
    while !neighbours.is_empty() {
        current = neighbours.pop().unwrap();
        patch.cells.push(current);
        remaining.remove(&current);
        neighbours.extend(
            current
                .neighbours(garden)
                .iter()
                .filter(|&c| {
                    garden.get(*c) == patch.vegetable && visited.insert(*c) && remaining.contains(c)
                })
                .cloned(),
        );
    }
    patch
}

fn map_garden(garden: &Garden) -> Vec<Patch> {
    let mut visited: HashSet<GardenCell> = HashSet::new();
    let mut remaining: HashSet<GardenCell> = HashSet::from_iter(
        (0..garden.width)
            .cartesian_product(0..garden.height)
            .map(|(x, y)| GardenCell(x, y)),
    );
    let mut patches: Vec<Patch> = vec![];
    while !remaining.is_empty() {
        let current = *remaining.iter().next().unwrap();
        let patch = fill_patch(current, &mut remaining, garden);
        for c in patch.cells.iter() {
            visited.insert(*c);
        }
        patches.push(patch);
    }
    patches
}

fn part1(patches: &[Patch]) -> usize {
    patches.iter().map(|p| p.cost()).sum()
}

fn part2(patches: &[Patch]) -> usize {
    patches.iter().map(|p| p.cost_after_discount()).sum()
}

fn main() {
    let garden = load("input.txt");
    let patches = map_garden(&garden);
    println!("Part 1: {}", part1(&patches));
    println!("Part 2: {}", part2(&patches));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let garden = load("test.txt");
        let patches = map_garden(&garden);
        assert_eq!(part1(&patches), 1930);
    }

    #[test]
    fn test_part2() {
        let garden = load("test.txt");
        let patches = map_garden(&garden);
        assert_eq!(part2(&patches), 1206);
    }
}
