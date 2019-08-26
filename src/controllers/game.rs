use std::io::{stdin, stdout, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::{
    controllers::Mode,
    models::{
        Alert, Attack, AttackResult, Board, Coordinates, Cursor, Game, Heading, Label, Level,
        Scores, Ship,
    },
    views::{AlertView, AttackView, BoardView, CursorView, LabelView, ScoresView, ShipView},
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
    let blue_instructions = Alert::new("It's time to fight!".to_string(), Level::Info);

    // Views
    let title_view = LabelView::new(Coordinates { x: 1, y: 1 }, title);
    let red_board_title_view = LabelView::new(Coordinates { x: 1, y: 3 }, red_board_title);
    let red_board_view = BoardView::new(Coordinates { x: 1, y: 4 }, red_board);
    let mut blue_instructions_view = AlertView::new(Coordinates { x: 1, y: 23 }, blue_instructions);
    let blue_board_title_view = LabelView::new(Coordinates { x: 1, y: 27 }, blue_board_title);
    let blue_board_view = BoardView::new(Coordinates { x: 1, y: 28 }, blue_board);
    let mut blue_ship_views: Vec<ShipView> = vec![];
    let mut red_team_score_view = ScoresView::new(Coordinates { x: 36, y: 0 }, game.blue_score);
    let mut blue_team_score_view = ScoresView::new(Coordinates { x: 36, y: 24 }, game.red_score);
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
    red_team_score_view.render(&mut stdout);
    blue_team_score_view.render(&mut stdout);
    blue_instructions_view.render(&mut stdout);
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
                if cursor.origin.y > 0 {
                    cursor = cursor.move_up();
                    cursor_view = cursor_view.update(cursor);
                }
            }
            Key::Char('a') => {
                if cursor.origin.x > 0 {
                    cursor = cursor.move_left();
                    cursor_view = cursor_view.update(cursor);
                }
            }
            Key::Char('s') => {
                if cursor.origin.y < game.height - 1 {
                    cursor = cursor.move_down();
                    cursor_view = cursor_view.update(cursor);
                }
            }
            Key::Char('d') => {
                if cursor.origin.x < game.width - 1 {
                    cursor = cursor.move_right();
                    cursor_view = cursor_view.update(cursor);
                }
            }
            Key::Char('f') => {
                match game.place_attack(cursor.origin) {
                    Ok(attack) => {
                        match attack.result {
                            AttackResult::Hit => {
                                blue_instructions_view = blue_instructions_view.update(Alert::new(
                                    "That was a hit!".to_string(),
                                    Level::Success,
                                ));
                            }
                            AttackResult::Miss => {
                                blue_instructions_view = blue_instructions_view
                                    .update(Alert::new("You missed!".to_string(), Level::Warning));
                            }
                        }
                        blue_instructions_view.render(&mut stdout);
                        stdout.flush().unwrap();

                        // Attack placed.  Now it's time for the
                        // AI to retaliate.

                        game.toggle_active_player();

                        loop {
                            let ai_attack_coords = game.auto_plan_attack().unwrap();

                            match game.place_attack(ai_attack_coords) {
                                Ok(attack) => {
                                    match attack.result {
                                        AttackResult::Hit => {
                                            blue_instructions_view =
                                                blue_instructions_view.update(Alert::new(
                                                    "They hit a ship!".to_string(),
                                                    Level::Warning,
                                                ));
                                        }
                                        AttackResult::Miss => {
                                            blue_instructions_view = blue_instructions_view.update(
                                                Alert::new("They missed!".to_string(), Level::Info),
                                            );
                                        }
                                    }

                                    blue_instructions_view.render(&mut stdout);
                                    stdout.flush().unwrap();

                                    break;
                                }
                                Err(_) => {
                                    // handle err?
                                    blue_instructions_view = blue_instructions_view.update(
                                        Alert::new("The AI is confused!".to_string(), Level::Error),
                                    );
                                    blue_instructions_view.render(&mut stdout);
                                    stdout.flush().unwrap();
                                }
                            }
                        }

                        blue_instructions_view = blue_instructions_view.update(Alert::new(
                            "Select a cell to attack!".to_string(),
                            Level::Info,
                        ));
                        blue_instructions_view.render(&mut stdout);
                        stdout.flush().unwrap();

                        game.toggle_active_player();
                    }
                    Err(_) => {
                        // handle err
                        blue_instructions_view = blue_instructions_view.update(Alert::new(
                            "An attack can't be made there!".to_string(),
                            Level::Warning,
                        ));
                        blue_instructions_view.render(&mut stdout);
                        stdout.flush().unwrap();
                    }
                }
            }
            _ => {}
        }

        // Was there a win?
        if game.blue_score.hits >= 16 || game.red_score.hits >= 16 {
            game.switch_mode(Mode::Endscreen);
            break;
        }

        // Update score views
        red_team_score_view = red_team_score_view.update(game.blue_score);
        blue_team_score_view = blue_team_score_view.update(game.red_score);

        // Rerender
        title_view.render(&mut stdout);
        red_board_title_view.render(&mut stdout);
        red_board_view.render(&mut stdout);
        blue_board_title_view.render(&mut stdout);
        blue_board_view.render(&mut stdout);
        red_team_score_view.render(&mut stdout);
        blue_team_score_view.render(&mut stdout);
        blue_instructions_view.render(&mut stdout);
        for ship_view in blue_ship_views.iter() {
            ship_view.render(&mut stdout);
        }
        // Populate attack views list with attacks and render.
        for attack in game.blue_attacks.clone().into_iter() {
            let attack_view = AttackView::new(blue_board_view.origin, attack);
            attack_view.render(&mut stdout);
        }
        for attack in game.red_attacks.clone().into_iter() {
            let attack_view = AttackView::new(red_board_view.origin, attack);
            attack_view.render(&mut stdout);
        }
        cursor_view.render(&mut stdout);

        stdout.flush().unwrap();
    }
}
