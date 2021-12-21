use std::io::{stdin, BufRead};

use itertools::Itertools;

fn ex1(digits: &[String]) -> usize {
    digits
        .iter()
        .filter(|x| [2, 3, 4, 7].contains(&x.chars().count()))
        .count()
}

fn ex2(informations: &[String], digits: &[String]) -> usize {
    todo!()
}

fn main() {
    let data = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|x| x.split('|').map(str::to_string).collect_vec())
        .map(|x| {
            (
                x[0].split_ascii_whitespace()
                    .map(str::to_string)
                    .collect_vec(),
                x[1].split_ascii_whitespace()
                    .map(str::to_string)
                    .collect_vec(),
            )
        })
        .collect_vec();
    let digits = data.iter().map(|x| x.1.clone()).collect_vec();
    println!("{}", digits.iter().map(|x| ex1(x)).sum::<usize>());
    println!("{}", data.iter().map(|(x, y)| ex2(x, y)).sum::<usize>());
}

#[test]
fn test_ex() {
    let data = [
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ]
    .map(|x| x.split('|').map(str::to_string).collect_vec())
    .map(|x| {
        (
            x[0].split_ascii_whitespace()
                .map(str::to_string)
                .collect_vec(),
            x[1].split_ascii_whitespace()
                .map(str::to_string)
                .collect_vec(),
        )
    });

    let digits = data.iter().map(|x| x.1.clone()).collect_vec();
    let result: usize = digits.iter().map(|x| ex1(x)).sum();
    assert_eq!(result, 26);
    assert_eq!(data.iter().map(|(x, y)| ex2(x, y)).sum::<usize>(), 5353);
}
