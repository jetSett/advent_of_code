use std::io::BufRead;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Floor,
    Occupied,
    Empty,
}

#[derive(Clone, Copy)]
enum RuleType {
    Simple,
    Complex,
}

#[derive(Debug, Clone, PartialEq)]
struct Grid(Vec<Vec<Cell>>);

impl Grid {
    fn build(lines: &[String]) -> Self {
        Grid(
            lines
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|cell| match cell {
                            'L' => Cell::Empty,
                            '#' => Cell::Occupied,
                            '.' => Cell::Floor,
                            _ => unreachable!("Not a valid cell char"),
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn value_neight_simple(&self, x: i32, y: i32) -> i32 {
        let mut val = -Grid::value(self.0[x as usize][y as usize]);
        for i in -1..=1 {
            for j in -1..=1 {
                if (0..self.0.len() as i32).contains(&(x + i))
                    && (0..self.0[0].len() as i32).contains(&(y + j))
                {
                    val += Grid::value(self.0[(x + i) as usize][(y + j) as usize]);
                }
            }
        }
        val
    }

    fn value_neight_complex(&self, x: i32, y: i32) -> i32 {
        let mut val = 0;
        for (i, j) in [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
        ]
        .iter()
        {
            let mut x = x + i;
            let mut y = y + j;
            while (0..self.0.len() as i32).contains(&x) && (0..self.0[0].len() as i32).contains(&y)
            {
                match self.0[(x) as usize][(y) as usize] {
                    Cell::Occupied => {
                        val += 1;
                        break;
                    }
                    Cell::Empty => break,
                    _ => (),
                };
                x += i;
                y += j;
            }
        }
        val
    }

    fn next_cell(&self, x: i32, y: i32, rule: RuleType, maximal_neight: i32) -> Cell {
        let old_cell = self.0[x as usize][y as usize];
        if old_cell == Cell::Floor {
            return old_cell;
        }
        let val = match rule {
            RuleType::Simple => self.value_neight_simple(x, y),
            RuleType::Complex => self.value_neight_complex(x, y),
        };
        if val == 0 {
            Cell::Occupied
        } else if val >= maximal_neight {
            Cell::Empty
        } else {
            old_cell
        }
    }

    fn one_step(&mut self, rule: RuleType, maximal_neight: i32) -> bool {
        let mut modified = false;
        let mut new_grid = Grid(vec![]);
        for (x, line) in self.0.iter().enumerate() {
            new_grid.0.push(vec![]);
            for (y, cell) in line.iter().enumerate() {
                let new_cell = self.next_cell(x as i32, y as i32, rule, maximal_neight);
                modified |= &new_cell != cell;
                new_grid.0[x].push(new_cell);
            }
        }
        self.0 = new_grid.0;
        modified
    }

    fn value(cell: Cell) -> i32 {
        match cell {
            Cell::Floor | Cell::Empty => 0,
            Cell::Occupied => 1,
        }
    }

    fn count_occupied(&self) -> usize {
        self.0
            .iter()
            .cloned()
            .map(|line| line.iter().filter(|x| *x == &Cell::Occupied).count())
            .sum()
    }

    fn _print(&self) {
        for line in &self.0 {
            for x in line {
                print!(
                    "{}",
                    match x {
                        Cell::Empty => 'L',
                        Cell::Floor => '.',
                        Cell::Occupied => '#',
                    }
                )
            }
            println!();
        }
    }
}

fn exercise_1(mut grid: Grid) -> i32 {
    while grid.one_step(RuleType::Simple, 4) {}
    grid.count_occupied() as i32
}

fn exercise_2(mut grid: Grid) -> i32 {
    while grid.one_step(RuleType::Complex, 5) {}
    grid.count_occupied() as i32
}

fn main() {
    let grid = Grid::build(
        &std::io::stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .collect::<Vec<_>>(),
    );
    println!("{}", exercise_1(grid.clone()));
    println!("{}", exercise_2(grid));
}

#[test]
fn test_one_step_simple() {
    let mut grid1 = Grid::build(&vec![
        "#.LL.L#.##".into(),
        "#LLLLLL.L#".into(),
        "L.L.L..L..".into(),
        "#LLL.LL.L#".into(),
        "#.LL.LL.LL".into(),
        "#.LLLL#.##".into(),
        "..L.L.....".into(),
        "#LLLLLLLL#".into(),
        "#.LLLLLL.L".into(),
        "#.#LLLL.##".into(),
    ]);
    grid1.one_step(RuleType::Simple, 4);
    grid1._print();
    let grid2 = Grid::build(&vec![
        "#.##.L#.##".into(),
        "#L###LL.L#".into(),
        "L.#.#..#..".into(),
        "#L##.##.L#".into(),
        "#.##.LL.LL".into(),
        "#.###L#.##".into(),
        "..#.#.....".into(),
        "#L######L#".into(),
        "#.LL###L.L".into(),
        "#.#L###.##".into(),
    ]);
    println!();
    grid2._print();
    assert_eq!(grid1, grid2);
}
#[test]
fn test_one_step_complex() {
    let mut grid1 = Grid::build(&vec![
        "#.L#.##.L#".into(),
        "#L#####.LL".into(),
        "L.#.#..#..".into(),
        "##L#.##.##".into(),
        "#.##.#L.##".into(),
        "#.#####.#L".into(),
        "..#.#.....".into(),
        "LLL####LL#".into(),
        "#.L#####.L".into(),
        "#.L####.L#".into(),
    ]);
    grid1.one_step(RuleType::Complex, 5);
    grid1._print();
    let grid2 = Grid::build(&vec![
        "#.L#.L#.L#".into(),
        "#LLLLLL.LL".into(),
        "L.L.L..#..".into(),
        "##LL.LL.L#".into(),
        "L.LL.LL.L#".into(),
        "#.LLLLL.LL".into(),
        "..L.L.....".into(),
        "LLLLLLLLL#".into(),
        "#.LLLLL#.L".into(),
        "#.L#LL#.L#".into(),
    ]);
    println!();
    grid2._print();
    assert_eq!(grid1, grid2);
}

#[test]
fn test_exo_1() {
    let grid = Grid::build(&vec![
        "L.LL.LL.LL".into(),
        "LLLLLLL.LL".into(),
        "L.L.L..L..".into(),
        "LLLL.LL.LL".into(),
        "L.LL.LL.LL".into(),
        "L.LLLLL.LL".into(),
        "..L.L.....".into(),
        "LLLLLLLLLL".into(),
        "L.LLLLLL.L".into(),
        "L.LLLLL.LL".into(),
    ]);
    assert_eq!(exercise_1(grid), 37);
}
#[test]
fn test_exo_2() {
    let grid = Grid::build(&vec![
        "L.LL.LL.LL".into(),
        "LLLLLLL.LL".into(),
        "L.L.L..L..".into(),
        "LLLL.LL.LL".into(),
        "L.LL.LL.LL".into(),
        "L.LLLLL.LL".into(),
        "..L.L.....".into(),
        "LLLLLLLLLL".into(),
        "L.LLLLLL.L".into(),
        "L.LLLLL.LL".into(),
    ]);
    assert_eq!(exercise_2(grid), 26);
}
