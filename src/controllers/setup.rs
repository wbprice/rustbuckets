use std::io::{stdin, stdout, Stdin, Stdout, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, style};

use crate::{
    controllers::Mode,
    models::{Board, Coordinates, Game, Label},
    views::{BoardView, LabelView},
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

    // Models
    let title = Label::new("Rustbuckets 0.1.0".to_string());
    let red_board_title = Label::new("Red Team".to_string());
    let blue_board_title = Label::new("Blue Team".to_string());
    let red_board = Board::new(game.width, game.height);
    let blue_board = Board::new(game.width, game.height);

    // Views
    let title_view = LabelView::new(Coordinates { x: 1, y: 1 }, &title);
    let red_board_title_view = LabelView::new(Coordinates { x: 1, y: 1 }, &red_board_title);
    let red_board_view = BoardView::new(Coordinates { x: 1, y: 2 }, &red_board);
    let blue_board_title_view = LabelView::new(Coordinates { x: 1, y: 1 }, &blue_board_title);
    let blue_board_view = BoardView::new(Coordinates { x: 1, y: 22 }, &blue_board);

    title_view.render(&mut stdout);
    red_board_title_view.render(&mut stdout);
    red_board_view.render(&mut stdout);

    blue_board_title_view.render(&mut stdout);
    blue_board_view.render(&mut stdout);

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
            }
            Key::Char('q') => {
                game.switch_mode(Mode::Title);
                break;
            }
            _ => {}
        }
    }
}
