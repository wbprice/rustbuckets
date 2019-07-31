use std::io::{stdin, stdout, Stdout, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, style};

#[derive(Debug)]
enum Faction {
    Red,
    Blue,
}


#[derive(Debug, Copy, Clone)]
struct Coordinates {
    x: u16,
    y: u16,
}

fn translate_game_coords_to_board_coords(coordinates: Coordinates) -> Coordinates {
    Coordinates {
        x: coordinates.x * 4 + 1,
        y: coordinates.y * 2 + 1,
    }
}

#[derive(Clone, Copy, Debug)]
struct Attack<'a> {
    coordinates: Coordinates,
    board: &'a Board,
    result: AttackResults,
}

impl<'a> Attack<'a> {
    fn new(coordinates: Coordinates, board: &Board) -> Attack {
        Attack {
            coordinates,
            board,
            result: AttackResults::Miss,
        }
    }

    fn render(self, stdout: &mut RawTerminal<Stdout>) {
        let symbol = match self.result {
            AttackResults::Hit => "X",
            AttackResults::Miss => "O",
        };

        let board_coords = translate_game_coords_to_board_coords(Coordinates {
            x: self.coordinates.x,
            y: self.coordinates.y
        });
        let screen_coords = Coordinates {
            x: board_coords.x + self.board.origin.x + 1,
            y: board_coords.y + self.board.origin.y
        };

        write!(
            stdout,
            "{}{}{}{}{}",
            Goto(screen_coords.x, screen_coords.y),
            color::Fg(color::White),
            color::Bg(color::Black),
            symbol,
            style::Reset
        )
        .unwrap();
    }
}

#[derive(Copy, Clone, Debug)]
enum AttackResults {
    Hit,
    Miss,
}

#[derive(Debug)]
enum Heading {
    North,
    East,
    West,
    South,
}

#[derive(Debug)]
enum Condition {
    Nominal,
    Damaged,
    Destroyed
}

#[derive(Debug, Copy, Clone)]

struct Cursor<'a>{
    coordinates: Coordinates,
    board: &'a Board
}

impl<'a> Cursor<'a> {
    fn new(coordinates: Coordinates, board: &'a Board) -> Cursor<'a> {
        Cursor {
            coordinates,
            board
        }
    }

    fn on_move(self, heading: Heading) -> Cursor<'a> {
        match heading {
            Heading::North => {
                if self.coordinates.y > 0 {
                    Cursor {
                        coordinates: Coordinates {
                            x: self.coordinates.x,
                            y: self.coordinates.y - 1,
                        },
                        board: self.board
                    }
                } else {
                    self
                }
            }
            Heading::East => {
                if self.coordinates.x < 7 {
                    Cursor {
                        coordinates: Coordinates {
                            x: self.coordinates.x + 1,
                            y: self.coordinates.y,
                        },
                        board: self.board
                    }
                } else {
                    self
                }
            }
            Heading::West => {
                if self.coordinates.x > 0 {
                    Cursor {
                        coordinates: Coordinates {
                            x: self.coordinates.x - 1,
                            y: self.coordinates.y,
                        },
                        board: self.board
                    }
                } else {
                    self
                }
            }
            Heading::South => {
                if self.coordinates.y < 7 {
                    Cursor {
                        coordinates: Coordinates {
                            x: self.coordinates.x,
                            y: self.coordinates.y + 1,
                        },
                        board: self.board,
                    }
                } else {
                    self
                }
            }
        }
    }

    fn render(self, stdout: &mut RawTerminal<Stdout>) {
        let board_coords = translate_game_coords_to_board_coords(self.coordinates);
        let screen_coords = Coordinates {
            x: board_coords.x + self.board.origin.x,
            y: board_coords.y + self.board.origin.y,
        };
        write!(
            stdout,
            "{}{}[ ]{}",
            Goto(screen_coords.x, screen_coords.y),
            color::Bg(color::Blue),
            style::Reset
        )
        .unwrap()
    }
}

