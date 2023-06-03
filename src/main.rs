mod game;
use std::io::{self, Write};

fn main() {
    let mut game_status = game::Game::new();
    game_status.print();

    loop {

        println!("---------------------------------");
        let mut index = String::new();
        println!("\t{}'s turn", game_status.player());
        print!("\tEnter a number (1 to 9): ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut index).unwrap();
        let index: usize = index.trim().parse().unwrap();
        let index = index - 1;

        if game_status.is_valid_index(index) {
            game_status.update_matrix(index);
        } else {
            continue;
        }
        game_status.print();
        let status = game_status.status();
        match status {
            0 => {
                println!("\tCongratulations! {} won!\n", game_status.player());
                break;
            }
            1 => {
                println!("\tGame Drew!\n");
                break;
            }
            _ => ()
        }

        game_status.update_index();

    }

}
