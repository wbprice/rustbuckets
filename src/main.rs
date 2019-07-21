use termion::{color, style};
use termion::cursor::Goto;
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, stdout, Stdout};


struct Game {
    should_exit: bool,
    turns: i8,
    boards: Vec<Board>,
}

impl Game {
    fn new() -> Game {
        let height = 8;
        let width = 8;

        let blue_board = Board::new(Faction::Blue, height, width);
        let red_board = Board::new(Faction::Red, height, width);

        Game {
            should_exit: false,
            turns: 0,
            boards: vec![blue_board, red_board],
        }
    }

    fn render_boards(&self, mut stdout: &mut RawTerminal<Stdout>) {
        for board in self.boards.iter() {
            board.render(&mut stdout);
        }
    }

    fn render(&self, mut stdout: &mut RawTerminal<Stdout>) {
        self.render_boards(&mut stdout);
    }

    fn start(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        self.render(&mut stdout);
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
    println!(
        "{}Rustbuckets v0.1.0{}",
        color::Fg(color::Red),
        style::Reset
    );

    let game = Game::new();
    game.start();
}
