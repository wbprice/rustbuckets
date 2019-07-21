use termion::{color, style};

#[derive(Debug)]
struct Game {
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
            turns: 0,
            boards: vec![blue_board, red_board],
        }
    }

    fn render_boards(&self) {
        for board in self.boards.iter() {
            board.render();
        }
    }

    fn render(&self) {
        self.render_boards();
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

    fn render(&self) {
        for _ in 0..self.height {
            for _ in 0..self.width {
                // Print blue waters to start
                print!("{}\u{3000}{}", color::Bg(color::Blue), style::Reset);
            }
            print!("\n");
        }
        print!("\n");
    }
}

fn main() {
    println!(
        "{}Rustbuckets v0.1.0{}",
        color::Fg(color::Red),
        style::Reset
    );

    let game = Game::new();
    game.render();
}
