mod controllers;
mod models;
mod views;

use models::Game;

fn main() {
    let mut game = Game {
        ..Default::default()
    };

    game.start();
}
