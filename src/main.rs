mod controllers;
mod views;
mod models;

use models::Game;

fn main() {
    let mut game = Game {
        ..Default::default()
    };

    game.start();
}
