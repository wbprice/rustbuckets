use std::io::{stdin, stdout, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::{
    controllers::Mode,
    models::{Board, Coordinates, Cursor, Game, Heading, Label, Ship, Attack},
    views::{BoardView, CursorView, LabelView, ShipView, AttackView},
};

pub fn game_controller(game: &mut Game) {
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
    let mut cursor = Cursor::default();

    // Views
    let title_view = LabelView::new(Coordinates { x: 1, y: 1 }, title);
    let red_board_title_view = LabelView::new(Coordinates { x: 1, y: 3 }, red_board_title);
    let red_board_view = BoardView::new(Coordinates { x: 1, y: 4 }, red_board);
    let blue_board_title_view = LabelView::new(Coordinates { x: 1, y: 22 }, blue_board_title);
    let blue_board_view = BoardView::new(Coordinates { x: 1, y: 23 }, blue_board);
    let mut blue_ship_views: Vec<ShipView> = vec![];
    for ship in game.blue_ships.iter() {
        blue_ship_views.push(ShipView::new(
            Coordinates {
                x: blue_board_view.origin.x,
                y: blue_board_view.origin.y,
            },
            ship.clone(),
        ))
    }
    let mut cursor_view = CursorView::new(red_board_view.origin, cursor);

    // Initial render
    title_view.render(&mut stdout);
    red_board_title_view.render(&mut stdout);
    red_board_view.render(&mut stdout);
    blue_board_title_view.render(&mut stdout);
    blue_board_view.render(&mut stdout);
    for ship_view in blue_ship_views.iter() {
        ship_view.render(&mut stdout);
    }
    cursor_view.render(&mut stdout);

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                game.switch_mode(Mode::Title);
                break;
            }
            Key::Char('w') => {
                cursor = cursor.move_up();
                cursor_view = cursor_view.update(cursor);
            }
            Key::Char('a') => {
                cursor = cursor.move_left();
                cursor_view = cursor_view.update(cursor);
            }
            Key::Char('s') => {
                cursor = cursor.move_down();
                cursor_view = cursor_view.update(cursor);
            }
            Key::Char('d') => {
                cursor = cursor.move_right();
                cursor_view = cursor_view.update(cursor);
            },
            Key::Char('f') => {
                match game.place_attack(cursor.origin) {
                    Ok(_) => {},
                    Err(_) => {
                        // handle err
                    }
                }
            }
            _ => {}
        }

        // Populate attack views list with attacks.
        for attack in game.blue_attacks.clone().into_iter() {
            let attack_view = AttackView::new(blue_board_view.origin, attack);
            attack_view.render(&mut stdout);
        }
        for attack in game.red_attacks.clone().into_iter() {
            let attack_view = AttackView::new(red_board_view.origin, attack);
            attack_view.render(&mut stdout);
        }

        // Rerender
        title_view.render(&mut stdout);
        red_board_title_view.render(&mut stdout);
        red_board_view.render(&mut stdout);
        blue_board_title_view.render(&mut stdout);
        blue_board_view.render(&mut stdout);
        for ship_view in blue_ship_views.iter() {
            ship_view.render(&mut stdout);
        }
        cursor_view.render(&mut stdout);

        stdout.flush().unwrap();
    }
}
