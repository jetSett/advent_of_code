use std::io::BufRead;

type Int = i64;

fn exercise_1(numbers: &mut Vec<Int>) -> Int {
    let mut number_jump_1 = 0;
    let mut number_jump_3 = 1;
    numbers.push(0);
    numbers.sort_unstable();
    for i in 1..numbers.len() {
        let jump = numbers[i] - numbers[i - 1];
        if jump == 1 {
            number_jump_1 += 1;
        }
        if jump == 3 {
            number_jump_3 += 1;
        }
    }
    number_jump_1 * number_jump_3
}

fn exercise_2(numbers: &mut Vec<Int>) -> Int {
    numbers.sort_unstable();
    numbers.reverse();
    if numbers[numbers.len() - 1] != 0 {
        numbers.push(0);
    }

    let n = numbers.len();

    let mut ways_to_go_to_end: Vec<Int> = Vec::new();
    ways_to_go_to_end.resize(n, 0);
    ways_to_go_to_end[0] = 1;

    for i in 1..n {
        let mut j = 1;
        while i >= j && numbers[i - j] - numbers[i] <= 3 {
            ways_to_go_to_end[i] += ways_to_go_to_end[i - j];
            j += 1;
        }
    }

    ways_to_go_to_end[n - 1]
}

fn main() {
    let mut numbers: Vec<Int> = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|x| x.parse::<Int>())
        .map(Result::unwrap)
        .collect();
    println!("{}", exercise_1(&mut numbers));
    println!("{}", exercise_2(&mut numbers));
}

#[test]
fn test_exo_1() {
    let mut numbers = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    assert_eq!(exercise_1(&mut numbers), 7 * 5);
    let mut numbers = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    assert_eq!(exercise_1(&mut numbers), 22 * 10);
}

#[test]
fn test_exo_2() {
    let mut numbers = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    assert_eq!(exercise_2(&mut numbers), 8);
    let mut numbers = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    assert_eq!(exercise_2(&mut numbers), 19208);
}
