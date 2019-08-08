mod entities;
mod systems;

use crate::{systems::Game};

fn main() {

    let game = Game {
        ..Default::default()
    };
    dbg!(game);
    
    println!("Hello world!");
}