use std::io::{stdin, stdout, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode};

use crate::{
    models::{
        Game,
        Label,
        Coordinates
    },
    views::{
        LabelView
    },
    controllers::{
        Mode
    }
};

pub fn title_controller(game: &mut Game) {
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
    let play_instructions = Label::new("Press F to start".to_string());
    let quit_instructions = Label::new("Press Q to quit".to_string());

    // Views
    let title_view = LabelView::new(Coordinates { x: 1, y: 1}, &title);
    let play_instructions_view = LabelView::new(Coordinates { x: 1, y: 2}, &play_instructions);
    let quit_instructions_view = LabelView::new(Coordinates { x: 1, y: 3}, &quit_instructions);

    title_view.render(&mut stdout);
    play_instructions_view.render(&mut stdout);
    quit_instructions_view.render(&mut stdout);

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('f') => {
                game.switch_mode(Mode::Setup);
                break;
            },
            Key::Char('q') => {
                game.switch_mode(Mode::Exit);
                break;
            }
            _ => {}
        }
    }
}