#[derive(Debug)]
struct Board {
    faction: Faction,
    origin: Coordinates,
    height: u16,
    width: u16,
}

impl Board {
    fn new(faction: Faction, origin: Coordinates, width: u16, height: u16) -> Board {
        Board {
            faction,
            origin,
            height,
            width
        }
    }

    fn _render_latitude_line(&self, stdout: &mut RawTerminal<Stdout>) {
        let mut output = "+".to_string();
        for _ in 0..8 {
            output.push_str("---+");
        }
        write!(
            stdout,
            "{}{}{}{}\n\r",
            color::Fg(color::White),
            color::Bg(color::Blue),
            output,
            style::Reset
        )
        .unwrap();
    }

    fn _render_longitude_line(&self, stdout: &mut RawTerminal<Stdout>) {
        let mut output = "|".to_string();
        for _ in 0..8 {
            output.push_str("   |");
        }
        write!(
            stdout,
            "{}{}{}{}\n\r",
            color::Fg(color::White),
            color::Bg(color::Blue),
            output,
            style::Reset
        )
        .unwrap();
    }

    fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, "{}", Goto(self.origin.x, self.origin.y)).unwrap();
        for _ in 1..self.height + 1 {
            self._render_latitude_line(stdout);
            self._render_longitude_line(stdout);
        }
        self._render_latitude_line(stdout);
        write!(stdout, "\n\r").unwrap();
    }
}

struct Label {
    origin: Coordinates,
    content: String,
}

impl Label {
    fn new(x: u16, y: u16, content: String) -> Label {
        Label {
            origin: Coordinates { x, y },
            content,
        }
    }

    fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(
            stdout,
            "{}{}{}{}",
            Goto(self.origin.x, self.origin.y),
            color::Fg(color::White),
            self.content,
            style::Reset
        )
        .unwrap();
    }
}

#[derive(Debug)]
struct Ship<'a> {
    origin: Coordinates,
    board: &'a Board,
    heading: Heading,
    condition: Condition,
    segments: Vec<ShipSegment>
}

impl<'a> Ship<'a> {
    fn new(origin: Coordinates, board: &'a Board, heading: Heading, length: u16) -> Ship {
        let mut segments: Vec<ShipSegment> = vec![];
        // For n segments in
        for n in 0..length {
            match heading {
                Heading::North => {
                    segments.push(
                        ShipSegment::new(
                            Coordinates {
                                x: origin.x,
                                y: origin.y - n
                            }
                        )
                    )
                },
                Heading::East => {
                    segments.push(
                        ShipSegment::new(
                            Coordinates {
                                x: origin.x + n,
                                y: origin.y
                            }
                        )
                    )
                },
                Heading::West => {
                    segments.push(
                        ShipSegment::new(
                            Coordinates {
                                x: origin.x - n,
                                y: origin.y
                            }
                        )
                    )
                },
                Heading::South => {
                    segments.push(
                        ShipSegment::new(
                            Coordinates {
                                x: origin.x,
                                y: origin.y + n
                            }
                        )
                    )
                }
            }
        }

        Ship {
            origin,
            board,
            heading,
            condition: Condition::Nominal,
            segments
        }
    }

    fn render(self, stdout: &mut RawTerminal<Stdout>) {
        for segment in self.segments {

        }
    }
}

#[derive(Debug)]
struct ShipSegment {
    coordinates: Coordinates,
    condition: Condition
}

impl ShipSegment {
    fn new(coordinates: Coordinates) -> ShipSegment {
        ShipSegment {
            coordinates,
            condition: Condition::Nominal
        }
    }

    fn render(self, base: Coordinates) {

    }
}


