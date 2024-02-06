use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

use super::map::Map;
use super::player::{Missile, Player};

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
            .map(|x| Missile {
                x: x.x,
                y: x.y,
                direction: x.direction,
                id: x.id,
                player_id: x.id,
                speed: x.speed,
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
        for player_arc in game.players.iter() {
            let mut player = player_arc.lock().unwrap();

            // Calculate next coordinates
            let mut next_x = player.x + (player.direction * PI / 180.0).sin() * player.speed * time;
            let mut next_y = player.y + (player.direction * PI / 180.0).cos() * player.speed * time;

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
                let distance = calc_distance(next_x, next_y, barrier.x, barrier.y);
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

        let mut updated_missiles: Vec<Missile> = Vec::new();

        for missile in game.missiles.iter() {
            let next_x = missile.x + (missile.direction * PI / 180.0).sin() * missile.speed;
            let next_y = missile.y + (missile.direction * PI / 180.0).cos() * missile.speed;

            let has_border_collision =
                next_x < 0.0 || next_y < 0.0 || next_x > game.map.width || next_y > game.map.height;

            if has_border_collision {
                continue;
            }

            let has_barrier_collision = game
                .map
                .barriers
                .iter()
                .any(|barrier| calc_distance(next_x, next_y, barrier.x, barrier.y) <= 0.0);

            if has_barrier_collision {
                continue;
            }

            let hit_player = game.players.iter().find(|player_arc| {
                let player = player_arc.lock().unwrap();

                let distance = calc_distance(next_x, next_y, player.x, player.y);

                player.id != missile.player_id && distance < player.r
            });

            let has_player_collision = hit_player.is_some();

            if has_player_collision {
                // TODO handle player collision
                continue;
            }

            updated_missiles.push(Missile {
                x: next_x,
                y: next_y,
                direction: missile.direction,
                id: missile.id,
                player_id: missile.id,
                speed: missile.speed,
            });
        }

        game.missiles = updated_missiles;
    }
}

fn calc_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
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

        assert_eq!(p.get_x(), 1.5);
        assert_eq!(p.get_y(), 1.5);
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

        for _ in 0..4 {
            game.process(1.0);
        }

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

        for _ in 0..50 {
            game.process(1.0);
        }

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
        const START_X: f64 = 0.0;
        const START_Y: f64 = 10.0;

        let p =
            Player::create_with_direction(START_X, START_Y, 1.0, 1.0, 60.0, 7, 0.0, MISSILE_SPEED);
        let mut map = Map::new(100.0, 100.0, 0, 0.0, SEED);
        map.barriers.push(Barrier {
            x: 10.0,
            y: 10.0,
            r: 1.0,
        });
        let game = Game::create(map);
        game.register_player(&p);

        p.rotate(90.0);
        p.fire();

        for _ in 0..9 {
            game.process(1.0);
        }

        let missiles = game.get_missiles();
        assert_eq!(missiles.len(), 1);
        assert_eq!(missiles[0].x, 9.0);
        assert_eq!(missiles[0].y, START_Y);

        game.process(1.0);

        let missiles = game.get_missiles();
        assert_eq!(missiles.len(), 0);
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

        for _ in 0..10 {
            game.process(1.0);
        }
        let missiles = game.get_missiles();
        assert_eq!(missiles.len(), 0);

        // TODO check hit player
    }
}
