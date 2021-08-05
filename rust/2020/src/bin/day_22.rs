use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

type Int = u32;

#[derive(Debug, PartialEq, Eq)]
enum Player {
    P1,
    P2,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card(Int);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Deck(VecDeque<Card>);

impl Deck {
    fn score(&self) -> Int {
        let n = self.0.len();
        self.0
            .iter()
            .enumerate()
            .map(|(i, x)| ((n - i) as Int) * x.0)
            .sum()
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    fn draw(&mut self) -> Card {
        self.0.pop_front().unwrap()
    }
    fn add_bottom(&mut self, card: Card) {
        self.0.push_back(card);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    deck_p1: Deck,
    deck_p2: Deck,
}

type Configuration = (Deck, Deck);

impl Game {
    fn from_lines(lines: &[String]) -> Self {
        let mut n = lines.len();
        n = n + (n % 2);
        let p = n / 2 - 2; // Number of cards
        let deck_p1: Deck = Deck(
            lines[1..(p + 1)]
                .iter()
                .map(|x| Card(x.parse::<Int>().unwrap()))
                .collect(),
        );
        let deck_p2: Deck = Deck(
            lines[n / 2 + 1..n / 2 + 1 + p]
                .iter()
                .map(|x| Card(x.parse::<Int>().unwrap()))
                .collect(),
        );
        Game { deck_p1, deck_p2 }
    }

    fn play_turn(&mut self) {
        let card_p1 = self.deck_p1.draw();
        let card_p2 = self.deck_p2.draw();
        if card_p1 >= card_p2 {
            self.deck_p1.add_bottom(card_p1);
            self.deck_p1.add_bottom(card_p2);
        } else {
            self.deck_p2.add_bottom(card_p2);
            self.deck_p2.add_bottom(card_p1);
        }
    }
    fn play_game(&mut self) -> Player {
        while !(self.deck_p1.is_empty() || self.deck_p2.is_empty()) {
            self.play_turn();
        }
        if self.deck_p1.is_empty() {
            Player::P2
        } else {
            Player::P1
        }
    }

    fn play_recursive(&mut self) -> Player {
        self._recurs_game(HashSet::new())
    }

    fn _recurs_game(&mut self, mut seen_configurations: HashSet<Configuration>) -> Player {
        while !(self.deck_p1.is_empty() || self.deck_p2.is_empty()) {
            let current_configuration = (self.deck_p1.clone(), self.deck_p2.clone());
            if seen_configurations.contains(&current_configuration) {
                return Player::P1;
            }

            seen_configurations.insert(current_configuration);

            let card_p1 = self.deck_p1.draw();
            let card_p2 = self.deck_p2.draw();

            let winner: Player;
            if card_p1.0 < self.deck_p1.len() as Int && card_p2.0 < self.deck_p2.len() as Int {
                winner = Game {
                    deck_p1: Deck(
                        self.deck_p1
                            .0
                            .range(0..(card_p1.0 as usize))
                            .cloned()
                            .collect(),
                    ),
                    deck_p2: Deck(
                        self.deck_p2
                            .0
                            .range(0..(card_p2.0 as usize))
                            .cloned()
                            .collect(),
                    ),
                }
                ._recurs_game(seen_configurations.clone())
            } else if card_p1.0 > card_p2.0 {
                winner = Player::P1
            } else {
                winner = Player::P2
            };

            if winner == Player::P1 {
                self.deck_p1.add_bottom(card_p1);
                self.deck_p1.add_bottom(card_p2);
            } else {
                self.deck_p2.add_bottom(card_p2);
                self.deck_p2.add_bottom(card_p1);
            }
        }
        if self.deck_p1.is_empty() {
            Player::P2
        } else {
            Player::P1
        }
    }
}

fn main() {
    let lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect();
    let mut game1 = Game::from_lines(&lines);
    let mut game2 = game1.clone();
    let deck_winner = match game1.play_game() {
        Player::P1 => game1.deck_p1,
        Player::P2 => game1.deck_p2,
    };
    println!("{}", deck_winner.score());

    let deck_winner = match game2.play_recursive() {
        Player::P1 => game2.deck_p1,
        Player::P2 => game2.deck_p2,
    };
    println!("{}", deck_winner.score());
}

#[test]
fn test_parsing() {
    assert_eq!(
        Game::from_lines(&[
            "Player 1:".to_string(),
            "9".to_string(),
            "2".to_string(),
            "6".to_string(),
            "3".to_string(),
            "1".to_string(),
            "".to_string(),
            "Player 2:".to_string(),
            "5".to_string(),
            "8".to_string(),
            "4".to_string(),
            "7".to_string(),
            "10".to_string(),
            "".to_string(),
        ]),
        Game {
            deck_p1: Deck(vec![9, 2, 6, 3, 1].into_iter().map(Card).collect()),
            deck_p2: Deck(vec![5, 8, 4, 7, 10].into_iter().map(Card).collect()),
        }
    );
}

#[test]
fn test_exo1() {
    let mut game = Game {
        deck_p1: Deck(vec![9, 2, 6, 3, 1].into_iter().map(Card).collect()),
        deck_p2: Deck(vec![5, 8, 4, 7, 10].into_iter().map(Card).collect()),
    };
    let winner = game.play_game();
    assert_eq!(winner, Player::P2);
    let deck_winner = match winner {
        Player::P1 => game.deck_p1,
        Player::P2 => game.deck_p2,
    };

    assert_eq!(deck_winner.score(), 306);
}

#[test]
fn test_exo2() {
    let mut game = Game {
        deck_p1: Deck(vec![9, 2, 6, 3, 1].into_iter().map(Card).collect()),
        deck_p2: Deck(vec![5, 8, 4, 7, 10].into_iter().map(Card).collect()),
    };
    let winner = game.play_recursive();
    assert_eq!(winner, Player::P2);
    let deck_winner = match winner {
        Player::P1 => game.deck_p1,
        Player::P2 => game.deck_p2,
    };

    assert_eq!(deck_winner.score(), 291);
}

#[test]
fn test_exo2_infinite() {
    let mut game = Game {
        deck_p1: Deck(vec![43, 19].into_iter().map(Card).collect()),
        deck_p2: Deck(vec![2, 29, 14].into_iter().map(Card).collect()),
    };
    game.play_recursive();
}
