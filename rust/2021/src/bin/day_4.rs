use std::io::{stdin, BufRead};

use itertools::Itertools;

#[derive(Clone)]
enum GridContent {
    Open(u32),
    Closed(u32),
}

#[derive(Clone)]
struct Grid {
    matrix: Vec<Vec<GridContent>>,
}

impl Grid {
    fn from_str(data: &[String]) -> Self {
        Grid {
            matrix: data
                .iter()
                .map(|str| {
                    str.split_ascii_whitespace()
                        .map(str::parse::<u32>)
                        .map(Result::unwrap)
                        .map(GridContent::Open)
                        .collect_vec()
                })
                .collect_vec(),
        }
    }

    fn is_won(&self) -> bool {
        for i in 0..self.matrix.len() {
            let mut won_segment = true;
            for j in 0..self.matrix.len() {
                if let GridContent::Open(_) = self.matrix[i][j] {
                    won_segment = false;
                }
            }
            if won_segment {
                return true;
            }
        }
        for j in 0..self.matrix.len() {
            let mut won_segment = true;
            for i in 0..self.matrix.len() {
                if let GridContent::Open(_) = self.matrix[i][j] {
                    won_segment = false;
                }
            }
            if won_segment {
                return true;
            }
        }
        false
    }
    fn all_unmarked(&self) -> Vec<u32> {
        self.matrix
            .iter()
            .cloned()
            .concat()
            .iter()
            .filter(|x| matches!(x, GridContent::Open(_)))
            .map(|x| match x {
                GridContent::Open(x) | GridContent::Closed(x) => *x,
            })
            .collect_vec()
    }
    fn remove_number(&mut self, x: u32) {
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix.len() {
                if let GridContent::Open(y) = self.matrix[i][j] {
                    if y == x {
                        self.matrix[i][j] = GridContent::Closed(x)
                    }
                }
            }
        }
    }
}

fn read_in(data: &[String]) -> (Vec<u32>, Vec<Grid>) {
    let numbers = data[0]
        .split(',')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect_vec();
    let mut grids = Vec::new();
    for i in 0..(data.len() - 2) / 6 {
        grids.push(Grid::from_str(&data[(2 + i * 6)..(1 + (i + 1) * 6)]));
    }

    (numbers, grids)
}

fn ex1(numbers: &[u32], mut grids: Vec<Grid>) -> u32 {
    for number in numbers {
        grids
            .iter_mut()
            .for_each(|grid| grid.remove_number(*number));
        for grid in grids.iter() {
            if grid.is_won() {
                return number * (grid.all_unmarked().into_iter().sum::<u32>()) as u32;
            }
        }
    }
    unreachable!();
}

fn ex2(numbers: &[u32], mut grids: Vec<Grid>) -> u32 {
    for number in numbers {
        grids
            .iter_mut()
            .for_each(|grid| grid.remove_number(*number));
        grids = grids
            .into_iter()
            .filter(|grid| !grid.is_won())
            .collect_vec();
        if grids.len() == 1 {
            break;
        }
    }
    for number in numbers {
        grids[0].remove_number(*number);
        if grids[0].is_won() {
            return number * (grids[0].all_unmarked().into_iter().sum::<u32>()) as u32;
        }
    }
    unreachable!();
}

fn main() {
    let data = stdin().lock().lines().map(Result::unwrap).collect_vec();
    let (numbers, grids) = read_in(&data);
    println!("{}", ex1(&numbers, grids.clone()));
    println!("{}", ex2(&numbers, grids));
}

#[test]
fn test_ex() {
    let (numbers, grids) = read_in(
        &[
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
            "",
        ]
        .map(str::to_string),
    );
    assert_eq!(ex1(&numbers, grids.clone()), 4512);
    assert_eq!(ex2(&numbers, grids), 1924)
}
