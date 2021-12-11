use std::io::stdin;

use itertools::Itertools;

fn ex1(entry: &[u32]) -> u32 {
    let mut entry = entry.to_vec();
    entry.sort_unstable();
    let median = entry[entry.len() / 2];
    entry
        .iter()
        .map(|x| (*x as i32 - median as i32).abs() as u32)
        .sum()
}
fn ex2(entry: &[u32]) -> u32 {
    let mut entry = entry.to_vec();
    entry.sort_unstable();
    let mut minimal = u32::MAX;
    for target in entry[0]..=entry[entry.len() - 1] {
        let fuel = entry
            .iter()
            .map(|x| (*x as i32 - target as i32).abs() as u32)
            .map(|x| x * (x + 1) / 2)
            .sum();
        minimal = minimal.min(fuel);
    }
    minimal
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let entry = line
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect_vec();
    println!("{}", ex1(&entry));
    println!("{}", ex2(&entry));
}

#[test]
fn test_ex() {
    assert_eq!(ex1(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 37);
    assert_eq!(ex2(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 168);
}
