use crate::{
    models::{
        Ship,
        Coordinates,
        Heading,
        Attack,
        Scores,
        Faction
    },
    controllers::{
        Mode
    }
};

#[derive(Debug, Default)]
pub struct Game {
    pub blue_score: Scores,
    pub red_score: Scores,
    pub blue_ships: Vec<Ship>,
    pub red_ships: Vec<Ship>,
    pub blue_attacks: Box<[Attack]>,
    pub red_attacks: Box<[Attack]>,
    pub active_player: Faction,
    pub mode: Mode
}

impl Game {
    fn toggle_active_player(&mut self) {
        self.active_player = match self.active_player {
            Faction::Blue => Faction::Red,
            Faction::Red => Faction::Blue
        }
    }

    fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn place_ship(&mut self, faction: Faction, ship: Ship) {
        match faction {
            Faction::Blue => {
                self.blue_ships.push(ship);
            },
            Faction::Red => {
                self.red_ships.push(ship);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_active_player() {
        let mut game = Game::default();
        assert_eq!(game.active_player, Faction::Blue);
        game.toggle_active_player();
        assert_eq!(game.active_player, Faction::Red);
    }

    #[test]
    fn test_switch_mode() {
        let mut game = Game::default();
        assert_eq!(game.mode, Mode::Title);
        game.switch_mode(Mode::Play);
        assert_eq!(game.mode, Mode::Play);
    }

    #[test]
    fn test_place_ship() {
        let mut game = Game::default();
        assert_eq!(game.blue_ships.len(), 0);
        game.place_ship(
            Faction::Blue,
            Ship::new(
                Coordinates {
                    x: 0,
                    y: 0
                },
                Heading::South
            )
        );
        assert_eq!(game.blue_ships.len(), 0);
    }
}