mod controllers;
mod models;

use models::Game;

fn main() {
    let mut game = Game {
        ..Default::default()
    };

    game.start();
}
