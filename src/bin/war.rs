use std::io;

use naipe::games::war::WarGame;
use naipe::games::Game;

fn main() -> Result<(), ()> {
    env_logger::init();

    let mut input = String::new();
    let stdin = io::stdin();

    let mut game_state = WarGame::default();
    while !game_state.tick()? {
        stdin.read_line(&mut input).unwrap();
    }
    if game_state.player_1_won() {
        println!("Player 1 Won!");
    } else {
        println!("Player 2 Won!");
    }
    Ok(())
}
