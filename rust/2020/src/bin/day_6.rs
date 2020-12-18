use std::collections::HashMap;
use std::io;
use std::io::BufRead;

pub fn exercise_1(lines: &[String]) {
    let mut hash_questions: HashMap<char, bool> = HashMap::new();
    let mut counter: i32 = 0;
    for line in lines {
        if line == "" {
            counter += hash_questions.keys().len() as i32;
            hash_questions.clear();
        }
        let _: Vec<Option<bool>> = line
            .chars()
            .map(|x| hash_questions.insert(x, true))
            .collect();
    }
    counter += hash_questions.keys().len() as i32;
    hash_questions.clear();

    println!("{}", counter);
}

pub fn exercise_2(lines: &[String]) {
    let mut hash_questions: HashMap<char, u32> = HashMap::new();
    let mut counter: i32 = 0;
    let mut number_attending = 0;

    let mut incr_counter = |hash_questions: &HashMap<char, u32>, number_attending| {
        for (_, val) in hash_questions.iter() {
            if val == &number_attending {
                counter += 1;
            }
        }
    };

    for line in lines {
        if line == "" {
            incr_counter(&hash_questions, number_attending);
            number_attending = 0;
            hash_questions.clear();
        } else {
            number_attending += 1;
            let _: Vec<()> = line
                .chars()
                .map(|x| {
                    let count = hash_questions.entry(x).or_insert(0);
                    *count += 1;
                })
                .collect();
        }
    }
    incr_counter(&hash_questions, number_attending);
    hash_questions.clear();

    println!("{}", counter);
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();

    exercise_1(&lines);
    exercise_2(&lines);
}
