use crate::card::Card;
use rand::seq::SliceRandom;

pub struct Game {
    pub board: Vec<Vec<Card>>,
}

impl Game {
    pub fn new(size: usize) -> Self {
        let values = ('A'..='Z').take(size * size / 2).collect::<Vec<_>>();
        let mut cards = values
            .iter()
            .flat_map(|&value| {
                vec![
                    Card {
                        value,
                        is_matched: false
                    };
                    2
                ]
            })
            .collect::<Vec<_>>();
        cards.shuffle(&mut rand::thread_rng());

        let board = cards
            .chunks(size)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>();

        Game { board }
    }
}

impl Game {
    pub fn display_board(&self) {
        for row in &self.board {
            for card in row {
                if card.is_matched {
                    print!("  ");
                } else {
                    print!("{} ", card.value);
                }
            }
            println!();
        }
    }
}

impl Game {
    pub fn select_card(&mut self, row: usize, col: usize) {
        let selected_card = &mut self.board[row - 1][col - 1];
        if !selected_card.is_matched {
            selected_card.is_matched = true;
            self.display_board(); // Show the selected card
        } else {
            println!("Card already matched. Choose another card.");
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|card| card.is_matched))
    }
}
