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
}

impl GameTrait for Mutex<Game> {
    fn register_player(self: &Arc<Self>, player: &Arc<Mutex<Player>>) {
        player.lock().unwrap().mount_game(Arc::clone(self));
        let mut game = self.lock().unwrap();
        game.players.push(Arc::clone(player));
    }
}

#[cfg(test)]
mod tests {}
