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
    pub fn toggle_active_player(&mut self) {
        self.active_player = match self.active_player {
            Faction::Blue => Faction::Red,
            Faction::Red => Faction::Blue
        }
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn get_ship_list(&self, faction: Faction) -> &Vec<Ship> {
        match faction {
            Faction::Red => &self.red_ships,
            Faction::Blue => &self.blue_ships 
        }
    }

    fn get_ship_list_mut(&mut self, faction: Faction) -> &Vec<Ship> {
        match faction {
            Faction::Red => &self.red_ships,
            Faction::Blue => &self.blue_ships 
        }
    }

    pub fn place_ship(&mut self, faction: Faction, ship: Ship) -> Result<(), &str> {
        let ship_list = self.get_ship_list_mut(faction);
        // if self.should_place_ship(&ship_list, &ship) {
            // self.blue_ships.push(ship);
            Ok(())
        // } else {
            // Err("Can't place ship there")
        // }
    }

    pub fn kiss_ling_ling(&self) {
        println!("{}{}{}", 
            "\u{1F436}",
            "\u{1F48B}", 
            "\u{1F407}", 
        );
    }

    fn should_place_ship(&self, ships: &Vec<Ship>, ship: &Ship) -> bool {
        for ship_segment in ship.segments.iter() {
            if self.is_ship_at_coordinates(&ships, &ship_segment.coordinates) {
                return false;
            }
        }
        true
    }

    fn is_ship_at_coordinates(&self, ships: &Vec<Ship>, coordinates: &Coordinates) -> bool {
        for ship in ships.iter() {
            for ship_segment in ship.segments.iter() {
                if ship_segment.coordinates.x == coordinates.x &&
                   ship_segment.coordinates.y == coordinates.y {
                       return true
                   }
            }
        }
        false
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

    #[test]
    fn test_kiss_ling_ling() {
        let mut game = Game::default();
        game.kiss_ling_ling();
    }
}