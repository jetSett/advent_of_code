use std::io::{stdin, BufRead};

use itertools::Itertools;

fn bool_to_u128(data: &[bool]) -> u128 {
    let mut x = 0;
    for b in data {
        x <<= 1;
        x += *b as u128;
    }
    x
}

fn ex2(data: &[Vec<bool>]) -> u128 {
    let mut data_oxygen = data.to_vec();
    let mut i = 0;

    while data_oxygen.len() > 1 {
        let count: usize = data_oxygen.iter().map(|b| b[i] as usize).sum();
        let filter = 2 * count >= data_oxygen.len();
        data_oxygen = data_oxygen.into_iter().filter(|b| b[i] == filter).collect();
        i += 1;
    }
    let oxygen = bool_to_u128(&data_oxygen[0]);

    let mut data_co2 = data.to_vec();
    let mut i = 0;

    while data_co2.len() > 1 {
        let count: usize = data_co2.iter().map(|b| b[i] as usize).sum();
        let filter = 2 * count >= data_co2.len();
        data_co2 = data_co2.into_iter().filter(|b| b[i] != filter).collect();
        i += 1;
    }
    let co2 = bool_to_u128(&data_co2[0]);
    oxygen * co2
}

fn ex1(data: &[Vec<bool>]) -> u128 {
    let mut epsilon = 0;
    let mut gamma = 0;

    let mut counter = Vec::new();
    counter.resize(data[0].len(), 0);

    for str in data {
        for i in 0..str.len() {
            counter[i] += str[i] as usize;
        }
    }
    let n = data.len();
    for i in counter {
        epsilon <<= 1;
        gamma <<= 1;
        if i < n / 2 {
            epsilon += 1;
        } else {
            gamma += 1;
        }
    }
    epsilon * gamma
}

fn main() {
    let data = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|x| x.chars().map(|y| y == '1').collect_vec())
        .collect_vec();
    println!("{}", ex1(&data));
    println!("{}", ex2(&data));
}

#[test]
fn test_ex() {
    let data = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ]
    .map(|x| x.chars().map(|y| y == '1').collect_vec());

    assert_eq!(ex1(&data), 198);
    assert_eq!(ex2(&data), 230);
}