fn main() {
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

    let red_board = Board::new(Faction::Blue, Coordinates { x: 1, y: 2 }, 8, 8);
    let blue_board = Board::new(Faction::Red, Coordinates { x: 1, y: 20 }, 8, 8);
    let mut cursor = Cursor::new(Coordinates { x: 0, y: 0 }, &red_board);
    let mut attacks: Vec<Attack> = Vec::new();
    let mut info = Label::new(1, 19, "Hello".to_string());
    let title = Label::new(1, 1, "Rustbuckets v0.1.0".to_string());

    red_board.render(&mut stdout);
    blue_board.render(&mut stdout);
    cursor.render(&mut stdout);
    info.render(&mut stdout);
    title.render(&mut stdout);
    for attack in attacks.clone() {
        attack.render(&mut stdout);
    }

    stdout.flush().unwrap();

    // Handle user inputs and render interface
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                write!(stdout, "{}", style::Reset).unwrap();
                break;
            }
            Key::Char('w') => {
                cursor = cursor.on_move(Heading::North);
            }
            Key::Char('a') => {
                cursor = cursor.on_move(Heading::West);
            }
            Key::Char('s') => {
                cursor = cursor.on_move(Heading::South);
            }
            Key::Char('d') => {
                cursor = cursor.on_move(Heading::East);
            }
            Key::Char('f') => {
                attacks.push(Attack::new(cursor.coordinates, &red_board));
            }
            _ => {}
        }

        red_board.render(&mut stdout);
        blue_board.render(&mut stdout);
        cursor.render(&mut stdout);
        info = Label::new(
            1,
            19,
            format!(
                "({},{})",
                cursor.coordinates.x,
                cursor.coordinates.y
            ),
        );
        for attack in attacks.clone() {
            attack.render(&mut stdout);
        }
        info.render(&mut stdout);
        stdout.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_coords_0_0() {
        let coords = Coordinates { x: 0, y: 0 };
        let result = translate_game_coords_to_board_coords(coords);
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn test_translate_coords_0_1() {
        let coords = Coordinates { x: 0, y: 1 };
        let result = translate_game_coords_to_board_coords(coords);
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 3);
    }

    #[test]
    fn test_translate_coords_1_0() {
        let coords = Coordinates { x: 1, y: 0 };
        let result = translate_game_coords_to_board_coords(coords);
        assert_eq!(result.x, 5);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn test_translate_coords_1_1() {
        let coords = Coordinates { x: 1, y: 1 };
        let result = translate_game_coords_to_board_coords(coords);
        assert_eq!(result.x, 5);
        assert_eq!(result.y, 3);
    }

    #[test]
    fn test_create_ship_east_0_0() {
        let origin = Coordinates { x: 0, y: 0 };
        let board = Board::new(Faction::Blue, Coordinates { x: 1, y: 2}, 8, 8);
        let ship = Ship::new(origin, &board, Heading::East, 3);

        assert_eq!(ship.segments.len(), 3);
        assert_eq!(ship.segments[0].coordinates.x, 0);
        assert_eq!(ship.segments[0].coordinates.y, 0);
        assert_eq!(ship.segments[1].coordinates.x, 1);
        assert_eq!(ship.segments[1].coordinates.y, 0);
        assert_eq!(ship.segments[2].coordinates.x, 2);
        assert_eq!(ship.segments[2].coordinates.y, 0);
    }

    #[test]
    fn test_create_ship_south_0_0() {
        let origin = Coordinates { x: 0, y: 0 };
        let board = Board::new(Faction::Blue, Coordinates { x: 1, y: 2}, 8, 8);
        let ship = Ship::new(origin, &board, Heading::South, 3);

        assert_eq!(ship.segments.len(), 3);
        assert_eq!(ship.segments[0].coordinates.x, 0);
        assert_eq!(ship.segments[0].coordinates.y, 0);
        assert_eq!(ship.segments[1].coordinates.x, 0);
        assert_eq!(ship.segments[1].coordinates.y, 1);
        assert_eq!(ship.segments[2].coordinates.x, 0);
        assert_eq!(ship.segments[2].coordinates.y, 2);
    }
}
