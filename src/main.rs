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

#[derive(Debug)]
struct Board {
    faction: Faction,
    origin: Coordinates,
    height: u16,
    width: u16,
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

#[derive(Debug, Copy, Clone)]
struct Cursor {
    coordinates: Coordinates,
    base: Coordinates,
}

#[derive(Clone, Copy, Debug)]
struct Attack {
    coordinates: Coordinates,
    base: Coordinates,
    result: AttackResults,
}

impl Attack {
    fn new(x: u16, y: u16, base_x: u16, base_y: u16) -> Attack {
        Attack {
            coordinates: Coordinates { x, y },
            base: Coordinates { x: base_x, y: base_y },
            result: AttackResults::Miss,
        }
    }

    fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        let symbol = match self.result {
            AttackResults::Hit => "X",
            AttackResults::Miss => "O",
        };

        let board_coords = translate_game_coords_to_board_coords(Coordinates {
            x: self.coordinates.x,
            y: self.coordinates.y
        });
        let screen_coords = Coordinates {
            x: board_coords.x + self.base.x + 1,
            y: board_coords.y + self.base.y
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

enum Heading {
    North,
    East,
    West,
    South,
}

impl Cursor {
    fn new(x: u16, y: u16, base_x: u16, base_y: u16) -> Cursor {
        Cursor {
            coordinates: Coordinates { x, y },
            base: Coordinates { x: base_x, y: base_y },
        }
    }

    fn on_move(self, heading: Heading) -> Cursor {
        match heading {
            Heading::North => {
                if self.coordinates.y > 0 {
                    Cursor {
                        coordinates: Coordinates {
                            x: self.coordinates.x,
                            y: self.coordinates.y - 1,
                        },
                        base: self.base,
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
                        base: self.base,
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
                        base: self.base,
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
                        base: self.base,
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
            x: board_coords.x + self.base.x,
            y: board_coords.y + self.base.y,
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

impl Board {
    fn new(faction: Faction, width: u16, height: u16, x: u16, y: u16) -> Board {
        Board {
            faction,
            origin: Coordinates { x, y },
            height,
            width,
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

struct Ship {
    length: u16,
    origin: Coordinates,
    heading: Heading
}

impl Ship {
    fn new(origin: Coordinates, heading: Heading, length: u16) -> Ship {
        Ship {
            origin,
            heading,
            length
        }
    }

    fn render(self) {

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

    let red_board = Board::new(Faction::Blue, 8, 8, 1, 2);
    let blue_board = Board::new(Faction::Red, 8, 8, 1, 20);
    let mut cursor = Cursor::new(0, 0, 1, 2);
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
                attacks.push(Attack::new(cursor.coordinates.x, cursor.coordinates.y, red_board.origin.x, red_board.origin.y));
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
}
