use std::sync::{Arc, Mutex, Weak};

use rand::prelude::*;

use super::game::Game;
use super::ray_marching::{ray_marching, RayHit};

#[derive(Debug)]
pub enum ViewHit {
    Barrier(f64),
    Border(f64),
    Enemy(f64),
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub direction: f64,
    pub speed: f64,
    max_speed: f64,
    game: Weak<Mutex<Game>>,
}

impl Player {
    pub fn create(x: f64, y: f64, r: f64, max_speed: f64) -> Arc<Mutex<Self>> {
        let player = Player {
            x,
            y,
            r,
            direction: rand::thread_rng().gen_range(-180f64..180f64),
            speed: 0.0,
            max_speed,
            game: Weak::new(),
        };
        Arc::new(Mutex::new(player))
    }

    pub fn create_with_direction(
        x: f64,
        y: f64,
        r: f64,
        max_speed: f64,
        direction: f64,
    ) -> Arc<Mutex<Self>> {
        let player = Player {
            x,
            y,
            r,
            direction,
            speed: 0.0,
            max_speed,
            game: Weak::new(),
        };
        Arc::new(Mutex::new(player))
    }

    pub fn mount_game(&mut self, game: Arc<Mutex<Game>>) {
        self.game = Arc::downgrade(&game);
    }
}

pub trait PlayerTrait {
    fn get_x(self: &Arc<Self>) -> f64;
    fn get_y(self: &Arc<Self>) -> f64;
    fn get_direction(self: &Arc<Self>) -> f64;
    fn rotate(self: &Arc<Self>, direction: f64);
    fn get_speed(self: &Arc<Self>) -> f64;
    fn set_speed(self: &Arc<Self>, speed: f64);
    fn view(self: &Arc<Self>) -> Vec<ViewHit>;
}

impl PlayerTrait for Mutex<Player> {
    fn get_x(self: &Arc<Self>) -> f64 {
        self.lock().unwrap().x
    }

    fn get_y(self: &Arc<Self>) -> f64 {
        self.lock().unwrap().y
    }

    fn get_direction(self: &Arc<Self>) -> f64 {
        self.lock().unwrap().direction
    }

    fn rotate(self: &Arc<Self>, direction: f64) {
        self.lock().unwrap().direction += direction;
    }

    fn get_speed(self: &Arc<Self>) -> f64 {
        self.lock().unwrap().speed
    }

    fn set_speed(self: &Arc<Self>, speed: f64) {
        let mut player = self.lock().unwrap();
        if speed <= player.max_speed {
            player.speed = speed;
        } else {
            player.speed = player.max_speed;
        }
    }

    fn view(self: &Arc<Self>) -> Vec<ViewHit> {
        const N_RAYS: u16 = 7;
        const VIEW_ANGEL: f64 = 60.0;

        // Get player's data

        let player = self.lock().unwrap();
        let weak_game = player.game.upgrade();
        if weak_game.is_none() {
            return Vec::new();
        }
        let game = weak_game.unwrap();
        let player_direction = player.direction;
        let player_radius = player.r;
        let player_x = player.x;
        let player_y = player.y;
        drop(player);

        // Send rays and aggregate hits

        let mut res = Vec::new();
        for i in 0..N_RAYS {
            let angel_offset = VIEW_ANGEL / ((N_RAYS - 1) as f64);
            let norm_i: f64 = (i as i16 - (N_RAYS as i16 / 2)) as f64; // Example: if N_RAYS = 7 and i is [0;7), then norm_i will be -[3;3].
            let ray_direction = player_direction + norm_i * angel_offset;
            let ray_hit = ray_marching(&game, player_x, player_y, ray_direction, player_radius);

            match ray_hit {
                RayHit::Barrier(x, y) => res.push(ViewHit::Barrier(
                    ((player_x - x).powi(2) + (player_y - y).powi(2)).sqrt(),
                )),
                RayHit::Border(x, y) => res.push(ViewHit::Border(
                    ((player_x - x).powi(2) + (player_y - y).powi(2)).sqrt(),
                )),
                RayHit::Player(x, y) => res.push(ViewHit::Enemy(
                    ((player_x - x).powi(2) + (player_y - y).powi(2)).sqrt(),
                )),
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        game::{Game, GameTrait},
        map::{Barrier, Map},
    };

    use super::{Player, PlayerTrait};
    use std::sync::{Arc, Mutex};

    const X: f64 = 100.0;
    const Y: f64 = 200.0;
    const R: f64 = 1.0;
    const DIRECTION: f64 = 0.0;
    const MAX_SPEED: f64 = 0.0;

    fn get_player() -> Arc<Mutex<Player>> {
        Player::create_with_direction(X, Y, R, MAX_SPEED, DIRECTION)
    }

    #[test]
    fn test_attrs() {
        let p = get_player();
        assert_eq!(p.get_x(), X);
        assert_eq!(p.get_y(), Y);
        assert_eq!(p.get_direction(), DIRECTION);
        assert_eq!(p.get_speed(), 0.0);
    }

    #[test]
    fn test_rotation() {
        let p = get_player();
        p.rotate(180.0);
        assert_eq!(p.get_direction(), 180.0);
        p.rotate(-360.0);
        assert_eq!(p.get_direction(), -180.0);
    }

    #[test]
    fn test_view() {
        let mut map = Map::new(150.0, 250.0, 0, 0.0);
        map.barriers.push(Barrier {
            x: 100.0,
            y: 250.0,
            r: 25.0,
        });
        let game = Game::create(map);
        let p = get_player();
        game.register_player(&p);
        println!("{:?}", p.view());
    }
}
