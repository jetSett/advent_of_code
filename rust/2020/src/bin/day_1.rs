use std::io;
use std::io::BufRead;

fn exercise_1(numbers: &mut Vec<u32>) {
    numbers.sort();
    numbers.reverse();
    for i in 0..numbers.len() {
        let mut j = numbers.len() - 1;
        while j > i && numbers[i] + numbers[j] < 2020 {
            j -= 1;
        }
        if numbers[i] + numbers[j] == 2020 {
            println!(
                "Found ! {} + {} => {}",
                numbers[i],
                numbers[j],
                numbers[i] * numbers[j]
            );
        }
    }
}

fn exercise_2(numbers: &mut Vec<u32>) {
    numbers.sort();
    let n = numbers.len();
    for i in 0..numbers.len() {
        let index = n - i - 1;
        let mut j = 0;
        while j < index && numbers[index] + numbers[j] <= 2020 {
            let target = 2020 - (numbers[index] + numbers[j]);
            if numbers.binary_search(&target).is_ok() {
                println!(
                    "Found : {} + {} + {} => {}",
                    numbers[index],
                    numbers[j],
                    target,
                    numbers[index] * numbers[j] * target
                );
            }
            j += 1;
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut numbers: Vec<u32> = vec![];
    for line in io::stdin().lock().lines() {
        numbers.push(line.unwrap().parse::<u32>().unwrap());
    }

    exercise_1(&mut numbers);
    exercise_2(&mut numbers);
    Ok(())
}
