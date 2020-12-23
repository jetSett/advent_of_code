use std::io::BufRead;

use itertools::zip;
use num::Integer;

type Int = i128;

fn exercise_1(start: Int, buses: &[Option<Int>]) -> Int {
    // remaining time, bus id
    let remaining_time: Vec<(Int, Int)> = buses
        .iter()
        .cloned()
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(|bus_id| (bus_id - start % bus_id, bus_id))
        .collect();

    let (time, bus) = remaining_time.iter().min().unwrap();
    time * bus
}

fn crt(targets: &[Int], modulos: &[Int]) -> Option<Int> {
    let mut current_modulus = 1;
    let mut current_value = 0;
    for (target, modulus) in zip(targets.iter(), modulos.iter()) {
        let gcd = current_modulus.extended_gcd(&modulus);
        assert!(gcd.x * current_modulus + gcd.y * modulus == gcd.gcd);
        if gcd.gcd != 1 {
            return None;
        }
        current_value = target * gcd.x * current_modulus + current_value * gcd.y * modulus;
        current_modulus *= modulus;
        current_value = current_value.rem_euclid(current_modulus);
    }
    Some(current_value)
}

fn exercise_2(buses: &[Option<Int>]) -> Int {
    let (target, modulo): (Vec<Int>, Vec<Int>) = buses
        .iter()
        .cloned()
        .enumerate()
        .filter(|x| x.1.is_some())
        .map(|(n, x)| ((x.unwrap() - n as Int), x.unwrap()))
        .unzip();
    dbg!(&target, &modulo);
    crt(&target, &modulo).expect("CRT algorithm could not conclude")
}

fn parse_string(line: &str) -> Vec<Option<Int>> {
    line.split(',').map(|x| x.parse().ok()).collect()
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let start: Int = lines.next().unwrap().map(|x| x.parse().unwrap()).unwrap();
    let buses = parse_string(&lines.next().unwrap().unwrap());

    println!("{}", exercise_1(start, &buses));
    println!("{}", exercise_2(&buses));
}

#[test]
fn test_parse_string() {
    assert_eq!(
        parse_string("7,13,x,x,59,x,31,19"),
        vec![
            Some(7),
            Some(13),
            None,
            None,
            Some(59),
            None,
            Some(31),
            Some(19)
        ]
    );
}

#[test]
fn test_exo1() {
    assert_eq!(exercise_1(939, &parse_string("7,13,x,x,59,x,31,19")), 295);
}

#[test]
fn test_exo2() {
    assert_eq!(exercise_2(&parse_string("7,13,x,x,59,x,31,19")), 1068781);
    assert_eq!(exercise_2(&parse_string("17,x,13,19")), 3417);
    assert_eq!(exercise_2(&parse_string("67,7,59,61")), 754018);
    assert_eq!(exercise_2(&parse_string("67,x,7,59,61")), 779210);
    assert_eq!(exercise_2(&parse_string("67,7,x,59,61")), 1261476);
    assert_eq!(exercise_2(&parse_string("1789,37,47,1889")), 1202161486);
}

#[test]
fn test_crt() {
    let primes = [17, 13, 19, 61];
    let total_mod: Int = primes.iter().product();
    let numbers = [5000, 15698, 123548, 123549, 13215687, 12358914, 985621];
    for n in numbers.iter() {
        let targets = primes.iter().map(|x| n % x).collect::<Vec<Int>>();
        assert_eq!(crt(&targets, &primes).unwrap(), n % total_mod);
    }
}
