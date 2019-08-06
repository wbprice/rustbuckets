use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    thread_rng, Rng,
};
use std::io::{stdin, stdout, Stdin, Stdout, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, style};



#[derive(Clone, Copy)]
struct Scores {
    hits: u16,
    misses: u16,
}

impl Scores {
    fn new() -> Scores {
        Scores { hits: 0, misses: 0 }
    }
}

#[derive(Copy, Clone)]
enum Mode {
    Title,
    Game,
    Endscreen,
    Quit,
}

#[derive(Clone, Copy)]
struct Game {
    blue_score: Scores,
    red_score: Scores,
    turn: Faction,
    origin: Coordinates,
    mode: Mode,
}

impl Game {
    fn new(origin: Coordinates) -> Game {
        Game {
            blue_score: Scores::new(),
            red_score: Scores::new(),
            turn: Faction::Blue,
            origin,
            mode: Mode::Title,
        }
    }

    fn increment_hits(self) -> Game {
        match self.turn {
            Faction::Blue => {
                let mut game = self.clone();
                game.blue_score.hits = game.blue_score.hits + 1;
                game
            }
            Faction::Red => {
                let mut game = self.clone();
                game.red_score.hits = game.red_score.hits + 1;
                game
            }
        }
    }

    fn increment_misses(self) -> Game {
        match self.turn {
            Faction::Blue => {
                let mut game = self.clone();
                game.blue_score.misses = game.blue_score.misses + 1;
                game
            }
            Faction::Red => {
                let mut game = self.clone();
                game.red_score.misses = game.red_score.misses + 1;
                game
            }
        }
    }

    fn switch_players(self) -> Game {
        match self.turn {
            Faction::Blue => {
                let mut game = self.clone();
                game.turn = Faction::Red;
                game
            }
            Faction::Red => {
                let mut game = self.clone();
                game.turn = Faction::Blue;
                game
            }
        }
    }

    fn toggle_mode(self, mode: Mode) -> Game {
        let mut game = self.clone();
        match mode {
            Mode::Title => {
                game.mode = Mode::Title;
                game
            }
            Mode::Game => {
                game.mode = Mode::Game;
                game
            }
            Mode::Endscreen => {
                game.mode = Mode::Endscreen;
                game
            }
            Mode::Quit => {
                game.mode = Mode::Quit;
                game
            }
        }
    }

    fn render(self, stdout: &mut RawTerminal<Stdout>) {
        writeln!(
            stdout,
            "{}{}{}{}{}Hits: {}{}Misses: {}{}",
            Goto(self.origin.x, self.origin.y),
            color::Fg(color::Blue),
            "Blue Team".to_string(),
            color::Fg(color::White),
            Goto(self.origin.x, self.origin.y + 1),
            self.blue_score.hits,
            Goto(self.origin.x, self.origin.y + 2),
            self.blue_score.misses,
            style::Reset
        )
        .unwrap();

        writeln!(
            stdout,
            "{}{}{}{}{}Hits: {}{}Misses: {}{}",
            Goto(self.origin.x, self.origin.y + 4),
            color::Fg(color::Red),
            "Red Team".to_string(),
            color::Fg(color::White),
            Goto(self.origin.x, self.origin.y + 5),
            self.red_score.hits,
            Goto(self.origin.x, self.origin.y + 6),
            self.red_score.misses,
            style::Reset
        )
        .unwrap();
    }
}

#[derive(Debug, Clone, Copy)]
enum Faction {
    Red,
    Blue,
}

impl Default for Faction {
    fn default() -> Self {
        Faction::Red
    }
}

