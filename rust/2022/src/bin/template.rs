use std::io::{stdin, BufRead};

type Entry = ();

fn parse(lines: &[String]) -> Entry {
    todo!()
}

fn ex1(entry: &Entry) -> u64 {
    todo!()
}

fn ex2(entry: &Entry) -> u64 {
    todo!()
}

fn main() {
    let entry = parse(
        &stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .collect::<Vec<String>>(),
    );
    println!("{}\n", ex1(&entry));
    println!("{}\n", ex2(&entry));
}

#[cfg(test)]
mod test {
    use super::*;

    fn gen_input_test() -> Vec<String> {
        vec![].into_iter().map(str::to_string).collect()
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
