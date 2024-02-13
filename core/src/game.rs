use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

use super::map::Map;
use super::player::{Missile, Player};

const TIME_STEP: f64 = 0.1;

pub struct Game {
    pub map: Map,
    pub players: Vec<Arc<Mutex<Player>>>,
    pub missiles: Vec<Missile>,
}

impl Game {
    pub fn create(map: Map) -> Arc<Mutex<Self>> {
        let game = Game {
            map,
            players: Vec::new(),
            missiles: Vec::new(),
        };
        Arc::new(Mutex::new(game))
    }
}

pub trait GameTrait {
    fn get_missiles(self: &Arc<Self>) -> Vec<Missile>;
    fn register_player(self: &Arc<Self>, player: &Arc<Mutex<Player>>);
    fn process(self: &Arc<Self>, time: f64);
}

impl GameTrait for Mutex<Game> {
    fn get_missiles(self: &Arc<Self>) -> Vec<Missile> {
        self.lock()
            .unwrap()
            .missiles
            .iter()
            .map(|m| Missile {
                x: m.x,
                y: m.y,
                direction: m.direction,
                id: m.id,
                player_id: m.player_id,
                speed: m.speed,
            })
            .collect()
    }

    fn register_player(self: &Arc<Self>, player: &Arc<Mutex<Player>>) {
        player.lock().unwrap().mount_game(Arc::clone(self));
        let mut game = self.lock().unwrap();
        game.players.push(Arc::clone(player));
    }

