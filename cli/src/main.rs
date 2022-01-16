mod controllers;
mod models;
mod views;

use controllers::{
    endscreen_controller, game_controller, setup_controller, title_controller, Mode,
};
use models::Game;

fn main() {
    let mut game = Game {
        ..Default::default()
    };

    loop {
        match game.mode {
            Mode::Title => title_controller(&mut game),
            Mode::Setup => {
                game = Game {
                    ..Default::default()
                };
                setup_controller(&mut game)
            }
            Mode::Play => game_controller(&mut game),
            Mode::Endscreen => endscreen_controller(&mut game),
            Mode::Exit => {
                break;
            }
        }
    }
}
