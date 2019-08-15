use std::io::{stdin, stdout, Stdin, Stdout, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, style};

use crate::{
    controllers::Mode,
    models::{Board, Coordinates, Game, Heading, Label, Ship},
    views::{BoardView, LabelView, ShipView},
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
    let title_view = LabelView::new(Coordinates { x: 1, y: 1 }, title);
    let red_board_title_view = LabelView::new(Coordinates { x: 1, y: 3 }, red_board_title);
    let red_board_view = BoardView::new(Coordinates { x: 1, y: 4 }, red_board);
    let blue_board_title_view = LabelView::new(Coordinates { x: 1, y: 22 }, blue_board_title);
    let blue_board_view = BoardView::new(Coordinates { x: 1, y: 23 }, blue_board);
    let mut red_ship_views: Vec<ShipView> = vec![];
    let mut blue_ship_views: Vec<ShipView> = vec![];
    for ship in game.red_ships.clone().into_iter() {
        red_ship_views.push(ShipView::new(
            Coordinates {
                x: red_board_view.origin.x,
                y: red_board_view.origin.y,
            },
            ship,
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
    let mut new_ship_origin = Coordinates { x: 0, y: 0 };
    let mut new_ship_length = ship_lengths_to_place.pop().unwrap();
    let mut new_ship_heading = Heading::East;
    let mut new_ship = Ship::new(new_ship_origin, new_ship_heading, new_ship_length);
    let mut new_ship_view = ShipView::new(blue_board_view.origin, new_ship);

    new_ship_view.render(&mut stdout);

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('f') => {
                match game.place_ship(Ship::new(
                    new_ship_origin,
                    new_ship_heading,
                    new_ship_length,
                )) {
                    Ok(_) => {
                        match ship_lengths_to_place.pop() {
                            Some(length) => {

                                blue_ship_views.push(ShipView::new(
                                    blue_board_view.origin,
                                    Ship::new(
                                        new_ship_origin,
                                        new_ship_heading,
                                        new_ship_length
                                    )
                                ));

                                new_ship_length = length;
                                new_ship_heading = Heading::East;
                                new_ship_origin = Coordinates { x: 0, y: 0 };

                                new_ship = Ship::new(new_ship_origin, new_ship_heading, new_ship_length);
                                new_ship_view = ShipView::new(blue_board_view.origin, new_ship);
                            }
                            None => {
                                game.switch_mode(Mode::Play);
                                break;
                            }
                        }
                    },
                    Err(_) => {
                        // handle err
                    }
                }
            }
            Key::Char('q') => {
                game.switch_mode(Mode::Title);
                break;
            }
            Key::Char('w') => {
                if new_ship_origin.y > 0 {
                    new_ship_origin = new_ship_origin.move_up();
                    new_ship_view = new_ship_view.update(Ship::new(
                        new_ship_origin,
                        new_ship_heading,
                        new_ship_length,
                    ));
                }
            }
            Key::Char('a') => {
                if new_ship_origin.x > 0 {
                    new_ship_origin = new_ship_origin.move_left();
                    new_ship_view = new_ship_view.update(Ship::new(
                        new_ship_origin,
                        new_ship_heading,
                        new_ship_length,
                    ));
                }
            }
            Key::Char('s') => {
                let should_move = match new_ship_heading {
                    Heading::South => new_ship_length + new_ship_origin.y < game.height,
                    Heading::East => 1 + new_ship_origin.y < game.height,
                };
                if should_move {
                    new_ship_origin = new_ship_origin.move_down();
                    new_ship_view = new_ship_view.update(Ship::new(
                        new_ship_origin,
                        new_ship_heading,
                        new_ship_length,
                    ));
                }
            }
            Key::Char('d') => {
                let should_move = match new_ship_heading {
                    Heading::South => 1 + new_ship_origin.y < game.height,
                    Heading::East => new_ship_length + new_ship_origin.x < game.height,
                };
                if should_move {
                    new_ship_origin = new_ship_origin.move_right();
                    new_ship_view = new_ship_view.update(Ship::new(
                        new_ship_origin,
                        new_ship_heading,
                        new_ship_length,
                    ));
                }
            }
            Key::Char('r') => {
                new_ship_heading = match new_ship_heading {
                    Heading::South => Heading::East,
                    Heading::East => Heading::South,
                };

                new_ship_view = new_ship_view.update(Ship::new(
                    new_ship_origin,
                    new_ship_heading,
                    new_ship_length,
                ))
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
        for ship_view in blue_ship_views.iter() {
            ship_view.render(&mut stdout);
        }
        new_ship_view.render(&mut stdout);

        stdout.flush().unwrap();
    }
}
