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

    fn place_ship(&mut self, faction: Faction, ship: Ship) -> Result<(), &str> {
        match faction {
            Faction::Blue => {
                self.blue_ships.push(ship);
                Ok(())
            },
            Faction::Red => {
                self.red_ships.push(ship);
                Ok(())
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
    fn test_place_ship_empty_board() {
        let mut game = Game::default();
        assert_eq!(game.blue_ships.len(), 0);
        let result = game.place_ship(
            Faction::Blue,
            Ship::default()
        );
        assert!(result.is_ok());
        assert_eq!(game.blue_ships.len(), 1);
    }

    #[test]
    fn test_place_ship_should_not_share_origin() {
        let mut game = Game::default();
        assert_eq!(game.blue_ships.len(), 0);
        game.place_ship(
            Faction::Blue,
            Ship::default()
        ).unwrap();
        assert_eq!(game.blue_ships.len(), 1);
        let result = game.place_ship(
            Faction::Blue,
            Ship::default()
        );
        assert!(result.is_err());
        assert_eq!(game.blue_ships.len(), 1);
    }

    #[test]
    fn test_ships_should_not_go_off_board() {
        let mut game = Game::default();
        assert_eq!(game.blue_ships.len(), 0);
        game.place_ship(
            Faction::Blue,
            Ship::default()
        ).unwrap();
        assert_eq!(game.blue_ships.len(), 1);
        let result = game.place_ship(
            Faction::Blue,
            Ship::default()
        );
        assert!(result.is_err());
        assert_eq!(game.blue_ships.len(), 1);
    }

    fn test_ships_should_not_overlap_with_other_ships() {
        let mut game = Game::default();
        assert_eq!(game.blue_ships.len(), 0);
        game.place_ship(
            Faction::Blue,
            Ship::default()
        ).unwrap();
        assert_eq!(game.blue_ships.len(), 1);
        let result = game.place_ship(
            Faction::Blue,
            Ship::default()
        );
        assert!(result.is_err());
        assert_eq!(game.blue_ships.len(), 1);
    }
}