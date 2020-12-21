use std::io::BufRead;

type Range = std::ops::Range<usize>;

// use advent_of_code_2020::RingVec;

type Int = i64;

fn contains(tab: &[Int], elt: &Int) -> bool {
    tab.iter().filter(|x| *x == elt).count() > 0
}

fn exercise_1(numbers: &[Int], size_prelude: usize) -> Option<Int> {
    let mut combinations: Vec<Vec<Int>> = Vec::new();

    for i in 0..numbers.len() {
        combinations.push(Vec::new());
        let current_number = numbers[i];
        let mut is_sum = i < size_prelude;
        for j in 1..(size_prelude + 1).min(i + 1) {
            let index = i - j;
            is_sum |= contains(&combinations[index], &current_number);

            combinations[index].push(numbers[i - j] + current_number);
        }
        if !is_sum {
            return Some(current_number);
        }
    }

    None
}

fn find_range(numbers: &[Int], target: Int) -> Option<Range> {
    let mut current_sum = numbers[0] + numbers[1];

    let mut current_range = Range { start: 0, end: 2 };

    while current_range.end < numbers.len() && !current_range.is_empty() {
        match target.cmp(&current_sum) {
            std::cmp::Ordering::Equal => return Some(current_range),
            std::cmp::Ordering::Greater => {
                current_sum += numbers[current_range.end];
                current_range.end += 1;
            }
            std::cmp::Ordering::Less => {
                current_sum -= numbers[current_range.start];
                current_range.start += 1;
            }
        }
    }

    None
}

fn exercise_2(numbers: &[Int], size_prelude: usize) -> Option<Int> {
    let target = exercise_1(numbers, size_prelude)?;
    let range = find_range(numbers, target)?;

    Some(numbers[range.clone()].iter().copied().min()? + numbers[range].iter().copied().max()?)
}

fn main() {
    let numbers: Vec<Int> = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|x| x.parse::<Int>())
        .map(Result::unwrap)
        .collect();

    println!("{}", exercise_1(&numbers, 25).unwrap());
    println!("{}", exercise_2(&numbers, 25).unwrap());
}

#[test]
fn test_exo_1() {
    let numbers = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    assert_eq!(exercise_1(&numbers, 5), Some(127));
}
#[test]
fn test_exo_2() {
    let numbers = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    assert_eq!(exercise_2(&numbers, 5), Some(62));
}
