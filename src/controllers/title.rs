use std::io::{stdin, stdout, Stdin, Stdout, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, style};

use crate::models::{
    Game,
    Label,
    Coordinates
};
use crate::controllers::{
    Mode
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

    let title = Label::new("Rustbuckets 0.1.0".to_string());
    let play_instructions = Label::new("Press F to start".to_string());
    let quit_instructions = Label::new("Press Q to quit".to_string());

    title.render(&mut stdout, Coordinates { x: 1, y: 1});
    play_instructions.render(&mut stdout, Coordinates { x: 1, y: 2});
    quit_instructions.render(&mut stdout, Coordinates { x: 1, y: 3});

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