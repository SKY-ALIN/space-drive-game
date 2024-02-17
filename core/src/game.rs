use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

use crate::player::PlayerStatus;

use super::map::Map;
use super::player::{Missile, Player};

const TIME_STEP: f64 = 0.1;

enum GameStatus {
    On,
    Over(Arc<Mutex<Player>>),
    OverDraw,
}

pub struct Game {
    pub map: Map,
    pub players: Vec<Arc<Mutex<Player>>>,
    pub missiles: Vec<Missile>,
    status: GameStatus,
}

impl Game {
    pub fn create(map: Map) -> Arc<Mutex<Self>> {
        let game = Game {
            map,
            players: Vec::new(),
            missiles: Vec::new(),
            status: GameStatus::On,
        };
        Arc::new(Mutex::new(game))
    }
}

pub trait GameTrait {
    fn register_player(self: &Arc<Self>, player: &Arc<Mutex<Player>>);
    fn process(self: &Arc<Self>, time: f64);
}

impl GameTrait for Mutex<Game> {
    fn register_player(self: &Arc<Self>, player: &Arc<Mutex<Player>>) {
        player.lock().unwrap().mount_game(Arc::clone(self));
        let mut game = self.lock().unwrap();
        game.players.push(Arc::clone(player));
    }

    fn process(self: &Arc<Self>, time: f64) {
        let mut game = self.lock().unwrap();
        let Game {
            ref map,
            ref mut missiles,
            ref mut players,
            ref mut status,
        } = *game;

        let mut time_left = time;
        let mut timedelta: f64;
        loop {
            if time_left > TIME_STEP {
                timedelta = TIME_STEP;
            } else if time_left <= TIME_STEP && time_left > 0.0 {
                timedelta = time_left;
            } else {
                break;
            }
            time_left = ((time_left - TIME_STEP) * 10000.0).round() / 10000.0;

            let mut alived_players_count = 0;
            let mut alived_player: Option<Arc<Mutex<Player>>> = None;
            for player_arc in players.iter() {
                let mut player = player_arc.lock().unwrap();

                if player.status != PlayerStatus::InGame {
                    continue;
                }

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
                alived_players_count += 1;
                alived_player = Some(Arc::clone(player_arc));
            }

            if alived_players_count == 0 {
                *status = GameStatus::OverDraw;
                break;
            } else if alived_players_count == 1 {
                let player = alived_player.unwrap();
                {
                    player.lock().unwrap().status = PlayerStatus::Win;
                }
                *status = GameStatus::Over(player);
                break;
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
                players.iter().map(|p| p.lock().unwrap()).all(|mut p| {
                    let is_collision = m.player_id != p.id
                        && ((m.x - p.x).powi(2) + (m.y - p.y).powi(2)).sqrt() < p.r
                        && p.status == PlayerStatus::InGame;
                    if is_collision {
                        p.status = PlayerStatus::KilledBy(m.player_id);
                    }
                    !is_collision
                })
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

    fn round_position(value: f64) -> f64 {
        (value * 100000.0).round() / 100000.0
    }

    #[test]
    fn test_movement() {
        let p = Player::create_with_direction(1.0, 1.0, 1.0, 1.0, 60.0, 7, 0.0, MISSILE_SPEED);
        let game = Game::create(Map::new(100.0, 100.0, 0, 0.0, SEED));
        game.register_player(&p);
        p.set_speed(0.5);

        game.process(1.0);
        p.rotate(90.0);
        game.process(1.0);

        assert_eq!(round_position(p.get_x()), 1.5);
        assert_eq!(round_position(p.get_y()), 1.5);
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

        assert_eq!(round_position(p.get_x()), 0.5);
        assert_eq!(round_position(p.get_y()), 0.5);
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

        assert_eq!(round_position(p.get_x()), 1.0);
        assert_eq!(round_position(p.get_y()), 1.0);
    }

    #[test]
    fn test_missiles_movement() {
        const START_X: f64 = 1.0;
        const START_Y: f64 = 1.0;

        let p =
            Player::create_with_direction(START_X, START_Y, 1.0, 1.0, 60.0, 7, 0.0, MISSILE_SPEED);
        let game = Game::create(Map::new(100.0, 100.0, 0, 0.0, SEED));
        game.register_player(&p);

        {
            let missiles = &game.lock().unwrap().missiles;
            assert_eq!(missiles.len(), 0);
        }

        // Launch two missiles in different directions
        p.fire();
        p.rotate(90.0);
        p.fire();
        game.process(1.0);

        {
            let missiles = &game.lock().unwrap().missiles;

            // Checking for missiles after launch
            // Checking constant speed and position change
            assert_eq!(missiles[0].speed, MISSILE_SPEED);
            assert_eq!(missiles[0].x, START_X);
            assert_eq!(round_position(missiles[0].y), START_Y + MISSILE_SPEED);

            assert_eq!(missiles[1].speed, MISSILE_SPEED);
            assert_eq!(round_position(missiles[1].x), START_X + MISSILE_SPEED);
            assert_eq!(missiles[1].y, START_Y);
        }

        game.process(4.0);

        let missiles = &game.lock().unwrap().missiles;

        // Checking constant speed and position change
        assert_eq!(missiles[0].speed, MISSILE_SPEED);
        assert_eq!(missiles[0].x, START_X);
        assert_eq!(round_position(missiles[0].y), START_Y + MISSILE_SPEED * 5.0);

        assert_eq!(missiles[1].speed, MISSILE_SPEED);
        assert_eq!(round_position(missiles[1].x), START_X + MISSILE_SPEED * 5.0);
        assert_eq!(missiles[1].y, START_Y);
    }

    #[test]
    fn test_missiles_borders_collision() {
        const START_X: f64 = 50.0;
        const START_Y: f64 = 50.0;
        const MAP_SIZE: f64 = 100.0;

        let p =
            Player::create_with_direction(START_X, START_Y, 1.0, 1.0, 60.0, 7, 0.0, MISSILE_SPEED);
        let game = Game::create(Map::new(MAP_SIZE, MAP_SIZE, 0, 0.0, SEED));
        game.register_player(&p);

        // Launch missiles in different directions to check collision for each border
        p.fire();
        p.rotate(90.0);
        p.fire();
        p.rotate(90.0);
        p.fire();
        p.rotate(90.0);
        p.fire();

        // Move until the last frame before collision
        game.process(49.0);

        {
            let missiles = &game.lock().unwrap().missiles;
            assert_eq!(missiles.len(), 4);

            // Make sure the missiles are still there and their position has changed.
            assert_eq!(missiles[0].x, START_X);
            assert_eq!(
                round_position(missiles[0].y),
                START_Y + MISSILE_SPEED * 49.0
            );

            assert_eq!(
                round_position(missiles[1].x),
                START_X + MISSILE_SPEED * 49.0
            );
            assert_eq!(missiles[1].y, START_Y);

            assert_eq!(missiles[2].x, START_X);
            assert_eq!(
                round_position(missiles[2].y),
                START_Y - MISSILE_SPEED * 49.0
            );

            assert_eq!(
                round_position(missiles[3].x),
                START_X - MISSILE_SPEED * 49.0
            );
            assert_eq!(missiles[3].y, START_Y);
        }

        game.process(2.0);

        let missiles = &game.lock().unwrap().missiles;
        // Check if the missiles were destroyed after the collision
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

        // Launch a missile into a barrier
        p.fire();

        {
            let missiles = &game.lock().unwrap().missiles;
            assert_eq!(missiles.len(), 1);
        }

        // Move until the last frame before collision
        game.process(8.0);

        {
            // Check updated position before collision
            let missiles = &game.lock().unwrap().missiles;
            assert_eq!(missiles[0].x, START_X);
            assert_eq!(round_position(missiles[0].y), START_Y + 8.0);
        }

        game.process(1.0);

        {
            let missiles = &game.lock().unwrap().missiles;
            // Check if the missile was destroyed after the collision
            assert_eq!(missiles.len(), 0);
        }
    }
}
