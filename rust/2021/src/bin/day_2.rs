use std::io::{stdin, BufRead};

use itertools::Itertools;

#[derive(Clone)]
pub enum Movement {
    Horizontal(i64),
    Vertical(i64),
}

fn ex1(movements: &[Movement]) -> i64 {
    let (mut x, mut y) = (0, 0);
    movements.iter().for_each(|m| match *m {
        Movement::Horizontal(m) => x += m,
        Movement::Vertical(m) => y += m,
    });
    x * y
}

fn ex2(movements: &[Movement]) -> i64 {
    let (mut x, mut y, mut aim) = (0, 0, 0);
    movements.iter().for_each(|m| match *m {
        Movement::Horizontal(m) => {
            x += m;
            y += aim * m
        }
        Movement::Vertical(m) => aim += m,
    });
    x * y
}

peg::parser!(
    grammar movement_parser() for str {
        rule number() -> i64
        = n:$(['0'..='9']+) {? n.parse().or(Err("i64")) }
        rule forward() -> Movement
            = "forward " n:number() {Movement::Horizontal(n)}
        rule up() -> Movement
            = "up " n:number() {Movement::Vertical(-n)}
        rule down() -> Movement
            = "down " n:number() {Movement::Vertical(n)}

        pub rule movement() -> Movement = m:(forward() / up() / down()) {m}
    }
);

fn main() {
    let instructions = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|x| movement_parser::movement(&x).unwrap())
        .collect_vec();

    println!("{}", ex1(&instructions));
    println!("{}", ex2(&instructions));
}

#[test]
fn test_ex() {
    let instructions = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ]
    .map(movement_parser::movement)
    .map(Result::unwrap);

    assert_eq!(ex1(&instructions), 150);
    assert_eq!(ex2(&instructions), 900);
}
