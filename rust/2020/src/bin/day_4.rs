#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::BufRead;

use std::collections::HashMap;

use regex::Regex;

fn check_password_accepted(keys: &[String]) -> bool {
    keys.contains(&"byr".to_string())
        && keys.contains(&"iyr".to_string())
        && keys.contains(&"eyr".to_string())
        && keys.contains(&"hgt".to_string())
        && keys.contains(&"hcl".to_string())
        && keys.contains(&"ecl".to_string())
        && keys.contains(&"pid".to_string())
}

pub fn exercise_1(lines: &[String]) {
    let regex_key_val = regex::Regex::new(r"(\w\w\w):[^ ]+").unwrap();
    let mut current_keys: Vec<String> = Vec::new();
    let mut accepted_password = 0;
    for line in lines {
        if line == &"".to_string() {
            if check_password_accepted(&current_keys) {
                accepted_password += 1;
            }
            current_keys.clear();
        } else {
            for matching in regex_key_val.captures_iter(line.as_str()) {
                current_keys.push(matching.get(1).unwrap().as_str().to_string());
            }
        }
    }
    if check_password_accepted(&current_keys) {
        accepted_password += 1;
    }

    println!("{}", accepted_password);
}

type KeyVal = HashMap<String, String>;

fn check_password_accepted_strict(keys: &KeyVal) -> bool {
    lazy_static! {
        static ref REGEX_YEAR: Regex = Regex::new(r"\d{4}").unwrap();
        static ref REGEX_COLOR: Regex = Regex::new(r"#[\da-f]{6}").unwrap();
        static ref VEC_COLOR: Vec<String> = vec![
            "amb".to_string(),
            "blu".to_string(),
            "brn".to_string(),
            "gry".to_string(),
            "grn".to_string(),
            "hzl".to_string(),
            "oth".to_string()
        ];
        static ref REGEX_PID: Regex = Regex::new(r"\d{9}").unwrap();
        static ref REGEX_HEIGHT: Regex = Regex::new(r"(\d+)([a-z]{2}+)").unwrap();
    }

    let byr = keys
        .get(&"byr".to_string())
        .map(|x: &String| {
            if REGEX_YEAR.is_match(x.as_str()) {
                let x: i32 = x.parse().unwrap();
                x >= 1920 && x <= 2002
            } else {
                false
            }
        })
        .unwrap_or(false);
    let iyr = keys
        .get(&"iyr".to_string())
        .map(|x: &String| {
            if REGEX_YEAR.is_match(x.as_str()) {
                let x: i32 = x.parse().unwrap();
                x >= 2010 && x <= 2020
            } else {
                false
            }
        })
        .unwrap_or(false);

    let eyr = keys
        .get(&"eyr".to_string())
        .map(|x: &String| {
            if REGEX_YEAR.is_match(x.as_str()) {
                let x: i32 = x.parse().unwrap();
                2020 <= x && x <= 2030
            } else {
                false
            }
        })
        .unwrap_or(false);

    let hcl = keys
        .get(&"hcl".to_string())
        .map(|x: &String| REGEX_COLOR.is_match(x.as_str()))
        .unwrap_or(false);

    let ecl = keys
        .get(&"ecl".to_string())
        .map(|x: &String| VEC_COLOR.contains(x))
        .unwrap_or(false);

    let pid = keys
        .get(&"pid".to_string())
        .map(|x: &String| REGEX_PID.is_match(x.as_str()))
        .unwrap_or(false);

    let hgt = keys
        .get(&"hgt".to_string())
        .map(|x: &String| {
            let capt = REGEX_HEIGHT.captures(x.as_str());
            if let Some(capt) = capt {
                let size: i32 = capt.get(1).unwrap().as_str().to_string().parse().unwrap();
                let unit = capt.get(2).unwrap().as_str().to_string();
                if &unit == "cm" {
                    150 <= size && size <= 193
                } else if &unit == "in" {
                    59 <= size && size <= 76
                } else {
                    false
                }
            } else {
                false
            }
        })
        .unwrap_or(false);

    byr && iyr && eyr && hcl && ecl && pid && hgt
}

// TODO: something is going on, I have 1 more password validated that I should...
pub fn exercise_2(lines: &[String]) {
    let regex_key_val = regex::Regex::new(r"(\w\w\w):([^ ]+)").unwrap();
    let mut current_keys: KeyVal = KeyVal::new();
    let mut accepted_password = 0;
    for line in lines {
        if line == &"".to_string() {
            if check_password_accepted_strict(&current_keys) {
                accepted_password += 1;
            }
            current_keys.clear();
        } else {
            for matching in regex_key_val.captures_iter(line.as_str()) {
                current_keys.insert(
                    matching.get(1).unwrap().as_str().to_string(),
                    matching.get(2).unwrap().as_str().to_string(),
                );
            }
        }
    }
    if check_password_accepted_strict(&current_keys) {
        accepted_password += 1;
    }

    println!("{}", accepted_password);
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();

    exercise_1(&lines);
    exercise_2(&lines);
}
