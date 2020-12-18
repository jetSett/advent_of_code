use std::io;
use std::io::BufRead;

fn parse_boarding_pass(boarding_pass: Vec<char>) -> u32 {
    let mut code = 0;
    for i in 0..10 {
        if boarding_pass[i] == 'B' {
            code += 1;
        }
        code <<= 1;
    }
    code >>= 1;
    code
}

pub fn exercise_1(codes: &[u32]) {
    println!("{}", codes.iter().max().unwrap());
}

pub fn exercise_2(codes: &mut [u32]) {
    codes.sort();
    let mut prev = codes[0];
    for i in 1..codes.len() {
        if codes[i] - prev > 1 {
            println!("{}", prev + 1);
        }
        prev = codes[i];
    }
}

fn main() {
    let mut codes: Vec<u32> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            parse_boarding_pass(
                line.unwrap()
                    .chars()
                    .map(|x| if x == 'R' { 'B' } else { x })
                    .collect(),
            )
        })
        .collect();
    exercise_1(&codes);
    exercise_2(&mut codes);
}
