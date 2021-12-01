use std::io::stdin;
use std::io::BufRead;

use itertools::Itertools;

fn ex1(measures: &[i32]) -> u32 {
    let mut counter = 0;

    for i in 1..measures.len() {
        if measures[i] > measures[i - 1] {
            counter += 1;
        }
    }
    counter
}

fn ex2(measures: &[i32]) -> u32 {
    let avg_measures: Vec<_> = measures
        .iter()
        .cloned()
        .chain([0, 0].into_iter())
        .tuple_windows::<(i32, i32, i32)>()
        .map(|(a, u, i)| a + u + i)
        .collect();
    ex1(&avg_measures)
}

fn main() {
    let measures: Vec<i32> = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|x| str::parse::<i32>(&x).unwrap())
        .collect();

    println!("{}", ex1(&measures));
    println!("{}", ex2(&measures));
}

#[test]
fn test_ex1() {
    let measures = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(ex1(&measures), 7);
}

#[test]
fn test_ex2() {
    let measures = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(ex2(&measures), 5);
}
