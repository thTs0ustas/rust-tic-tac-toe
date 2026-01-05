use super::player::Player;
use super::turn::Turn;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    Taken(Turn),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Taken(turn) => match turn {
                Turn::X => write!(f, "X"),
                Turn::O => write!(f, "O"),
            },
        }
    }
}
pub struct Game {
    board: [Cell; 9],
    player1: Player,
    player2: Player,
    current_turn: Turn,
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        Self {
            board: [Cell::Empty; 9],
            current_turn: Turn::X,
            player1,
            player2,
        }
    }

    fn is_draw(&self) -> bool {
        self.board.iter().all(|c| matches!(c, Cell::Taken(_)))
    }

    fn winner(&self) -> Option<&Player> {
        let winning_combinations = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for combo in &winning_combinations {
            if let Cell::Taken(c) = self.board[combo[0]]
                && self.board[combo[1]] == Cell::Taken(c)
                && self.board[combo[2]] == Cell::Taken(c)
            {
                return match self.current_turn {
                    Turn::X => Some(&self.player1),
                    Turn::O => Some(&self.player2),
                };
            }
        }

        None
    }

    fn render(&self) {
        for (i, chunk) in self.board.chunks(3).enumerate() {
            let row_cells: Vec<String> = chunk
                .iter()
                .enumerate()
                .map(|(j, cell)| match cell {
                    Cell::Empty => ((i * 3) + j + 1).to_string(),
                    _ => cell.to_string(),
                })
                .collect();

            println!(" {} | {} | {} ", row_cells[0], row_cells[1], row_cells[2]);
            if i < 2 {
                println!("-----------");
            }
        }
    }

    fn update_board(&mut self, idx: usize) {
        self.board[idx] = Cell::Taken(self.current_turn);
    }

    fn take_turn(&mut self) {
        let mut player_move = String::new();

        loop {
            player_move.clear();
            let prompt = std::io::stdin().read_line(&mut player_move);

            match prompt {
                Ok(_) => {
                    let idx = match player_move.trim().parse::<usize>() {
                        Ok(n @ 1..=9) => n - 1,
                        _ => {
                            println!("Please enter a number between 1 and 9.");
                            continue;
                        }
                    };

                    if let Cell::Empty = self.board[idx] {
                        self.update_board(idx);
                        break;
                    } else {
                        println!("Pick a different number");
                        continue;
                    }
                }
                Err(e) => {
                    println!("Error reading input: {}. Please try again.", e);
                }
            }
        }
    }

    pub fn play(&mut self) {
        loop {
            println!("\nCurrent board:");
            self.render();
            self.take_turn();

            if let Some(winner) = self.winner() {
                println!("{} wins!", winner.name);
                break;
            }

            if self.is_draw() {
                println!("It's a draw!");
                break;
            }

            self.current_turn = self.current_turn.other();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let p1 = Player { name: "A".into() };
        let p2 = Player { name: "B".into() };
        let game = Game::new(p1, p2);
        assert!(game.winner().is_none());
        assert!(!game.is_draw());
    }

    #[test]
    fn test_x_wins() {
        let p1 = Player { name: "A".into() };
        let p2 = Player { name: "B".into() };
        let mut game = Game::new(p1, p2);

        // X moves 0
        game.update_board(0);
        // Force turn switch manually for setup?
        // update_board uses current_turn.
        game.current_turn = Turn::O;

        // O moves 3
        game.update_board(3);
        game.current_turn = Turn::X;

        // X moves 1
        game.update_board(1);
        game.current_turn = Turn::O;

        // O moves 4
        game.update_board(4);
        game.current_turn = Turn::X;

        // X moves 2
        game.update_board(2);

        // Check winner. Winner logic checks current_turn (X) and board.
        // If X connects 0,1,2, winner() returns Some(p1) because current_turn is X.
        let w = game.winner();
        assert!(w.is_some());
        assert_eq!(w.unwrap().name, "A");
    }
}
