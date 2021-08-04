use std::io;
use std::io::BufRead;

type Field = Vec<Vec<char>>;

fn test_slope(field: &Field, delta_x: i32, delta_y: i32) -> i64 {
    let n: i32 = field.len() as i32;
    let mut number_tree = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    while y < n {
        if field[y as usize][x as usize] == '#' {
            number_tree += 1;
        }
        x += delta_x;
        x %= field[0].len() as i32;
        y += delta_y;
    }
    number_tree
}

pub fn exercise_1(field: &Field) {
    println!("{}", test_slope(field, 3, 1));
}

pub fn exercise_2(field: &Field) {
    let a1 = test_slope(field, 1, 1);
    let a2 = test_slope(field, 3, 1);
    let a3 = test_slope(field, 5, 1);
    let a4 = test_slope(field, 7, 1);
    let a5 = test_slope(field, 1, 2);
    println!("{}", a1 * a2 * a3 * a4 * a5);
}

fn main() -> anyhow::Result<()> {
    let mut field: Field = Vec::new();
    for line in io::stdin().lock().lines() {
        field.push(line.unwrap().chars().collect());
    }

    exercise_1(&field);
    exercise_2(&field);

    Ok(())
}
