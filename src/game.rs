pub mod gm {
    use crate::player::player::Player;
    use crate::turn::turn::Turn;

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Cell {
        Empty,
        Taken(Turn),
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
                board: [
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                ],
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
            let elements: Vec<String> = self
                .board
                .iter()
                .enumerate()
                .map(|(idx, cell)| match cell {
                    Cell::Empty => (idx + 1).to_string(),
                    Cell::Taken(c) => match c {
                        Turn::X => "X".to_string(),
                        Turn::O => "O".to_string(),
                    },
                })
                .collect();

            println!("{} | {} | {}", elements[0], elements[1], elements[2]);
            println!("---------");
            println!("{} | {} | {}", elements[3], elements[4], elements[5]);
            println!("---------");
            println!("{} | {} | {}", elements[6], elements[7], elements[8]);
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
}
