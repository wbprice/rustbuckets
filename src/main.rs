use termion::{color, style};
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::cursor::Goto;
use std::io::{Write, stdout, Stdout, stdin};
use termion::input::TermRead;

#[derive(Debug)]
enum Faction {
    Red,
    Blue,
}

#[derive(Debug)]
struct Board {
    faction: Faction,
    height: u16,
    width: u16,
}

#[derive(Debug, Copy, Clone)]
struct Coordinates {
    x: u16,
    y: u16
}

#[derive(Debug, Copy, Clone)]
struct Cursor {
    coordinates: Coordinates
}

enum Heading {
    North,
    East,
    West,
    South
}

impl Cursor {
    fn new(x: u16, y: u16) -> Cursor {
        Cursor {
            coordinates: Coordinates {
                x,
                y
            }
        }
    }

    fn on_move(self, heading: Heading) -> Cursor {
        match heading {
            Heading::North => Cursor {
                coordinates: Coordinates {
                    x: self.coordinates.x,
                    y: self.coordinates.y - 1
                }
            },
            Heading::East => Cursor {
                coordinates: Coordinates {
                    x: self.coordinates.x + 1,
                    y: self.coordinates.y
                }
            },
            Heading::West => Cursor { 
                coordinates: Coordinates {
                    x: self.coordinates.x - 1,
                    y: self.coordinates.y
                }
            },
            Heading::South => Cursor {
                coordinates: Coordinates {
                    x: self.coordinates.x,
                    y: self.coordinates.y + 1
                }
            }
        }
    }

    fn render(self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, "{}{}Q{}x: {}, y: {}{}",
            Goto(self.coordinates.x, self.coordinates.y),
            color::Fg(color::Red),
            Goto(1, 18),
            self.coordinates.x,
            self.coordinates.y,
            style::Reset
        ).unwrap()
    }
}

impl Board {
    fn new(faction: Faction, width: u16, height: u16) -> Board {
        Board {
            faction,
            height,
            width
        }
    }

    fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        for _ in 1..self.height + 1 {
            for _ in 1..self.width + 1 {
                // Print blue waters to start
                write!(stdout, "{}\u{3000}{}", 
                    color::Bg(color::Blue), 
                    style::Reset).unwrap();
            }
            write!(stdout, "\n\r").unwrap();
        }
        write!(stdout, "\n\r").unwrap();
    }
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(stdout, "{}{}{}", 
        termion::clear::All,
        Goto(1,1),
        termion::cursor::Hide
    ).unwrap();

    let red_board = Board::new(Faction::Blue, 8, 8);
    let blue_board = Board::new(Faction::Red, 8, 8);
    let mut cursor = Cursor::new(1, 1);

    red_board.render(&mut stdout);
    blue_board.render(&mut stdout);
    cursor.render(&mut stdout);

    // Handle user inputs and render interface
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                write!(stdout, "{}", style::Reset).unwrap();
                break;
            },
            Key::Char('w') => {
                if cursor.coordinates.y > 1 {
                    cursor = cursor.on_move(Heading::North);
                }
            }, 
            Key::Char('a') => {
                if cursor.coordinates.x > 1 {
                    cursor = cursor.on_move(Heading::West);
                }
            },
            Key::Char('s') => {
                if cursor.coordinates.y < 8 {
                    cursor = cursor.on_move(Heading::South);
                }
            },
            Key::Char('d') => {
                if cursor.coordinates.x < 8 {
                    cursor = cursor.on_move(Heading::East);
                }
            },
            _ => {}
        }

        red_board.render(&mut stdout);
        blue_board.render(&mut stdout);
        cursor.render(&mut stdout);

        stdout.flush().unwrap();
    }
}
