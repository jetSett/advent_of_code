#![feature(destructuring_assignment)]
use std::io::BufRead;

peg::parser! {
grammar instruction_parser() for str {
    rule number() -> i32
      = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule north() -> Instruction
       = "N" n:number() {Instruction::N(n)}
    rule south() -> Instruction
       = "S" n:number() {Instruction::S(n)}
    rule east() -> Instruction
       = "E" n:number() {Instruction::E(n)}
    rule west() -> Instruction
       = "W" n:number() {Instruction::W(n)}
    rule left() -> Instruction
       = "L" n:number() {Instruction::L(n)}
    rule right() -> Instruction
       = "R" n:number() {Instruction::R(n)}
    rule forward() -> Instruction
       = "F" n:number() {Instruction::F(n)}
    pub rule instruction() -> Instruction
       = inst:(north()/south()/east()/west()/left()/right()/forward()) {inst}

}}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
    facing: Direction,
}

fn rotate(mut facing: Direction, angle: i32) -> Direction {
    // trigo sense
    if angle % 90 != 0 {
        panic!("Should not do non-squared angles");
    }

    let mut number_round = angle / 90;
    while number_round != 0 {
        facing = match (facing, number_round > 0) {
            (Direction::East, true) | (Direction::West, false) => Direction::North,
            (Direction::West, true) | (Direction::East, false) => Direction::South,
            (Direction::North, true) | (Direction::South, false) => Direction::West,
            (Direction::South, true) | (Direction::North, false) => Direction::East,
        };
        if number_round < 0 {
            number_round += 1;
        } else {
            number_round -= 1;
        }
    }
    // dbg!(facing);
    facing
}

fn execute_instruction_1(pos: &Position, inst: &Instruction) -> Position {
    let mut pos = pos.clone();
    match (inst, pos.facing) {
        (Instruction::N(x), _) | (Instruction::F(x), Direction::North) => pos.y += x,
        (Instruction::S(x), _) | (Instruction::F(x), Direction::South) => pos.y -= x,
        (Instruction::E(x), _) | (Instruction::F(x), Direction::East) => pos.x += x,
        (Instruction::W(x), _) | (Instruction::F(x), Direction::West) => pos.x -= x,
        (Instruction::L(x), _) => pos.facing = rotate(pos.facing, *x),
        (Instruction::R(x), _) => pos.facing = rotate(pos.facing, -*x),
    }
    pos
}

fn exercise_1(instructions: &[Instruction]) -> i32 {
    let mut current_position = Position {
        x: 0,
        y: 0,
        facing: Direction::East,
    };
    for inst in instructions {
        current_position = execute_instruction_1(&current_position, inst);
    }

    current_position.x.abs() + current_position.y.abs()
}

fn execute_instruction_2(
    (pos, target): &(Position, Position),
    inst: &Instruction,
) -> (Position, Position) {
    let (mut pos, mut target) = (pos.clone(), target.clone());
    match inst {
        Instruction::N(x) => target.y += x,
        Instruction::S(x) => target.y -= x,
        Instruction::E(x) => target.x += x,
        Instruction::W(x) => target.x -= x,
        Instruction::L(mut x) => {
            if x % 90 != 0 {
                panic!("Should not do non-squared angles");
            }
            x /= 90;
            for _ in 0..x {
                (target.x, target.y) = (-target.y, target.x)
            }
        }
        Instruction::R(mut x) => {
            if x % 90 != 0 {
                panic!("Should not do non-squared angles");
            }
            x /= 90;
            for _ in 0..x {
                (target.x, target.y) = (target.y, -target.x)
            }
        }
        Instruction::F(n) => {
            pos.x += n * target.x;
            pos.y += n * target.y;
        }
    }
    (pos, target)
}

fn exercise_2(instructions: &[Instruction]) -> i32 {
    let (mut current_position, mut target) = (
        Position {
            x: 0,
            y: 0,
            facing: Direction::East,
        },
        Position {
            x: 10,
            y: 1,
            facing: Direction::East,
        },
    );
    for inst in instructions {
        (current_position, target) = execute_instruction_2(&(current_position, target), inst);
    }

    current_position.x.abs() + current_position.y.abs()
}

fn main() {
    let instructions = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|x| instruction_parser::instruction(&x).unwrap())
        .collect::<Vec<_>>();

    println!("{}", exercise_1(&instructions));
    println!("{}", exercise_2(&instructions));
}

#[test]
fn test_parser() {
    use Instruction::*;
    assert_eq!(
        vec![F(10), N(3), F(7), R(90), F(11)],
        vec!["F10", "N3", "F7", "R90", "F11"]
            .iter()
            .cloned()
            .map(instruction_parser::instruction)
            .map(Result::unwrap)
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_exo1() {
    let instructions = vec!["F10", "N3", "F7", "R90", "F11"]
        .iter()
        .cloned()
        .map(instruction_parser::instruction)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    assert_eq!(exercise_1(&instructions), 25);
}
#[test]
fn test_exo2() {
    let instructions = vec!["F10", "N3", "F7", "R90", "F11"]
        .iter()
        .cloned()
        .map(instruction_parser::instruction)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    assert_eq!(exercise_2(&instructions), 286);
}