    fn process(self: &Arc<Self>, time: f64) {
        let mut game = self.lock().unwrap();
        let Game {
            map,
            missiles,
            players,
        } = &mut *game;

        let mut time_left = time;

        loop {
            let mut timedelta = TIME_STEP;
            if time_left > TIME_STEP {
                time_left -= TIME_STEP;
            } else if time_left <= TIME_STEP && time_left > 0.0 {
                timedelta = TIME_STEP;
                time_left -= TIME_STEP;
            } else {
                break;
            }
            time_left = (time_left * 10000.0).round() / 10000.0;

            for player_arc in players.iter() {
                let mut player = player_arc.lock().unwrap();

                // Calculate next coordinates

                let mut next_x =
                    player.x + (player.direction * PI / 180.0).sin() * player.speed * timedelta;
                let mut next_y =
                    player.y + (player.direction * PI / 180.0).cos() * player.speed * timedelta;

                // Borders collision detection and handling

                if next_x - player.r < 0.0 {
                    next_x = player.r;
                } else if next_x + player.r > map.width {
                    next_x = map.width - player.r;
                }
                if next_y - player.r < 0.0 {
                    next_y = player.r;
                } else if next_y + player.r > map.height {
                    next_y = map.height - player.r;
                }

                // Barriers collision detection

                for barrier in map.barriers.iter() {
                    let distance =
                        ((next_x - barrier.x).powi(2) + (next_y - barrier.y).powi(2)).sqrt();
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

            for missile in missiles.iter_mut() {
                missile.x += (missile.direction * PI / 180.0).sin() * missile.speed * timedelta;
                missile.y += (missile.direction * PI / 180.0).cos() * missile.speed * timedelta;
            }

            // Borders collision

            missiles.retain(|m| m.x >= 0.0 && m.y >= 0.0 && m.x <= map.width && m.y <= map.height);

            // Barriers collision

            missiles.retain(|m| {
                map.barriers
                    .iter()
                    .all(|b| ((m.x - b.x).powi(2) + (m.y - b.y).powi(2)).sqrt() >= b.r)
            });

            // Players collision

            missiles.retain(|m| {
                players
                    .iter()
                    .map(|p| p.lock().unwrap())
                    .all(|p| ((m.x - p.x).powi(2) + (m.y - p.y).powi(2)).sqrt() >= p.r)
            });
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
    const MISSILE_SPEED: f64 = 1.0;

    #[test]
    fn test_movement() {
        let p = Player::create_with_direction(1.0, 1.0, 1.0, 1.0, 60.0, 7, 0.0, MISSILE_SPEED);
        let game = Game::create(Map::new(100.0, 100.0, 0, 0.0, SEED));
        game.register_player(&p);
        p.set_speed(0.5);

        game.process(1.0);
        p.rotate(90.0);
        game.process(1.0);

        assert_eq!((p.get_x() * 100.0).round() / 100.0, 1.5);
        assert_eq!((p.get_y() * 100.0).round() / 100.0, 1.5);
    }

    #[test]
    fn test_borders_collision() {
        let p = Player::create_with_direction(1.0, 1.0, 0.5, 1.0, 60.0, 7, -180.0, MISSILE_SPEED);
        let game = Game::create(Map::new(100.0, 100.0, 0, 0.0, SEED));
        game.register_player(&p);
        p.set_speed(1.0);

        game.process(1.0);
        p.rotate(90.0);
        game.process(1.0);

        assert_eq!((p.get_x() * 100.0).round() / 100.0, 0.5);
        assert_eq!((p.get_y() * 100.0).round() / 100.0, 0.5);
    }

    #[test]
    fn test_barriers_collision() {
        let p = Player::create_with_direction(1.0, 1.0, 1.0, 1.0, 60.0, 7, 0.0, MISSILE_SPEED);
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

        game.process(1.0);
        p.rotate(90.0);
        game.process(1.0);

        assert_eq!((p.get_x() * 100.0).round() / 100.0, 1.0);
        assert_eq!((p.get_y() * 100.0).round() / 100.0, 1.0);
    }

    #[test]
    fn test_missiles_movement() {
        const START_X: f64 = 1.0;
        const START_Y: f64 = 1.0;

        let p =
            Player::create_with_direction(START_X, START_Y, 1.0, 1.0, 60.0, 7, 0.0, MISSILE_SPEED);
        let game = Game::create(Map::new(100.0, 100.0, 0, 0.0, SEED));
        game.register_player(&p);

        let missiles = game.get_missiles();
        assert_eq!(missiles.len(), 0);

        p.fire();
        p.rotate(90.0);
        p.fire();
        p.rotate(90.0);
        game.process(1.0);

        let missiles = game.get_missiles();

        assert_eq!(missiles.len(), 2);

        assert_eq!(missiles[0].speed, MISSILE_SPEED);
        assert_eq!(missiles[0].x, START_X);
        assert_eq!(missiles[0].y, START_Y + MISSILE_SPEED);

        assert_eq!(missiles[1].speed, MISSILE_SPEED);
        assert_eq!(missiles[1].x, START_X + MISSILE_SPEED);
        assert_eq!(missiles[1].y, START_Y);

        game.process(4.0);

        let missiles = game.get_missiles();

        assert_eq!(missiles.len(), 2);

        assert_eq!(missiles[0].speed, MISSILE_SPEED);
        assert_eq!(missiles[0].x, START_X);
        assert_eq!(missiles[0].y, START_Y + MISSILE_SPEED * 5.0);

        assert_eq!(missiles[1].speed, MISSILE_SPEED);
        assert_eq!(missiles[1].x, START_X + MISSILE_SPEED * 5.0);
        assert_eq!(missiles[1].y, START_Y);
    }

    #[test]
    fn test_missiles_borders_collision() {
        const START_X: f64 = 50.0;
        const START_Y: f64 = 50.0;

        let p =
            Player::create_with_direction(START_X, START_Y, 1.0, 1.0, 60.0, 7, 0.0, MISSILE_SPEED);
        let game = Game::create(Map::new(100.0, 100.0, 0, 0.0, SEED));
        game.register_player(&p);

        p.fire();
        p.rotate(90.0);
        p.fire();
        p.rotate(90.0);
        p.fire();
        p.rotate(90.0);
        p.fire();
        p.rotate(90.0);

        game.process(50.0);

        let missiles = game.get_missiles();
        assert_eq!(missiles.len(), 4);

        assert_eq!(missiles[0].x, START_X);
        assert_eq!(missiles[0].y, START_Y + MISSILE_SPEED * 50.0);

        assert_eq!(missiles[1].x, START_X + MISSILE_SPEED * 50.0);
        assert_eq!(missiles[1].y, START_Y);

        assert_eq!(missiles[2].x, START_X);
        assert_eq!(missiles[2].y, START_Y - MISSILE_SPEED * 50.0);

        assert_eq!(missiles[3].x, START_X - MISSILE_SPEED * 50.0);
        assert_eq!(missiles[3].y, START_Y);

        game.process(1.0);

        let missiles = game.get_missiles();
        assert_eq!(missiles.len(), 0);
    }

    #[test]
    fn test_missiles_barriers_collision() {
        const START_X: f64 = 10.0;
        const START_Y: f64 = 10.0;

        const TARGET_X: f64 = 10.0;
        const TARGET_Y: f64 = 20.0;

        let p =
            Player::create_with_direction(START_X, START_Y, 1.0, 1.0, 90.0, 7, 0.0, MISSILE_SPEED);
        let mut map = Map::new(100.0, 100.0, 0, 0.0, SEED);
        map.barriers.push(Barrier {
            x: TARGET_X,
            y: TARGET_Y,
            r: 1.0,
        });
        let game = Game::create(map);
        game.register_player(&p);

        p.fire();

        assert_eq!(game.get_missiles().len(), 1);

        for _ in 0..9 {
            game.process(1.0);
        }

        let missiles = game.get_missiles();
        assert_eq!(missiles.len(), 1);
        assert_eq!(missiles[0].x, START_X);
        assert_eq!(missiles[0].y, START_Y + 9.0);

        game.process(1.0);

        assert_eq!(game.get_missiles().len(), 0);
    }

    #[test]
    fn test_missiles_players_collision() {
        const START_X: f64 = 10.0;
        const START_Y: f64 = 10.0;

        const TARGET_X: f64 = 10.0;
        const TARGET_Y: f64 = 20.0;

        let p1 =
            Player::create_with_direction(START_X, START_Y, 1.0, 1.0, 60.0, 7, 0.0, MISSILE_SPEED);
        let p2 = Player::create_with_direction(
            TARGET_X,
            TARGET_Y,
            1.0,
            1.0,
            60.0,
            7,
            0.0,
            MISSILE_SPEED,
        );

        let game = Game::create(Map::new(100.0, 100.0, 0, 0.0, SEED));
        game.register_player(&p1);
        game.register_player(&p2);

        p1.fire();

        let missiles = game.get_missiles();
        assert_eq!(missiles.len(), 1);

        game.process(10.0);

        let missiles = game.get_missiles();
        assert_eq!(missiles.len(), 0);

        // TODO check hit player
    }
}
