use termion::{color, style};
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::cursor::Goto;
use std::io::{Write, stdout, Stdout, stdin};
use termion::input::TermRead;

struct Game {
    boards: Vec<Board>,
    cursor: Coordinates
}

impl Game {
    fn new() -> Game {
        let height = 8;
        let width = 8;

        Game {
            cursor: Coordinates {
                x: 1,
                y: 2
            },
            boards: vec![
                Board::new(Faction::Blue, height, width),
                Board::new(Faction::Red, height, width)
            ]
        }
    }

    fn render_boards(&self, mut stdout: &mut RawTerminal<Stdout>) {
        for board in self.boards.iter() {
            board.render(&mut stdout);
        }
    }

    fn render_cursor(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, "{}{} {}",
            Goto(self.cursor.x as u16, self.cursor.y as u16),
            color::Bg(color::Red),
            style::Reset
        ).unwrap();
    }

    fn render_title(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(
            stdout,
            "{}Rustbuckets v0.1.0{}\n\r",
            color::Fg(color::Red),
            style::Reset
        ).unwrap();
    }

    fn render(&self, mut stdout: &mut RawTerminal<Stdout>) {
        // Clear everything, hide the cursor
        write!(stdout, "{}{}{}", 
            termion::clear::All,
            Goto(1,1),
            termion::cursor::Hide
        ).unwrap();
        self.render_title(&mut stdout);
        self.render_boards(&mut stdout);
        self.render_cursor(&mut stdout);
    }

    fn on_keypress(&self, stdout: &mut RawTerminal<Stdout>) {
        let stdin = stdin();
        write!(stdout, "{}", termion::cursor::Hide).unwrap();
        stdout.flush().unwrap();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => break,
                _ => {}
            }
        }
        stdout.flush().unwrap();
    }

    fn start(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        self.render(&mut stdout);
        self.on_keypress(&mut stdout);
    }
}

#[derive(Debug)]
enum Faction {
    Red,
    Blue,
}

#[derive(Debug)]
struct Board {
    faction: Faction,
    height: i8,
    width: i8,
}

#[derive(Debug)]
struct Coordinates {
    x: i8,
    y: i8
}

impl Board {
    fn new(faction: Faction, width: i8, height: i8) -> Board {
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


    let game = Game::new();
    game.start();
}
