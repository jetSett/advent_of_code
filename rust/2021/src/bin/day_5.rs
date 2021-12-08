use std::{
    cmp::{max, min},
    collections::HashMap,
    io::{stdin, BufRead},
};

use itertools::Itertools;

pub struct Line(i32, i32, i32, i32);

peg::parser!(
    grammar line_parser() for str {
        rule number() -> i32
        = n:$(['0'..='9']+) {? n.parse().or(Err("i32")) }

        pub rule line() -> Line = a:number() "," b:number() " -> " c:number() "," d:number(){Line(a,b,c,d)}
    }
);

fn points_in_line(line: &Line) -> Vec<(i32, i32)> {
    let vector = (-(line.0 - line.2).signum(), -(line.1 - line.3).signum());
    let (mut x, mut y) = (line.0, line.1);
    let mut points = vec![(line.2, line.3)];

    while (x, y) != (line.2, line.3) {
        points.push((x, y));
        x += vector.0;
        y += vector.1;
    }
    points
}

fn ex1(lines: &[Line]) -> usize {
    let mut points = HashMap::new();

    for l in lines.iter().filter(|l| l.0 == l.2 || l.1 == l.3) {
        for p in points_in_line(l) {
            *points.entry(p).or_insert(0) += 1;
        }
    }

    points.into_iter().filter(|(_, v)| v > &1).count()
}

fn ex2(lines: &[Line]) -> usize {
    let mut points = HashMap::new();

    for l in lines {
        for p in points_in_line(l) {
            *points.entry(p).or_insert(0) += 1;
        }
    }

    points.into_iter().filter(|(_, v)| v > &1).count()
}

fn main() {
    let lines = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|x| line_parser::line(&x).unwrap())
        .collect_vec();

    println!("{}", ex1(&lines));
    println!("{}", ex2(&lines));
}

#[test]
fn test_ex() {
    let lines = [
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ]
    .map(line_parser::line)
    .map(Result::unwrap);

    assert_eq!(ex1(&lines), 5);
    assert_eq!(ex2(&lines), 12);
}
