use std::sync::{Arc, Mutex};

use super::map::Map;
use super::player::Player;

pub struct Game {
    pub map: Map,
    players: Vec<Arc<Mutex<Player>>>,
}

impl Game {
    pub fn create(map: Map) -> Arc<Mutex<Self>> {
        let game = Game {
            map,
            players: Vec::new(),
        };
        Arc::new(Mutex::new(game))
    }
}

pub trait GameTrait {
    fn register_player(self: &Arc<Self>, player: &Arc<Mutex<Player>>);
    fn process(self: &Arc<Self>);
}

impl GameTrait for Mutex<Game> {
    fn register_player(self: &Arc<Self>, player: &Arc<Mutex<Player>>) {
        player.lock().unwrap().mount_game(Arc::clone(self));
        let mut game = self.lock().unwrap();
        game.players.push(Arc::clone(player));
    }

    fn process(self: &Arc<Self>) {
        for player_arc in self.lock().unwrap().players.iter() {
            let mut player = player_arc.lock().unwrap();
            player.x += (player.direction * std::f64::consts::PI / 180.0).sin();
            player.y += (player.direction * std::f64::consts::PI / 180.0).cos();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{player::{Player, PlayerTrait}, map::Map};

    use super::{Game, GameTrait};

    #[test]
    fn test_movement() {
        let p = Player::create_with_direction(0.0, 0.0, 0.0);
        let game = Game::create(Map::new(0, 0, 0, 0));
        game.register_player(&p);
        game.process();
        let _ = p.rotate(90.0);
        game.process();
        assert_eq!(p.get_x(), 1.0);
        assert_eq!(p.get_y(), 1.0);
    }
}
