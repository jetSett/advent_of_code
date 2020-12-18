use std::io;
use std::io::BufRead;

fn check_password_1(mini: i32, maxi: i32, letter: char, password: String) -> i32 {
    let count_letter = password.chars().filter(|x| &letter == x).count();
    if count_letter >= mini as usize && count_letter <= maxi as usize {
        return 1;
    } else {
        return 0;
    }
}

pub fn exercise_1(lines: &Vec<String>) {
    let regex_input = regex::Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut counter = 0;
    for line in lines{
        let matching = regex_input.captures(line.as_str()).unwrap();
        let mini: i32 = matching
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .parse()
            .unwrap();
        let maxi: i32 = matching
            .get(2)
            .unwrap()
            .as_str()
            .to_string()
            .parse()
            .unwrap();
        let letter: char = matching
            .get(3)
            .unwrap()
            .as_str()
            .to_string()
            .parse()
            .unwrap();
        let password: String = matching.get(4).unwrap().as_str().to_string();
        counter += check_password_1(mini, maxi, letter, password);
    }
    println!("{}", counter);
}
fn check_password_2(i1: i32, i2: i32, letter: char, password: String) -> i32 {
    let letters: Vec<char> = password.chars().collect();
    return ((letters[(i1 - 1) as usize] == letter) ^ (letters[(i2 - 1) as usize] == letter))
        as i32;
}

pub fn exercise_2(lines: &Vec<String>) {
    let regex_input = regex::Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut counter = 0;
    for line in lines {
        let matching = regex_input.captures(line.as_str()).unwrap();
        let mini: i32 = matching
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .parse()
            .unwrap();
        let maxi: i32 = matching
            .get(2)
            .unwrap()
            .as_str()
            .to_string()
            .parse()
            .unwrap();
        let letter: char = matching
            .get(3)
            .unwrap()
            .as_str()
            .to_string()
            .parse()
            .unwrap();
        let password: String = matching.get(4).unwrap().as_str().to_string();
        counter += check_password_2(mini, maxi, letter, password);
    }
    println!("{}", counter);
}

fn main() -> anyhow::Result<()> {
    let lines : Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap().into()).collect();

    exercise_1(&lines);
    exercise_2(&lines);

    Ok(())
}
