
mod connect_four;
use connect_four::*;

fn main() {
    let mut game_board = Board::new(Player::Red);

    game_board.run_game();
    // game_board.update_cell(0, 3, Cell::Player(Player::Yellow));
    // game_board.print();
}

