use std::io;

mod card;
mod game;

use game::Game;

fn main() {
    let size = 4;
    let mut game = Game::new(size);

    while !game.is_game_over() {
        game.display_board();

        println!("Enter the row and column of the first card (e.g., 1 2):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let (row, col) = parse_input(&input);
        game.select_card(row, col);
    }

    println!("Congratulations! You've matched all the pairs.");
}

fn parse_input(input: &str) -> (usize, usize) {
    let mut iter = input.trim().split_whitespace();
    let row: usize = iter.next().unwrap().parse().expect("Invalid row input");
    let col: usize = iter.next().unwrap().parse().expect("Invalid column input");
    (row, col)
}
