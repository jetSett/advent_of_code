#![feature(box_syntax)]

use std::io::BufRead;

use itertools::Itertools;

type Int = u64;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Number(Int),
    Addition(Box<Operation>, Box<Operation>),
    Multiplication(Box<Operation>, Box<Operation>),
}

impl Operation {
    fn evaluate(&self) -> Int {
        use Operation::*;
        match self {
            Number(x) => *x,
            Addition(x, y) => x.evaluate() + y.evaluate(),
            Multiplication(x, y) => x.evaluate() * y.evaluate(),
        }
    }
}

peg::parser! {
grammar operation_parser_pt1() for str {
    rule number() -> Operation
        = n:$(['0'..='9']+) { Operation::Number(n.parse().unwrap()) }

    pub rule operation() -> Operation =
        precedence!{
            x:(@) " + " y:@ { Operation::Addition(box x, box y) }
            x:(@) " * " y:@ { Operation::Multiplication(box x, box y) }
            n:number() { n }
            --
            "(" e:operation() ")" { e }
        }
    }
}
peg::parser! {

    grammar operation_parser_pt2() for str {
        rule number() -> Operation
            = n:$(['0'..='9']+) { Operation::Number(n.parse().unwrap()) }

        pub rule operation() -> Operation =
            precedence!{
                x:(@) " * " y:@ { Operation::Multiplication(box x, box y) }
                --
                x:(@) " + " y:@ { Operation::Addition(box x, box y) }
                n:number() { n }
                --
                "(" e:operation() ")" { e }
            }
        }
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect_vec();

    let pt1 = lines
        .iter()
        .map(|x| operation_parser_pt1::operation(&x))
        .map(Result::unwrap)
        .collect_vec();
    let pt2 = lines
        .iter()
        .map(|x| operation_parser_pt2::operation(&x))
        .map(Result::unwrap)
        .collect_vec();

    let evaluations_pt1 = pt1.iter().map(|x| x.evaluate()).collect_vec();
    println!("{}", evaluations_pt1.iter().sum::<Int>());
    let evaluations_pt2 = pt2.iter().map(|x| x.evaluate()).collect_vec();
    println!("{}", evaluations_pt2.iter().sum::<Int>());
}

#[test]
fn test_parsing_pt1() -> anyhow::Result<()> {
    use Operation::*;
    assert_eq!(
        operation_parser_pt1::operation("3 + 4")?,
        Addition(box Number(3), box Number(4))
    );
    assert_eq!(
        operation_parser_pt1::operation("1 + 2 + 3")?,
        Addition(box Addition(box Number(1), box Number(2)), box Number(3))
    );
    assert_eq!(
        operation_parser_pt1::operation("1 + (2 + 3)")?,
        Addition(box Number(1), box Addition(box Number(2), box Number(3)))
    );
    assert_eq!(
        operation_parser_pt1::operation("1 + 2 * 3")?,
        Multiplication(box Addition(box Number(1), box Number(2)), box Number(3))
    );
    assert_eq!(
        operation_parser_pt1::operation("1 * 2 + 3")?,
        Addition(
            box Multiplication(box Number(1), box Number(2)),
            box Number(3)
        )
    );
    Ok(())
}

#[test]
fn test_evaluate_pt1() -> anyhow::Result<()> {
    assert_eq!(
        operation_parser_pt1::operation("2 * 3 + (4 * 5)")?.evaluate(),
        26
    );
    assert_eq!(
        operation_parser_pt1::operation("5 + (8 * 3 + 9 + 3 * 4 * 3)")?.evaluate(),
        437
    );
    assert_eq!(
        operation_parser_pt1::operation("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")?.evaluate(),
        12240
    );
    assert_eq!(
        operation_parser_pt1::operation("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")?
            .evaluate(),
        13632
    );
    Ok(())
}

#[test]
fn test_parsing_pt2() -> anyhow::Result<()> {
    use Operation::*;
    assert_eq!(
        operation_parser_pt2::operation("3 + 4")?,
        Addition(box Number(3), box Number(4))
    );
    assert_eq!(
        operation_parser_pt2::operation("1 + 2 + 3")?,
        Addition(box Addition(box Number(1), box Number(2)), box Number(3))
    );
    assert_eq!(
        operation_parser_pt2::operation("1 + (2 + 3)")?,
        Addition(box Number(1), box Addition(box Number(2), box Number(3)))
    );
    assert_eq!(
        operation_parser_pt2::operation("1 + 2 * 3")?,
        Multiplication(box Addition(box Number(1), box Number(2)), box Number(3))
    );
    assert_eq!(
        operation_parser_pt2::operation("1 * 2 + 3")?,
        Multiplication(box Number(1), box Addition(box Number(2), box Number(3)),)
    );
    Ok(())
}

#[test]
fn test_evaluate_pt2() -> anyhow::Result<()> {
    assert_eq!(
        operation_parser_pt2::operation("1 + (2 * 3) + (4 * (5 + 6))")?.evaluate(),
        51
    );
    assert_eq!(
        operation_parser_pt2::operation("2 * 3 + (4 * 5)")?.evaluate(),
        46
    );
    assert_eq!(
        operation_parser_pt2::operation("5 + (8 * 3 + 9 + 3 * 4 * 3)")?.evaluate(),
        1445
    );
    assert_eq!(
        operation_parser_pt2::operation("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")?.evaluate(),
        669060
    );
    assert_eq!(
        operation_parser_pt2::operation("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")?
            .evaluate(),
        23340
    );
    Ok(())
}
