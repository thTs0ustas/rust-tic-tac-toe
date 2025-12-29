mod game;
mod player;
mod turn;
use crate::game::Game;
use crate::player::Player;

fn main() {
    println!("Hello, world!");
    println!("This is a Tic Tac Toe game.");
    println!("Set players' names and start playing!\n\n");

    println!("------------------");
    println!("Enter name for Player 1 (X): ");
    let player1 = Player {
        name: read_player(),
    };

    println!("------------------");
    println!("Enter name for Player 2 (O): ");
    let player2 = Player {
        name: read_player(),
    };

    println!("------------------");
    println!("Player 1: {:?}", player1);
    println!("Player 2: {:?}", player2);
    println!("------------------\n\n");

    let mut game = Game::new(player1, player2);
    game.play();
}

fn read_player() -> String {
    let mut buf = String::new();

    loop {
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) if !buf.trim().is_empty() => {
                return buf.trim().to_owned();
            }
            Ok(_) => {
                println!("Name cannot be empty. Please enter a valid name: ");
                continue;
            }
            Err(e) => {
                println!("Error reading input: {}. Please try again.", e);
                continue;
            }
        };
    }
}
