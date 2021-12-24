
mod connect_four;
use connect_four::*;

fn main() {
    let mut game_board = Board::new(Player::Red);
    game_board.run_game();
}

