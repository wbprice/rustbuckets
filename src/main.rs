mod controllers;
mod models;

use models::Game;

fn main() {
    let game = Game {
        ..Default::default()
    };

    dbg!(game);
    println!("Hello world!");
}
