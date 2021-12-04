use aoc_utils::read_lines;
use std::fs::File;
use std::io::{BufReader, Lines};

type Score = u32;
type Card = [[Score; 5]; 5];

fn card_from_bufreader(lines: &mut Lines<BufReader<File>>) -> Option<Card> {
    let mut card: Card = [[0; 5]; 5];
    if let None = lines.next() {
        return None;
    }
    for i in 0..5 {
        for (j, n) in lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .enumerate()
        {
            if let Ok(num) = n.parse() {
                card[i][j] = num;
            }
        }
    }
    Some(card)
}

struct Player {
    card: Card,
    checked: [[bool; 5]; 5],
}

impl Player {
    fn from_card(card: Card) -> Player {
        Player {
            card,
            checked: [[false; 5]; 5],
        }
    }

    // Returns true if the player won.
    fn observe(&mut self, number: Score) -> Option<Score> {
        for i in 0..self.card.len() {
            for j in 0..self.card[i].len() {
                if self.card[i][j] == number {
                    if !self.checked[i][j] {
                        self.checked[i][j] = true;
                        if self.check_win(i, j) {
                            return Some(self.get_score(number));
                        }
                    }
                }
            }
        }
        None
    }

    fn check_win(&self, row: usize, col: usize) -> bool {
        let mut did_win = true;
        for v in self.checked[row] {
            if !v {
                did_win = false;
                break;
            }
        }
        if did_win {
            return true;
        }

        did_win = true;
        for v in self.checked {
            if !v[col] {
                did_win = false
            }
        }
        if did_win {
            return true;
        }
        false
    }

    fn get_score(&self, number_entered: Score) -> Score {
        let mut score = 0;
        for i in 0..self.checked.len() {
            for j in 0..self.checked[i].len() {
                if !self.checked[i][j] {
                    score += self.card[i][j];
                }
            }
        }
        return score * number_entered;
    }
}

fn main() {
    bingo();
}

fn bingo() {
    let mut players: Vec<Player> = Vec::new();
    let mut lines = read_lines("./input").unwrap();
    let bingo_string = lines.next().unwrap().unwrap();
    loop {
        match card_from_bufreader(&mut lines) {
            Some(card) => {
                players.push(Player::from_card(card));
            }
            None => {
                break;
            }
        }
    }
    let mut first_winner = false;
    for value in bingo_string.split(",") {
        let number_to_observe: Score = value.parse().unwrap();
        let mut i: usize = 0;
        while i < players.len() {
            if let Some(score) = players[i].observe(number_to_observe) {
                // This player won!
                if !first_winner {
                    println!("The first winner got score {}", score);
                    first_winner = true;
                } else if players.len() == 1 {
                    println!("The last winner got score {}", score);
                    return;
                }
                players.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
