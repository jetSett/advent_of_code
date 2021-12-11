use std::{collections::HashMap, hash::Hash, io::stdin};

use itertools::Itertools;
type Memoize = HashMap<(u128, u128), u128>;

fn memoized_lanternfish(remain: u128, time: u128, memoization: &mut Memoize) -> u128 {
    if time <= remain {
        return 1;
    }
    if let Some(val) = memoization.get(&(remain, time)) {
        return *val;
    }
    let result = if remain == 0 {
        memoized_lanternfish(6, time - 1, memoization)
            + memoized_lanternfish(8, time - 1, memoization)
    } else {
        memoized_lanternfish(remain - 1, time - 1, memoization)
    };
    memoization.insert((remain, time), result);
    result
}

fn ex1(starting_point: &[u128], time: u128) -> u128 {
    let mut total = 0;
    let mut memoization = Memoize::new();
    for value in starting_point {
        total += memoized_lanternfish(*value, time, &mut memoization);
    }
    total
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let entry = line
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u128>().unwrap())
        .collect_vec();

    println!("{}", ex1(&entry, 80));
    println!("{}", ex1(&entry, 256));
}

#[test]
fn test_ex() {
    let entry = [3u128, 4, 3, 1, 2];
    assert_eq!(ex1(&entry, 80), 5934);
    assert_eq!(ex1(&entry, 256), 26984457539);
}
