use std::collections::HashSet;
use std::hash::Hash;
use std::io::{stdin, BufRead};

use itertools::Itertools;

type Compartment = Vec<char>;
type Bag = (Compartment, Compartment);

type Entry = Vec<Bag>;

fn parse(lines: &[String]) -> Entry {
    lines
        .iter()
        .map(|s| {
            (
                s[..s.len() / 2].chars().collect_vec(),
                s[s.len() / 2..].chars().collect_vec(),
            )
        })
        .collect()
}

fn get_priority(c: &char) -> u64 {
    match c {
        'a'..='z' => *c as u64 - 'a' as u64 + 1,
        'A'..='Z' => *c as u64 - 'A' as u64 + 27,
        _ => unreachable!(),
    }
}

fn ex1(entry: &Entry) -> u64 {
    entry
        .iter()
        .map(|(comp1, comp2)| {
            let c1_types: HashSet<char> = comp1.iter().cloned().collect();
            let c2_types: HashSet<char> = comp2.iter().cloned().collect();
            let intersect_types = c1_types.intersection(&c2_types);
            intersect_types.map(get_priority).sum::<u64>()
        })
        .sum::<u64>()
}

fn ex2(entry: &Entry) -> u64 {
    todo!()
}

fn main() {
    let entry = parse(
        &stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .collect::<Vec<String>>(),
    );
    println!("{}\n", ex1(&entry));
    println!("{}\n", ex2(&entry));
}

#[cfg(test)]
mod test {
    use super::*;

    fn gen_input_test() -> Vec<String> {
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .into_iter()
        .map(str::to_string)
        .collect()
    }

    #[test]
    fn test_ex1() {
        let entry = parse(&gen_input_test());
        assert_eq!(ex1(&entry), 151);
    }
    #[test]
    fn test_ex2() {
        let entry = parse(&gen_input_test());
        println!("{:?}", entry);
        assert_eq!(ex2(&entry), 45000);
    }
}
