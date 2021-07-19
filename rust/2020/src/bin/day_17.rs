use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::BufRead,
};

use itertools::Itertools;

type Int = i32;
#[derive(Debug, Clone, Copy)]
enum State {
    Active,
    Inactive,
}

impl Default for State {
    fn default() -> Self {
        State::Inactive
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Coordinate {
    x: Int,
    y: Int,
    z: Int,
    w: Int,
}
impl Coordinate {
    fn neightbords_3d(&self) -> Vec<Coordinate> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .filter(|((dx, dy), dz)| dx != &0 || dy != &0 || dz != &0)
            .map(|((dx, dy), dz)| Coordinate {
                x: self.x + dx,
                y: self.y + dy,
                z: self.z + dz,
                w: self.w,
            })
            .collect()
    }
    fn neightbords_4d(&self) -> Vec<Coordinate> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(((dx, dy), dz), dw)| dx != &0 || dy != &0 || dz != &0 || dw != &0)
            .map(|(((dx, dy), dz), dw)| Coordinate {
                x: self.x + dx,
                y: self.y + dy,
                z: self.z + dz,
                w: self.w + dw,
            })
            .collect()
    }
}
#[derive(Clone)]
struct Grid {
    active_cells: HashSet<Coordinate>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            active_cells: HashSet::new(),
        }
    }

    fn set(&mut self, coord: Coordinate, state: State) {
        match state {
            State::Active => {
                self.active_cells.insert(coord);
            }
            State::Inactive => {
                self.active_cells.remove(&coord);
            }
        }
    }
    fn get(&self, coord: &Coordinate) -> State {
        if self.active_cells.contains(coord) {
            State::Active
        } else {
            State::Inactive
        }
    }

    fn apply_score(&mut self, score: HashMap<Coordinate, Int>) {
        let mut new_active_cells = HashSet::new();
        for (coord, score) in score {
            match self.get(&coord) {
                State::Active => {
                    if score == 2 || score == 3 {
                        new_active_cells.insert(coord);
                    }
                }
                State::Inactive => {
                    if score == 3 {
                        new_active_cells.insert(coord);
                    }
                }
            }
        }
        self.active_cells = new_active_cells;
    }

    fn step_3d(&mut self) {
        let mut score = HashMap::new();
        for coord in &self.active_cells {
            for neight in coord.neightbords_3d() {
                *score.entry(neight).or_insert(0) += 1;
            }
        }
        self.apply_score(score);
    }

    fn step_4d(&mut self) {
        let mut score = HashMap::new();
        for coord in &self.active_cells {
            for neight in coord.neightbords_4d() {
                *score.entry(neight).or_insert(0) += 1;
            }
        }
        self.apply_score(score);
    }
    fn count_active(&self) -> usize {
        self.active_cells.len()
    }
}

fn lines_to_grid(lines: &[String]) -> Grid {
    let mut grid = Grid::new();
    let lines = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|x| match x {
                    '#' => State::Active,
                    '.' => State::Inactive,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    for (i, l) in lines.iter().enumerate() {
        for (j, s) in l.iter().enumerate() {
            grid.set(
                Coordinate {
                    x: i as Int,
                    y: j as Int,
                    z: 0,
                    w: 0,
                },
                *s,
            );
        }
    }
    grid
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect_vec();

    let mut grid = lines_to_grid(&lines);
    let mut grid2 = grid.clone();
    for _ in 0..6 {
        grid.step_3d();
    }

    println!("{}", grid.count_active());

    for _ in 0..6 {
        grid2.step_4d();
    }
    println!("{}", grid2.count_active());
}

#[cfg(test)]
mod test {
    use super::*;

    fn grid_test() -> Grid {
        let lines = vec![".#.", "..#", "###"]
            .iter()
            .cloned()
            .map(str::to_string)
            .collect_vec();
        lines_to_grid(&lines)
    }

    #[test]
    fn test_reading() {
        let grid = grid_test();
        assert_eq!(grid.count_active(), 5);
    }

    #[test]
    fn test_part_1() {
        let mut grid = grid_test();
        for _ in 0..6 {
            grid.step_3d();
            println!("{}", grid.count_active());
        }
        assert_eq!(grid.count_active(), 112);
    }

    #[test]
    fn test_part_2() {
        let mut grid = grid_test();
        for _ in 0..6 {
            grid.step_4d();
            println!("{}", grid.count_active());
        }
        assert_eq!(grid.count_active(), 848);
    }
}
