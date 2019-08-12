mod models;
mod controllers;

use models::Game;

fn main() {

    let game = Game {
        ..Default::default()
    };

    dbg!(game);
    println!("Hello world!");
}