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
        let game = self.lock().unwrap();
        for player_arc in game.players.iter() {
            let mut player = player_arc.lock().unwrap();

            // Calculate next coordinates
            let mut next_x =
                player.x + (player.direction * std::f64::consts::PI / 180.0).sin() * player.speed;
            let mut next_y =
                player.y + (player.direction * std::f64::consts::PI / 180.0).cos() * player.speed;

            // Borders collision detection and handling
            if next_x - player.r < 0.0 {
                next_x = player.r;
            } else if next_x + player.r > game.map.width {
                next_x = game.map.width - player.r;
            }
            if next_y - player.r < 0.0 {
                next_y = player.r;
            } else if next_y + player.r > game.map.height {
                next_y = game.map.height - player.r;
            }

            // Barriers collision detection
            for barrier in game.map.barriers.iter() {
                let distance = ((next_x - barrier.x).powi(2) + (next_y - barrier.y).powi(2)).sqrt();
                if distance < (player.r + barrier.r) {
                    // Don't move player if detect collision
                    next_x = player.x;
                    next_y = player.y;
                    break;
                }
            }

            // Players collision detection
            // todo

            player.x = next_x;
            player.y = next_y;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        map::{Barrier, Map},
        player::{Player, PlayerTrait},
    };

    use super::{Game, GameTrait};

    const SEED: u64 = 12345;
    #[test]
    fn test_movement() {
        let p = Player::create_with_direction(1.0, 1.0, 1.0, 1.0, 0.0);
        let game = Game::create(Map::new(100.0, 100.0, 0, 0.0, SEED));
        game.register_player(&p);
        p.set_speed(0.5);

        game.process();
        p.rotate(90.0);
        game.process();

        assert_eq!(p.get_x(), 1.5);
        assert_eq!(p.get_y(), 1.5);
    }

    #[test]
    fn test_borders_collision() {
        let p = Player::create_with_direction(1.0, 1.0, 0.5, 1.0, -180.0);
        let game = Game::create(Map::new(100.0, 100.0, 0, 0.0, SEED));
        game.register_player(&p);
        p.set_speed(1.0);

        game.process();
        p.rotate(90.0);
        game.process();

        assert_eq!((p.get_x() * 100.0).round() / 100.0, 0.5);
        assert_eq!((p.get_y() * 100.0).round() / 100.0, 0.5);
    }

    #[test]
    fn test_barriers_collision() {
        let p = Player::create_with_direction(1.0, 1.0, 1.0, 1.0, 0.0);
        let mut map = Map::new(100.0, 100.0, 0, 0.0, SEED);
        map.barriers.push(Barrier {
            x: 1.0,
            y: 3.0,
            r: 1.0,
        });
        map.barriers.push(Barrier {
            x: 3.0,
            y: 1.0,
            r: 1.0,
        });
        let game = Game::create(map);
        game.register_player(&p);
        p.set_speed(1.0);

        game.process();
        p.rotate(90.0);
        game.process();

        assert_eq!((p.get_x() * 100.0).round() / 100.0, 1.0);
        assert_eq!((p.get_y() * 100.0).round() / 100.0, 1.0);
    }
}
