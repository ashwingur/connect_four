use std::io;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    Red,
    Yellow,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell {
    Player(Player),
    Empty,
}

pub struct Board {
    pub current_player: Player,
    pub board: [[Cell; 7]; 6],
}

pub enum GameMoveResult {
    Valid,
    Won(Player),
    Stalemate,
}

impl Board {
    /// Creates a new board with the given starting player. The board is initialised to empty cells
    /// which is a 6x7 grid.
    pub fn new(starting_player: Player) -> Board {
        Board {
            current_player: starting_player,
            board: [[Cell::Empty; 7]; 6]
        }
    }

    pub fn run_game(&mut self) {
        // Get user input 
        loop {
            self.print();
            println!("Player {:?}, enter a move: ", self.current_player);
            let mut input = String::new();

            if let Err(e) = io::stdin().read_line(&mut input) {
                println!("Error getting input: {}", e);
            } else {
                match input.trim().parse::<usize>() {
                    Ok(n) => {
                        if n > 7 || n < 1 {
                            println!("Column {} is invalid", n);
                        } else {
                            match self.game_move(n - 1) {
                                Ok(game_move_result) => {
                                    match game_move_result {
                                        GameMoveResult::Valid => (),
                                        GameMoveResult::Won(p) => {
                                            println!("{:?} has a connect 4!\n", p);
                                            self.print();
                                            return;
                                        }
                                        GameMoveResult::Stalemate => {
                                            println!("Gameover, Stalemate");
                                            return;
                                        }
                                    }
                                }
                                Err(e) => println!("{}", e),
                            }
                        }
                    },
                    Err(_) => {
                        println!("Please enter a valid column number");
                    }
                }
            }
        }
    }

    pub fn game_move(&mut self, col: usize) -> Result<GameMoveResult, String> {
        // First check for stalemate
        let mut stalemate = true;
        for i in 0..7 {
            if let Some(_) = self.row_available(i) {
                stalemate = false;
                break;
            }
        }
        if stalemate {
            return Ok(GameMoveResult::Stalemate)
        }

        if let Some(row) = self.row_available(col) {
            self.update_cell(row, col, Cell::Player(self.current_player));
            if self.has_won(row, col) {
                return Ok(GameMoveResult::Won(self.current_player))
            }
            self.current_player = if self.current_player == Player::Yellow {
                Player::Red
            } else {
                Player::Yellow
            };
            Ok(GameMoveResult::Valid)
        } else {
            Err(format!("Column {} is full.", col))
        }
    }

    fn row_available(&self, col: usize) -> Option<usize> {
        for i in 0..6 {
            if self.board[i][col] == Cell::Empty {
                return Some(i);
            }
        }
        None
    }

    /// Prints the current state of the board
    pub fn print(&self) {
        for row in self.board.iter().rev() {
            for cell in row {
                match cell {
                    Cell::Player(player) => {
                        match player {
                            Player::Red => print!("😈  "),
                            Player::Yellow => print!("😳  ")
                        }
                    }
                    Cell::Empty => {
                        print!(" _  ")
                    }
                }
            }
            println!()
        }
        println!();
        for i in 1..8 {
            print!(" {}  ", i);
        }
        println!("\n");
    }

