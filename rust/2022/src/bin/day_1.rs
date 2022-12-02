use std::io::{stdin, BufRead};

fn parse(lines: &[String]) -> Vec<Vec<u64>> {
    let mut elves_backpack = Vec::new();
    let mut current_bp = Vec::new();
    for l in lines {
        if l.is_empty() {
            elves_backpack.push(current_bp.clone());
            current_bp.clear();
        } else {
            let current_val = l.parse::<u64>().unwrap();
            current_bp.push(current_val);
        }
    }
    elves_backpack.push(current_bp);
    elves_backpack
}

fn ex1(elves_backpack: &[Vec<u64>]) -> u64 {
    elves_backpack
        .iter()
        .map(|bp| bp.iter().sum())
        .max()
        .unwrap()
}

fn ex2(elves_backpack: &[Vec<u64>]) -> u64 {
    let mut elves_total: Vec<u64> = elves_backpack.iter().map(|bp| bp.iter().sum()).collect();
    elves_total.sort_unstable();
    elves_total.reverse();
    elves_total[0] + elves_total[1] + elves_total[2]
}

fn main() {
    let elves_backpack = parse(
        &stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .collect::<Vec<String>>(),
    );
    println!("{}\n", ex1(&elves_backpack));
    println!("{}\n", ex2(&elves_backpack));
}

#[cfg(test)]
mod test {
    use super::*;

    fn gen_input_test() -> Vec<String> {
        vec![
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "4000".to_string(),
            "".to_string(),
            "5000".to_string(),
            "6000".to_string(),
            "".to_string(),
            "7000".to_string(),
            "8000".to_string(),
            "9000".to_string(),
            "".to_string(),
            "10000".to_string(),
        ]
    }

    #[test]
    fn test_ex1() {
        let entry = parse(&gen_input_test());
        assert_eq!(ex1(&entry), 24000);
    }
    #[test]
    fn test_ex2() {
        let entry = parse(&gen_input_test());
        println!("{:?}", entry);
        assert_eq!(ex2(&entry), 45000);
    }
}
