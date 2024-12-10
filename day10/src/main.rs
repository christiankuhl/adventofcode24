use std::{
    collections::{HashMap, HashSet},
    fs,
};

type GridPoint = (i8, i8);

struct Node {
    value: i8,
    neighbours: Vec<GridPoint>,
}

struct Graph {
    nodes: HashMap<GridPoint, Node>,
    trailheads: Vec<GridPoint>,
}

impl Graph {
    fn new(path: &str) -> Self {
        let mut width: i8 = 0;
        let mut height: i8 = 0;
        let mut nodes = HashMap::new();
        let mut trailheads = Vec::new();
        fs::read_to_string(path)
            .expect("File not found")
            .split('\n')
            .enumerate()
            .for_each(|(y, s)| {
                width = s.len() as i8;
                height += 1;
                s.chars().enumerate().for_each(|(x, c)| {
                    let v = c.to_digit(10).expect("Expected a i8") as i8;
                    let p = (x as i8, y as i8);
                    nodes.insert(
                        p,
                        Node {
                            value: v,
                            neighbours: Vec::new(),
                        },
                    );
                    if v == 0 {
                        trailheads.push(p);
                    }
                })
            });
        for y in 0..height {
            for x in 0..width {
                let p = (x, y);
                let u = nodes.get(&p).expect("Inconsistent input").value;
                let mut neighbours = Vec::new();
                for dy in -1i8..2i8 {
                    for dx in -1i8..2i8 {
                        if dx == 0 && dy == 0 || dx != 0 && dy != 0 {
                            continue;
                        }
                        let q = (x + dx, y + dy);
                        let v = nodes.get(&q).map(|n| n.value).unwrap_or(-2);
                        if v - u == 1 {
                            neighbours.push(q);
                        }
                    }
                }
                nodes.get_mut(&p).expect("Inconsistent input").neighbours = neighbours;
            }
        }
        Self { nodes, trailheads }
    }
    fn collect_paths(&self, current_path: Vec<GridPoint>, paths: &mut Vec<Vec<GridPoint>>) {
        let node = self.nodes.get(current_path.last().unwrap()).unwrap();
        if node.value == 9 {
            paths.push(current_path.clone());
            return;
        }
        for n in node.neighbours.iter() {
            let mut next_path = current_path.clone();
            next_path.push(*n);
            self.collect_paths(next_path, paths);
        }
    }
}

fn hiking_trails(graph: &Graph) -> Vec<Vec<Vec<GridPoint>>> {
    let mut all_trails = Vec::new();
    for p in graph.trailheads.iter() {
        let mut paths = Vec::new();
        let current_path = vec![*p];
        graph.collect_paths(current_path, &mut paths);
        all_trails.push(paths);
    }
    all_trails
}

fn part1(trails: &[Vec<Vec<GridPoint>>]) -> usize {
    trails
        .iter()
        .map(|trails_for_head| {
            trails_for_head
                .iter()
                .map(|trail| *trail.last().unwrap())
                .collect::<HashSet<GridPoint>>()
                .len()
        })
        .sum()
}

fn part2(trails: &[Vec<Vec<GridPoint>>]) -> usize {
    trails.iter().map(|t| t.len()).sum()
}

fn main() {
    let graph = Graph::new("input.txt");
    let trails = hiking_trails(&graph);
    println!("Part 1: {}", part1(&trails));
    println!("Part 2: {}", part2(&trails));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let graph = Graph::new("test.txt");
        let trails = hiking_trails(&graph);
        assert_eq!(part1(&trails), 36);
    }

    #[test]
    fn test_part2() {
        let graph = Graph::new("test.txt");
        let trails = hiking_trails(&graph);
        assert_eq!(part2(&trails), 81);
    }
}