#[derive(Debug, Copy, Clone, Default)]
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
    fn new(coordinates: Coordinates, board: &'a Board, ships: &'a Vec<Ship>) -> Attack<'a> {
        let mut hit = false;
        for ship in ships.into_iter() {
            for segment in ship.segments.iter() {
                if segment.coordinates.x == coordinates.x && segment.coordinates.y == coordinates.y
                {
                    hit = true;
                }
            }
        }

        if hit {
            Attack {
                coordinates,
                board,
                result: AttackResults::Hit,
            }
        } else {
            Attack {
                coordinates,
                board,
                result: AttackResults::Miss,
            }
        }
    }

    fn render(self, stdout: &mut RawTerminal<Stdout>) {
        let symbol = match self.result {
            AttackResults::Hit => "X",
            AttackResults::Miss => "^",
        };

        let board_coords = translate_game_coords_to_board_coords(Coordinates {
            x: self.coordinates.x,
            y: self.coordinates.y,
        });
        let screen_coords = Coordinates {
            x: board_coords.x + self.board.origin.x + 1,
            y: board_coords.y + self.board.origin.y,
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

#[derive(Debug, Clone, Copy)]
enum Heading {
    North,
    East,
    West,
    South,
}

impl Default for Heading {
    fn default() -> Self {
        Heading::North
    }
}

impl Distribution<Heading> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Heading {
        match rng.gen_range(0, 3) {
            0 => Heading::North,
            1 => Heading::East,
            2 => Heading::West,
            3 => Heading::South,
            _ => Heading::North,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Cursor<'a> {
    coordinates: Coordinates,
    board: &'a Board,
}

impl<'a> Cursor<'a> {
    fn new(coordinates: Coordinates, board: &'a Board) -> Cursor<'a> {
        Cursor { coordinates, board }
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
                        board: self.board,
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
                        board: self.board,
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
                        board: self.board,
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

#[derive(Debug, Default)]
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
    fn new(origin: Coordinates, content: String) -> Label {
        Label { origin, content }
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
    segments: Vec<ShipSegment>,
}


impl<'a> Ship<'a> {
    fn new(origin: Coordinates, board: &'a Board, heading: Heading, length: u16) -> Ship {
        let mut segments: Vec<ShipSegment> = vec![];
        // For n segments in
        for n in 0..length {
            match heading {
                Heading::North => segments.push(ShipSegment::new(Coordinates {
                    x: origin.x,
                    y: origin.y - n,
                })),
                Heading::East => segments.push(ShipSegment::new(Coordinates {
                    x: origin.x + n,
                    y: origin.y,
                })),
                Heading::West => segments.push(ShipSegment::new(Coordinates {
                    x: origin.x - n,
                    y: origin.y,
                })),
                Heading::South => segments.push(ShipSegment::new(Coordinates {
                    x: origin.x,
                    y: origin.y + n,
                })),
            }
        }

        Ship {
            origin,
            board,
            heading,
            segments,
        }
    }

    fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        for segment in self.segments.iter() {
            let board_coords = translate_game_coords_to_board_coords(segment.coordinates);
            let screen_coords = Coordinates {
                x: board_coords.x + self.board.origin.x,
                y: board_coords.y + self.board.origin.y,
            };

            write!(
                stdout,
                "{}{}   {}",
                Goto(screen_coords.x, screen_coords.y),
                color::Bg(color::Red),
                style::Reset
            )
            .unwrap();
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ShipSegment {
    coordinates: Coordinates,
}

impl ShipSegment {
    fn new(coordinates: Coordinates) -> ShipSegment {
        ShipSegment { coordinates }
    }
}

fn auto_select_origin(board: &Board) -> Coordinates {
    let mut rng = rand::thread_rng();
    // Randomly choose origin
    Coordinates {
        x: rng.gen_range(0, board.width),
        y: rng.gen_range(0, board.height),
    }
}

fn is_legal_heading(origin: Coordinates, heading: Heading, length: u16) -> bool {
    match heading {
        Heading::North => {
            // There should be enough room to place the ship heading north
            if origin.y >= length {
                true
            } else {
                false
            }
        }
        Heading::South => {
            // There should be enough room to place the ship heading south
            if 8 - origin.y >= length {
                true
            } else {
                false
            }
        }
        Heading::West => {
            // There should be enough room to place the ship heading west
            if origin.x >= length {
                true
            } else {
                false
            }
        }
        Heading::East => {
            // There should be enough room to place the ship heading east
            if 8 - origin.x >= length {
                true
            } else {
                false
            }
        }
    }
}

fn is_ship_at_coordinates(ships: &Vec<Ship>, coordinates: Coordinates) -> bool {
    let mut result = false;
    for ship in ships {
        for segment in ship.segments.iter() {
            if coordinates.x == segment.coordinates.x && coordinates.y == segment.coordinates.y {
                result = true;
                break;
            }
        }
    }
    return result;
}

fn is_legal_ship_placement(ships: &Vec<Ship>, new_ship: Ship) -> bool {
    let mut result = true;
    for ship in ships {
        // If a ship segement belonging to new_ship is also in ship
        // the ship placement is not legal
        for new_segment in new_ship.segments.iter() {
            for segment in ship.segments.iter() {
                if new_segment.coordinates.x == segment.coordinates.x
                    && new_segment.coordinates.y == segment.coordinates.y
                {
                    result = false;
                    break;
                }
            }
        }
    }
    result
}

fn autocreate_ship<'a>(ships: &Vec<Ship>, board: &'a Board, length: u16) -> Ship<'a> {
    loop {
        // Create an origin
        // Any origin on the board is legal as long as there's not already a ship there
        let mut origin = auto_select_origin(board);
        let mut origin_is_legal = !is_ship_at_coordinates(ships, origin);

        // Select a heading
        // Any heading that doesn't lead the ship off the board is valid
        let mut heading: Heading = rand::random();
        let mut heading_is_legal = is_legal_heading(origin, heading, length);

        // If origin isn't legal, pick another one randomly until it is
        while !origin_is_legal {
            origin = auto_select_origin(board);
            origin_is_legal = is_ship_at_coordinates(ships, origin);
        }

        // Iterate through possible headings to find a legal heading
        let mut random = thread_rng();
        let mut headings = vec![Heading::North, Heading::East, Heading::West, Heading::South];
        headings.shuffle(&mut random);

        for h in headings {
            if is_legal_heading(origin, h, length) {
                heading = h;
                heading_is_legal = true;
                break;
            }
        }

        // If both heading and origin aren't good, we can't continue.
        // Try again.
        if origin_is_legal && heading_is_legal {
            // Create ship with legal origin and legal heading
            let ship = Ship::new(origin, board, heading, length);
            // If the newly created ship doesn't collide with existing ships,
            // it's legal.
            // Otherwise, find a new origin and heading and try again.
            if is_legal_ship_placement(ships, ship) {
                return Ship::new(origin, board, heading, length);
            }
        }
    }
}

fn main() {
    let mut game = Game::new(Coordinates { x: 38, y: 2 });

    loop {
        match game.mode {
            Mode::Title => {
                // Title mode setup
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

                // Create entities
                let title = Label::new(Coordinates { x: 1, y: 1 }, "Rustbuckets".to_string());
                let instructions =
                    Label::new(Coordinates { x: 1, y: 2 }, "Press F to start".to_string());
                let quit_instructions =
                    Label::new(Coordinates { x: 1, y: 2 }, "Press F to start".to_string());

                // Initial render
                title.render(&mut stdout);
                instructions.render(&mut stdout);
                quit_instructions.render(&mut stdout);
                stdout.flush().unwrap();

                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('q') => {
                            game = game.toggle_mode(Mode::Quit);
                            break;
                        }
                        Key::Char('f') => {
                            game = game.toggle_mode(Mode::Game);
                            break;
                        }
                        _ => {}
                    }

                    // Rerender after handling input
                    title.render(&mut stdout);
                    instructions.render(&mut stdout);
                    quit_instructions.render(&mut stdout);
                    stdout.flush().unwrap();
                }
            }
            Mode::Game => {
                // Clear all
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

                // Instantiate game entities
                let red_board = Board::new(Faction::Blue, Coordinates { x: 1, y: 2 }, 8, 8);
                let blue_board = Board::new(Faction::Red, Coordinates { x: 1, y: 20 }, 8, 8);
                let mut cursor = Cursor::new(Coordinates { x: 0, y: 0 }, &red_board);
                let mut attacks: Vec<Attack> = Vec::new();
                let mut ships: Vec<Ship> = Vec::new();
                let title =
                    Label::new(Coordinates { x: 1, y: 1 }, "Rustbuckets v0.1.0".to_string());

                // Put some ships in the red_board
                for length in vec![2, 2, 3, 4, 5] {
                    ships.push(autocreate_ship(&ships, &red_board, length));
                }

                // Initial render
                red_board.render(&mut stdout);
                blue_board.render(&mut stdout);
                title.render(&mut stdout);
                game.render(&mut stdout);
                for ship in ships.iter() {
                    ship.render(&mut stdout);
                }
                for attack in attacks.clone() {
                    attack.render(&mut stdout);
                }
                cursor.render(&mut stdout);
                stdout.flush().unwrap();

                // Handle user inputs and render interface
                for c in stdin.keys() {

                    match c.unwrap() {
                        Key::Char('q') => {
                            game = game.toggle_mode(Mode::Title);
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
                            let attack = Attack::new(cursor.coordinates, &red_board, &ships);
                            game = match attack.result {
                                AttackResults::Hit => game.increment_hits(),
                                AttackResults::Miss => game.increment_misses(),
                            };
                            attacks.push(attack);
                        }
                        _ => {}
                    }

                    if game.blue_score.hits >= 17 || game.red_score.hits >= 17 {
                        game = game.toggle_mode(Mode::Endscreen);
                        break;
                    }

                    // Initial render
                    red_board.render(&mut stdout);
                    blue_board.render(&mut stdout);
                    title.render(&mut stdout);
                    game.render(&mut stdout);
                    for ship in ships.iter() {
                        ship.render(&mut stdout);
                    }
                    for attack in attacks.clone() {
                        attack.render(&mut stdout);
                    }
                    cursor.render(&mut stdout);
                    stdout.flush().unwrap();
                }
            }
            Mode::Endscreen => {
                // Endscreen mode setup
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

                // Create entities
                let title = Label::new(Coordinates { x: 1, y: 1 }, "Game End".to_string());
                let quit_instructions =
                    Label::new(Coordinates { x: 1, y: 2 }, "Press Q to quit".to_string());
                let replay_instructions = Label::new(
                    Coordinates { x: 1, y: 3 },
                    "Press F to play again".to_string(),
                );

                // Initial render
                title.render(&mut stdout);
                quit_instructions.render(&mut stdout);
                replay_instructions.render(&mut stdout);
                stdout.flush().unwrap();

                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('q') => {
                            game = game.toggle_mode(Mode::Quit);
                            break;
                        }
                        Key::Char('f') => {
                            game = Game::new(Coordinates { x: 38, y: 2 });
                            game = game.toggle_mode(Mode::Game);
                            break;
                        }
                        _ => {}
                    }

                    // Rerender after handling input
                    title.render(&mut stdout);
                    quit_instructions.render(&mut stdout);
                    replay_instructions.render(&mut stdout);
                    stdout.flush().unwrap();
                }
            }
            Mode::Quit => {
                let mut stdout = stdout().into_raw_mode().unwrap();
                write!(stdout, "{}", style::Reset).unwrap();
                break;
            }
        }
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
        let board = Board::new(Faction::Blue, Coordinates { x: 1, y: 2 }, 8, 8);
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
        let board = Board::new(Faction::Blue, Coordinates { x: 1, y: 2 }, 8, 8);
        let ship = Ship::new(origin, &board, Heading::South, 3);

        assert_eq!(ship.segments.len(), 3);
        assert_eq!(ship.segments[0].coordinates.x, 0);
        assert_eq!(ship.segments[0].coordinates.y, 0);
        assert_eq!(ship.segments[1].coordinates.x, 0);
        assert_eq!(ship.segments[1].coordinates.y, 1);
        assert_eq!(ship.segments[2].coordinates.x, 0);
        assert_eq!(ship.segments[2].coordinates.y, 2);
    }

    #[test]
    fn test_auto_select_origin() {
        let board = Board::new(Faction::Blue, Coordinates { x: 1, y: 2 }, 8, 8);
        let origin = auto_select_origin(&board);

        assert!(origin.x <= 7);
        assert!(origin.x >= 0);
        assert!(origin.y >= 0);
        assert!(origin.y <= 7);
    }

    #[test]
    fn test_is_legal_heading_north_0_0() {
        let origin = Coordinates { x: 0, y: 0 };
        let result = is_legal_heading(origin, Heading::North, 2);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_legal_heading_north_0_2() {
        let origin = Coordinates { x: 0, y: 2 };
        let result = is_legal_heading(origin, Heading::North, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_legal_heading_south_0_7() {
        let origin = Coordinates { x: 0, y: 7 };
        let result = is_legal_heading(origin, Heading::South, 2);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_legal_heading_south_0_5() {
        let origin = Coordinates { x: 0, y: 5 };
        let result = is_legal_heading(origin, Heading::South, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_legal_heading_west_0_0() {
        let origin = Coordinates { x: 0, y: 0 };
        let result = is_legal_heading(origin, Heading::West, 2);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_legal_heading_west_0_2() {
        let origin = Coordinates { x: 0, y: 2 };
        let result = is_legal_heading(origin, Heading::West, 2);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_legal_ship_placement_empty_board() {
        let board = Board::new(Faction::Blue, Coordinates { x: 0, y: 0 }, 8, 8);
        let ships: Vec<Ship> = vec![];
        let tentative_ship = Ship::new(Coordinates { x: 0, y: 1 }, &board, Heading::South, 2);
        let result = is_legal_ship_placement(&ships, tentative_ship);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_legal_ship_placement_non_empty_board() {
        let board = Board::new(Faction::Blue, Coordinates { x: 0, y: 0 }, 8, 8);
        let ships: Vec<Ship> = vec![Ship::new(
            Coordinates { x: 0, y: 1 },
            &board,
            Heading::South,
            2,
        )];
        let tentative_ship = Ship::new(Coordinates { x: 0, y: 1 }, &board, Heading::South, 2);
        let result = is_legal_ship_placement(&ships, tentative_ship);

        assert_eq!(result, false);
    }

    #[test]
    fn test_autocreate_ship_empty_board() {
        let board = Board::new(Faction::Blue, Coordinates { x: 0, y: 0 }, 8, 8);
        let mut ships: Vec<Ship> = vec![];
        let ship1 = autocreate_ship(&ships, &board, 2);
        ships.push(ship1);
        assert_eq!(ships.len(), 1);
    }

    #[test]
    fn test_autocreate_ship_5_ships() {
        let board = Board::new(Faction::Blue, Coordinates { x: 0, y: 0 }, 8, 8);
        let mut ships: Vec<Ship> = vec![];

        for length in vec![2, 2, 3, 4, 5] {
            let ship = autocreate_ship(&ships, &board, length);
            ships.push(ship);
        }
        assert_eq!(ships.len(), 5);
    }
}