use std::io::{stdin, stdout, Stdin, Stdout, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, style};

use crate::models::{
    Game,
    Board,
    Label,
    Coordinates
};
use crate::controllers::{
    Mode
};

pub fn setup_controller(game: &mut Game) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();

    let title = Label::new("Rustbuckets 0.1.0".to_string());
    let red_board_title = Label::new("Red Team".to_string());
    let blue_board_title = Label::new("Blue Team".to_string());
    let red_board = Board::new(game.width, game.height);
    let blue_board = Board::new(game.width, game.height);

    title.render(&mut stdout, Coordinates { x: 1, y: 1});
    red_board_title.render(&mut stdout, Coordinates { x: 1, y: 3});
    red_board.render(&mut stdout, Coordinates { x: 1, y: 4});

    blue_board_title.render(&mut stdout, Coordinates { x: 1, y: 22});
    blue_board.render(&mut stdout, Coordinates { x: 1, y : 23});

    stdout.flush().unwrap();

    for length in vec![2, 2, 3, 4, 5] {
        let ship = game
            .auto_create_ship(length)
            .expect("Should have been able to create the ship");
        game.place_ship(ship)
            .expect("Should have been able to place the ship!");
    }

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('f') => {
                game.switch_mode(Mode::Play);
                break;
            },
            Key::Char('q') => {
                game.switch_mode(Mode::Title);
                break;
            },
            _ => {}
        }
    }
}