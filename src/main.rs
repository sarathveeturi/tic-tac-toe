use std::io;
use std::cmp::Ordering;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
    Human,
    Computer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    X,
    O,
}

#[derive(Debug)]
struct Board {
    cells: [Cell; 9],
}

impl Board {
    fn new() -> Board {
        Board { cells: [Cell::Empty; 9] }
    }

    fn display(&self) {
        println!(" 0 | 1 | 2 ");
        println!("-----------");
        println!(" 3 | 4 | 5 ");
        println!("-----------");
        println!(" 6 | 7 | 8 ");
        println!();

        for (i, cell) in self.cells.iter().enumerate() {
            let symbol = match *cell {
                Cell::Empty => ' ',
                Cell::X => 'X',
                Cell::O => 'O',
            };
            print!(" {} ", symbol);

            if i % 3 == 2 {
                println!();
            } else {
                print!("|");
            }
        }

        println!();
    }

    fn make_move(&mut self, index: usize, player: Player) -> Result<(), &'static str> {
        if index < 0 || index >= 9 || self.cells[index] != Cell::Empty {
            return Err("Invalid move. Please try again.");
        }

        self.cells[index] = match player {
            Player::Human => Cell::X,
            Player::Computer => Cell::O,
        };

        Ok(())
    }

    fn check_winner(&self, player: Player) -> bool {
        let symbol = match player {
            Player::Human => Cell::X,
            Player::Computer => Cell::O,
        };

        // Check rows, columns, and diagonals
        for i in 0..3 {
            if self.cells[i] == symbol && self.cells[i + 3] == symbol && self.cells[i + 6] == symbol {
                return true; // Check columns
            }
            if self.cells[i * 3] == symbol && self.cells[i * 3 + 1] == symbol && self.cells[i * 3 + 2] == symbol {
                return true; // Check rows
            }
        }

        if self.cells[0] == symbol && self.cells[4] == symbol && self.cells[8] == symbol {
            return true; // Check diagonal from top-left to bottom-right
        }

        if self.cells[2] == symbol && self.cells[4] == symbol && self.cells[6] == symbol {
            return true; // Check diagonal from top-right to bottom-left
        }

        false
    }

    fn is_full(&self) -> bool {
        !self.cells.iter().any(|&cell| cell == Cell::Empty)
    }
}

fn main() {
    let mut board = Board::new();
    let mut player_turn = Player::Human;

    loop {
        board.display();

        match player_turn {
            Player::Human => {
                println!("Enter your move (0-8):");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let index: usize = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number between 0 and 8.");
                        continue;
                    }
                };

                match board.make_move(index, Player::Human) {
                    Ok(_) => {
                        if board.check_winner(Player::Human) {
                            board.display();
                            println!("You win!");
                            break;
                        } else if board.is_full() {
                            board.display();
                            println!("It's a draw!");
                            break;
                        }

                        player_turn = Player::Computer;
                    }
                    Err(err) => println!("{}", err),
                }
            }
            Player::Computer => {
                let mut available_moves: Vec<usize> = Vec::new();
                for (i, &cell) in board.cells.iter().enumerate() {
                    if cell == Cell::Empty {
                        available_moves.push(i);
                    }
                }

                if let Some(&computer_move) = available_moves.choose(&mut rand::thread_rng()) {
                    match board.make_move(computer_move, Player::Computer) {
                        Ok(_) => {
                            if board.check_winner(Player::Computer) {
                                board.display();
                                println!("Computer wins!");
                                break;
                            } else if board.is_full() {
                                board.display();
                                println!("It's a draw!");
                                break;
                            }

                            player_turn = Player::Human;
                        }
                        Err(_) => continue,
                    }
                }
            }
        }
    }
}
