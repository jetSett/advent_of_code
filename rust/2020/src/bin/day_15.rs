use std::collections::HashMap;
use std::io::BufRead;

fn exercise_1(numbers: &[i32], stopping_time: i32) -> i32 {
    let mut current_time = 1;
    let mut last_time_seen: HashMap<i32, i32> = HashMap::new();

    for n in &numbers[0..(numbers.len() - 1)] {
        last_time_seen.insert(*n, current_time);
        current_time += 1;
    }

    // number said at time current_time-1, NOT already inserted
    let mut previous_number = numbers[numbers.len() - 1];

    while current_time < stopping_time {
        // At this point, previous_number is not in last_time_seen for the last time
        current_time += 1;
        let last_time = *last_time_seen
            .get(&previous_number)
            .unwrap_or(&(current_time - 1));

        last_time_seen.insert(previous_number, current_time - 1);

        previous_number = current_time - 1 - last_time;
    }
    dbg!(current_time);
    dbg!(previous_number);
    previous_number
}

fn main() {
    let numbers = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", exercise_1(&numbers, 2020));
    println!("{}", exercise_1(&numbers, 30000000));
}

#[test]
fn test_exo_1() {
    assert_eq!(exercise_1(&[0, 3, 6], 10), 0);
    println!();
    assert_eq!(exercise_1(&[1, 3, 2], 2020), 1);
    assert_eq!(exercise_1(&[2, 1, 3], 2020), 10);
    assert_eq!(exercise_1(&[1, 2, 3], 2020), 27);
    assert_eq!(exercise_1(&[2, 3, 1], 2020), 78);
    assert_eq!(exercise_1(&[3, 2, 1], 2020), 438);
    assert_eq!(exercise_1(&[3, 1, 2], 2020), 1836);
}