    pub fn update_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.board[row][col] = cell;
    }

    pub fn has_won(&self, row: usize, col: usize) -> bool {
        let mut connection_count = 0;
        // Checking row
        for i in 0..7 {
            if let Cell::Player(p) = self.board[row][i] {
                if p == self.current_player {
                    connection_count += 1;
                    if connection_count == 4 {
                        return true;
                    }
                } else {
                    connection_count = 0;
                }
            } else {
                connection_count = 0;
            }
        }
        // Checking column
        connection_count = 0;
        for i in 0..6 {
            if let Cell::Player(p) = self.board[i][col] {
                if p == self.current_player {
                    connection_count += 1;
                    if connection_count == 4 {
                        return true;
                    }
                } else {
                    connection_count = 0;
                }
            } else {
                connection_count = 0;
            }
        }
        // Checking north east diagonal
        // First go to the bottom left most point of the diagonal
        let mut coord = (row, col);
        if row <= col {
            coord.0 = 0;
            coord.1 -= row;
        } else {
            coord.0 -= col;
            coord.1 = 0;
        }
        connection_count = 0;
        let range = if 6 - coord.0 < 7 - coord.1 {
            6 - coord.0
        } else {
            7 - coord.1
        };
        for i in 0..range {
            if let Cell::Player(p) = self.board[coord.0 + i][coord.1 + i] {
                if p == self.current_player {
                    connection_count += 1;
                    if connection_count == 4 {
                        return true;
                    }
                } else {
                    connection_count = 0;
                }
            } else {
                connection_count = 0;
            }
        }

        // south east diagonal
        coord = (row, col);
        if (col + row) as i32 - 6 > 0 {
            coord.0 -= 6 - col;
            coord.1 = 6;
        } else {
            coord.0 = 0;
            coord.1 += row;
        }
        connection_count = 0;
        let range = if 5 - coord.0 < coord.1 {
            5 - coord.0
        } else {
            coord.1 + 1
        };
        for i in 0..range {
            if let Cell::Player(p) = self.board[coord.0 + i][coord.1 - i] {
                if p == self.current_player {
                    connection_count += 1;
                    if connection_count == 4 {
                        return true;
                    }
                } else {
                    connection_count = 0;
                }
                
            } else {
                connection_count = 0;
            }
        }
        false
    }
}

// Run "cargo test -- --nocapture" to display the println statements in the tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_board() {
        let starting_player = Player::Red;

        let board = Board::new(starting_player);

        assert_eq!(board.current_player, Player::Red);

        for row in board.board {
            assert!(row.iter().eq([Cell::Empty; 7].iter()));   
        }

    }

    #[test]
    fn horizontal_connect_four() {
        let mut board = Board::new(Player::Red);

        board.update_cell(0, 1, Cell::Player(Player::Red));
        board.update_cell(0, 2, Cell::Player(Player::Yellow));
        board.update_cell(0, 3, Cell::Player(Player::Red));
        board.update_cell(0, 4, Cell::Player(Player::Red));
        board.update_cell(0, 5, Cell::Player(Player::Red));
        board.update_cell(0, 6, Cell::Player(Player::Red));

        board.update_cell(1, 1, Cell::Player(Player::Red));
        board.update_cell(1, 2, Cell::Player(Player::Red));
        board.update_cell(1, 3, Cell::Player(Player::Red));

        board.print();
        assert!(board.has_won(0, 3));
        assert!(!board.has_won(1, 1));
    }

    #[test]
    fn vertical_connect_four() {
        let mut board = Board::new(Player::Yellow);

        board.update_cell(0, 3, Cell::Player(Player::Yellow));
        board.update_cell(1, 3, Cell::Player(Player::Red));
        board.update_cell(2, 3, Cell::Player(Player::Yellow));
        board.update_cell(3, 3, Cell::Player(Player::Yellow));
        board.update_cell(4, 3, Cell::Player(Player::Yellow));
        board.update_cell(5, 3, Cell::Player(Player::Yellow));

        board.print();

        assert!(board.has_won(2, 3));
    }

    #[test]
    fn diagonal_connect_four() {
        let mut board = Board::new(Player::Yellow);

        // Left diagonal
        board.update_cell(0, 3, Cell::Player(Player::Yellow));
        board.update_cell(1, 3, Cell::Player(Player::Red));
        board.update_cell(2, 4, Cell::Player(Player::Red));
        board.update_cell(3, 5, Cell::Player(Player::Red));
        board.update_cell(4, 6, Cell::Player(Player::Red));

        // Right diagonal
        board.update_cell(3, 3, Cell::Player(Player::Red));
        board.update_cell(4, 2, Cell::Player(Player::Red));
        board.update_cell(5, 1, Cell::Player(Player::Red));

        // 2 long diagonal
        board.update_cell(5, 5, Cell::Player(Player::Red));

        board.print();

        assert!(board.has_won(2, 4));
        assert!(board.has_won(3, 3));
        assert!(!board.has_won(5, 5));
    }


}