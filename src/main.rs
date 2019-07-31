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

#[derive(Debug, Copy, Clone)]
struct Cursor {
    coordinates: Coordinates,
    base: Coordinates,
}

#[derive(Clone, Copy, Debug)]
struct Attack {
    coordinates: Coordinates,
    base: Coordinates,
    result: AttackResults
}

impl Attack {
    fn new(x: u16, y: u16) -> Attack {
        Attack {
            coordinates: Coordinates {
                x,
                y
            },
            base: Coordinates {
                x,
                y
            },
            result: AttackResults::Miss
        }
    }

    fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        let symbol = match self.result {
            AttackResults::Hit => "X",
            AttackResults::Miss => "O"
        };

        write!(
            stdout,
            "{}{}{}{}{}",
            Goto(self.coordinates.x, self.coordinates.y),
            color::Fg(color::White),
            color::Bg(color::Black),
            symbol,
            style::Reset
        ).unwrap();
    }
}

#[derive(Copy, Clone, Debug)]
enum AttackResults {
    Hit,
    Miss
}

enum Heading {
    North,
    East,
    West,
    South,
}

impl Cursor {
    fn new(x: u16, y: u16) -> Cursor {
        Cursor {
            coordinates: Coordinates { x, y },
            base: Coordinates { x, y },
        }
    }

    fn on_move(self, heading: Heading) -> Cursor {
        match heading {
            Heading::North => {
                if self.coordinates.y - self.base.y > 0 {
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
                if self.coordinates.x - self.base.x < 7 {
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
                if self.coordinates.x - self.base.x > 0 {
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
                if self.coordinates.y - self.base.y < 7 {
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
        write!(
            stdout,
            "{}{}[ ]{}",
            Goto(self.coordinates.x, self.coordinates.y),
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

    fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, "{}", Goto(self.origin.x, self.origin.y)).unwrap();
        for _ in 1..self.height + 1 {
            for _ in 1..self.width + 1 {
                // Print blue waters to start
                write!(stdout, "{}\u{3000}{}", color::Bg(color::Blue), style::Reset).unwrap();
            }
            write!(stdout, "\n\r").unwrap();
        }
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
    let blue_board = Board::new(Faction::Red, 8, 8, 1, 11);
    let mut cursor = Cursor::new(1, 2);
    let mut attacks : Vec<Attack> = Vec::new();
    let mut info = Label::new(1, 19, "Hello there".to_string());
    let title = Label::new(1, 1, "Rustbuckets v1.0".to_string());

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
            },
            Key::Char('f') => {
                attacks.push(Attack::new(cursor.coordinates.x, cursor.coordinates.y));
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
                cursor.coordinates.x - cursor.base.x,
                cursor.coordinates.y - cursor.base.y
            ),
        );
        for attack in attacks.clone() {
            attack.render(&mut stdout);
        }
        info.render(&mut stdout);
        stdout.flush().unwrap();
    }
}
