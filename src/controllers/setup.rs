use std::io::{stdin, stdout, Stdin, Stdout, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, style};

use crate::{
    controllers::Mode,
    models::{Board, Coordinates, Game, Label, Ship, Heading},
    views::{BoardView, LabelView, ShipView, NewShipView},
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

    // Setup AI ships
    // Toggle to red player
    game.toggle_active_player();
    for length in vec![2, 2, 3, 4, 5] {
        let ship = game
            .auto_create_ship(length)
            .expect("Should have been able to create the ship");
        game.place_ship(ship)
            .expect("Should have been able to place the ship!");
    }
    // Toggle to blue player
    game.toggle_active_player();

    // Views
    let title_view = LabelView::new(Coordinates { x: 1, y: 1 }, &title);
    let red_board_title_view = LabelView::new(Coordinates { x: 1, y: 3 }, &red_board_title);
    let red_board_view = BoardView::new(Coordinates { x: 1, y: 4 }, &red_board);
    let blue_board_title_view = LabelView::new(Coordinates { x: 1, y: 22 }, &blue_board_title);
    let blue_board_view = BoardView::new(Coordinates { x: 1, y: 23 }, &blue_board);
    let mut red_ship_views : Vec<ShipView> = vec![];
    let mut blue_ship_views : Vec<ShipView> = vec![];
    for ship in game.red_ships.iter() {
        red_ship_views.push(ShipView::new(
            Coordinates {
                x: red_board_view.origin.x,
                y: red_board_view.origin.y
            },
            &ship
        ))
    }

    // Initial render
    title_view.render(&mut stdout);
    red_board_title_view.render(&mut stdout);
    red_board_view.render(&mut stdout);
    blue_board_title_view.render(&mut stdout);
    blue_board_view.render(&mut stdout);
    for ship_view in red_ship_views.iter() {
        ship_view.render(&mut stdout);
    }


    // Preamble for letting players place their own ships
    let mut ship_lengths_to_place = vec![2, 2, 3, 4, 5];
    let mut new_ship_origin = Coordinates {
        x: 0,
        y: 0
    };
    let mut new_ship_length = ship_lengths_to_place.pop().unwrap();
    let mut new_ship_heading = Heading::East;
    let mut new_ship = Ship::new(
        new_ship_origin,
        new_ship_heading,
        new_ship_length
    );
    let mut new_ship_view = NewShipView::new(
        blue_board_view.origin,
        new_ship
    );

    new_ship_view.render(&mut stdout);

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('f') => {
                game.switch_mode(Mode::Play);
                break;
            }
            Key::Char('q') => {
                game.switch_mode(Mode::Title);
                break;
            },
            Key::Char('w') => {

            },
            Key::Char('a') => {

            },
            Key::Char('s') => {

            },
            Key::Char('d') => {

            }
            Key::Char('r') => {
                new_ship_heading = match new_ship_heading {
                    Heading::South => Heading::East,
                    Heading::East => Heading::South
                };

                new_ship_view = new_ship_view.update(
                    Ship::new(
                        new_ship_origin,
                        new_ship_heading,
                        new_ship_length
                    )
                )
            }
            _ => {}
        }

        // Rerender
        title_view.render(&mut stdout);
        red_board_title_view.render(&mut stdout);
        red_board_view.render(&mut stdout);
        blue_board_title_view.render(&mut stdout);
        blue_board_view.render(&mut stdout);
        for ship_view in red_ship_views.iter() {
            ship_view.render(&mut stdout);
        }
        new_ship_view.render(&mut stdout);

        stdout.flush().unwrap();
    }
}
