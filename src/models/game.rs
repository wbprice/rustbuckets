use crate::{
    models::{
        Ship,
        Coordinates,
        Heading,
        Attack,
        AttackResult,
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
    pub blue_attacks: Vec<Attack>,
    pub red_attacks: Vec<Attack>,
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

    fn increment_hits(&mut self) {
        match self.active_player {
            Faction::Blue => {
                self.blue_score.hits += 1;
            },
            Faction::Red => {
                self.red_score.hits += 1;
            }
        }
    }

    fn increment_misses(&mut self) {
        match self.active_player {
            Faction::Blue => {
                self.blue_score.misses += 1;
            },
            Faction::Red => {
                self.red_score.misses += 1;
            }
        }
    }

    pub fn place_ship(&mut self, ship: Ship) -> Result<(), &str> {
        match self.active_player {
            Faction::Red => {
                if self.should_place_ship(&self.red_ships, &ship) {
                    self.red_ships.push(ship);
                    Ok(())
                } else {
                    Err("Can't place a ship there")
                }
            }
            Faction::Blue => {
                if self.should_place_ship(&self.blue_ships, &ship) {
                    self.blue_ships.push(ship);
                    Ok(())
                } else {
                    Err("Can't place a ship there")
                }
            }
        }
    }

    fn should_place_attack(&self, attacks: &Vec<Attack>, coordinates: &Coordinates) -> bool {
        for attack in attacks.iter() {
            if attack.coordinates.x == coordinates.x &&
               attack.coordinates.y == coordinates.y {
                   return false;
               }
        }
        return true
    }

    pub fn place_attack(&mut self, coordinates: Coordinates) -> Result<(), &str> {
        match self.active_player {
            Faction::Red => {
                if self.should_place_attack(&self.blue_attacks, &coordinates) {
                    let attack = Attack::new(
                        &self.blue_ships,
                        coordinates
                    );
                    match attack.result {
                        AttackResult::Hit => {
                            self.increment_hits();
                        },
                        AttackResult::Miss => {
                            self.increment_misses();
                        }
                    };
                    self.blue_attacks.push(attack);
                    Ok(())
                } else {
                    Err("Can't place an attack there")
                }
            },
            Faction::Blue => {
                if self.should_place_attack(&self.red_attacks, &coordinates) {
                    let attack = Attack::new(
                        &self.red_ships,
                        coordinates
                    );
                    match attack.result {
                        AttackResult::Hit => {
                            self.increment_hits();
                        },
                        AttackResult::Miss => {
                            self.increment_misses();
                        }
                    };
                    self.red_attacks.push(attack);
                    Ok(())
                } else {
                    Err("Can't place an attack there")
                }
            }
        }
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
                return false
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
        let result = game.place_ship(Ship::default());
        assert!(result.is_ok());
        assert_eq!(game.blue_ships.len(), 1);
    }

    #[test]
    fn test_place_ship_should_not_share_origin() {
        let mut game = Game::default();
        assert_eq!(game.blue_ships.len(), 0);
        game.place_ship(Ship::default()).unwrap();
        assert_eq!(game.blue_ships.len(), 1);
        let result = game.place_ship(Ship::default());
        assert!(result.is_err());
        assert_eq!(game.blue_ships.len(), 1);
    }

    #[test]
    fn test_ships_should_not_go_off_board() {
        let mut game = Game::default();
        assert_eq!(game.blue_ships.len(), 0);
        game.place_ship(Ship::default()).unwrap();
        assert_eq!(game.blue_ships.len(), 1);
        let result = game.place_ship(Ship::default());
        assert!(result.is_err());
        assert_eq!(game.blue_ships.len(), 1);
    }

    #[test]
    fn test_ships_should_not_overlap_with_other_ships() {
        let mut game = Game::default();
        assert_eq!(game.blue_ships.len(), 0);
        game.place_ship(Ship::default()).unwrap();
        assert_eq!(game.blue_ships.len(), 1);
        let result = game.place_ship(Ship::default());
        assert!(result.is_err());
        assert_eq!(game.blue_ships.len(), 1);
    }

    #[test]
    fn test_new_attack_hit() {
        let mut game = Game::default();
        game.place_ship(Ship::default()).unwrap();
        assert_eq!(game.blue_ships.len(), 1);
        game.toggle_active_player();
        game.place_attack(Coordinates { x: 0, y: 0 }).unwrap();
        assert_eq!(game.blue_attacks.len(), 1);
        assert_eq!(game.blue_attacks[0].result, AttackResult::Hit);
    }

    #[test]
    fn test_new_attack_miss() {
        let mut game = Game::default();
        game.place_ship(Ship::default()).unwrap();
        assert_eq!(game.blue_ships.len(), 1);
        game.toggle_active_player();
        game.place_attack(Coordinates { x: 2, y: 2 }).unwrap();
        assert_eq!(game.blue_attacks.len(), 1);
        assert_eq!(game.blue_attacks[0].result, AttackResult::Miss);
    }

    #[test]
    fn test_new_attack_already_made() {
        let mut game = Game::default();
        game.place_ship(Ship::default()).unwrap();
        assert_eq!(game.blue_ships.len(), 1);
        game.toggle_active_player();
        game.place_attack(Coordinates { x: 0, y: 0 }).unwrap();
        let result = game.place_attack(Coordinates { x: 0, y: 0});
        assert!(result.is_err());
        assert_eq!(game.blue_attacks.len(), 1);
    }

    #[test]
    fn test_kiss_ling_ling() {
        let game = Game::default();
        game.kiss_ling_ling();
        let love = true;
        assert!(love);
    }
}