use std::io::{stdin, BufRead};

#[derive(Debug, PartialEq, Clone)]
enum GameMoveOpponent {
    A,
    B,
    C,
}

#[derive(Debug, PartialEq, Clone)]
enum GameMoveMe {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Clone)]
enum RockPaperScisor {
    Rock,
    Paper,
    Scisor,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

type Hint = (GameMoveOpponent, GameMoveMe);

type Entry = Vec<Hint>;

fn parse(lines: &[String]) -> Entry {
    lines
        .iter()
        .map(|l| {
            let first = match l.chars().nth(0).unwrap() {
                'A' => GameMoveOpponent::A,
                'B' => GameMoveOpponent::B,
                'C' => GameMoveOpponent::C,
                _ => unreachable!(),
            };
            let second = match l.chars().nth(2).unwrap() {
                'X' => GameMoveMe::X,
                'Y' => GameMoveMe::Y,
                'Z' => GameMoveMe::Z,
                _ => unreachable!(),
            };
            (first, second)
        })
        .collect()
}

fn who_win(opponent: RockPaperScisor, me: RockPaperScisor) -> Outcome {
    use RockPaperScisor::*;
    match (opponent, me) {
        (Rock, Rock) | (Paper, Paper) | (Scisor, Scisor) => Outcome::Draw,
        (Rock, Scisor) | (Scisor, Paper) | (Paper, Rock) => Outcome::Lose,
        (Scisor, Rock) | (Paper, Scisor) | (Rock, Paper) => Outcome::Win,
    }
}

fn gmo_to_rpc(g: GameMoveOpponent) -> RockPaperScisor {
    use GameMoveOpponent::*;
    use RockPaperScisor::*;
    match g {
        A => Rock,
        B => Paper,
        C => Scisor,
    }
}
fn gmm_to_rpc(g: GameMoveMe) -> RockPaperScisor {
    use GameMoveMe::*;
    use RockPaperScisor::*;
    match g {
        X => Rock,
        Y => Paper,
        Z => Scisor,
    }
}

fn score_outcome(out: Outcome) -> u64 {
    match out {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    }
}

fn score_choice(rpc: RockPaperScisor) -> u64 {
    match rpc {
        RockPaperScisor::Rock => 1,
        RockPaperScisor::Paper => 2,
        RockPaperScisor::Scisor => 3,
    }
}

fn evaluate_match(opp: RockPaperScisor, me: RockPaperScisor) -> u64 {
    score_choice(me.clone()) + score_outcome(who_win(opp, me))
}

fn ex1(entry: &Entry) -> u64 {
    entry
        .clone()
        .into_iter()
        .map(|(gmo, gmm)| (gmo_to_rpc(gmo), gmm_to_rpc(gmm)))
        .map(|(x, y)| evaluate_match(x, y))
        .sum::<u64>()
}

fn find_my_choice(opp: RockPaperScisor, out: Outcome) -> RockPaperScisor {
    use Outcome::*;
    use RockPaperScisor::*;
    match (opp, out) {
        (x, Draw) => x,
        (Rock, Win) | (Scisor, Lose) => Paper,
        (Rock, Lose) | (Paper, Win) => Scisor,
        (Scisor, Win) | (Paper, Lose) => Rock,
    }
}

fn gmm_to_outcome(g: GameMoveMe) -> Outcome {
    use GameMoveMe::*;
    use Outcome::*;
    match g {
        X => Lose,
        Y => Draw,
        Z => Win,
    }
}

fn ex2(entry: &Entry) -> u64 {
    entry
        .clone()
        .into_iter()
        .map(|(gmo, gmm)| (gmo_to_rpc(gmo), gmm_to_outcome(gmm)))
        .map(|(opp, out)| {
            let me = find_my_choice(opp.clone(), out);
            (opp, me)
        })
        .map(|(x, y)| evaluate_match(x, y))
        .sum::<u64>()
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
        vec!["A Y", "B X", "C Z"]
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn test_ex1() {
        let entry = parse(&gen_input_test());
        assert_eq!(ex1(&entry), 15);
    }
    #[test]
    fn test_ex2() {
        let entry = parse(&gen_input_test());
        println!("{:?}", entry);
        assert_eq!(ex2(&entry), 12);
    }
}
