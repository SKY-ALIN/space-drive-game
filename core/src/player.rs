use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};

use rand::prelude::*;

use super::game::Game;
use super::ray_marching::{ray_marching, RayHit};

fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, PartialEq)]
pub enum ViewHit {
    Barrier(f64),
    Border(f64),
    Enemy(f64),
}

pub struct Missile {
    pub x: f64,
    pub y: f64,
    pub direction: f64,
    pub id: usize,
    pub player_id: usize,
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub direction: f64,
    pub speed: f64,
    max_speed: f64,
    view_angel: f64,
    rays_amount: u16,
    game: Weak<Mutex<Game>>,
    pub id: usize,
}

impl Player {
    pub fn create(
        x: f64,
        y: f64,
        r: f64,
        max_speed: f64,
        view_angel: f64,
        rays_amount: u16,
    ) -> Arc<Mutex<Self>> {
        let player = Player {
            x,
            y,
            r,
            direction: rand::thread_rng().gen_range(-180f64..180f64),
            speed: 0.0,
            max_speed,
            view_angel,
            rays_amount,
            game: Weak::new(),
            id: get_id(),
        };
        Arc::new(Mutex::new(player))
    }

    pub fn create_with_direction(
        x: f64,
        y: f64,
        r: f64,
        max_speed: f64,
        view_angel: f64,
        rays_amount: u16,
        direction: f64,
    ) -> Arc<Mutex<Self>> {
        let player = Player {
            x,
            y,
            r,
            direction,
            speed: 0.0,
            max_speed,
            view_angel,
            rays_amount,
            game: Weak::new(),
            id: get_id(),
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
    fn fire(self: &Arc<Self>);
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
        let player_rays_amount = player.rays_amount;
        let player_view_angel = player.view_angel;
        let player_id = player.id;
        drop(player);

        // Send rays and aggregate hits

        let mut res = Vec::new();
        for i in 0..player_rays_amount {
            let angel_offset = if player_rays_amount > 1 {
                player_view_angel / ((player_rays_amount - 1) as f64)
            } else {
                player_view_angel
            };
            let norm_i: f64 = (i as i16 - (player_rays_amount as i16 / 2)) as f64; // Example: if N_RAYS = 7 and i is [0;7), then norm_i will be -[3;3].
            let ray_direction = player_direction + norm_i * angel_offset;
            let ray_hit = ray_marching(&game, player_x, player_y, ray_direction, player_id);

            match ray_hit {
                RayHit::Barrier(x, y) => res.push(ViewHit::Barrier(
                    ((player_x - x).powi(2) + (player_y - y).powi(2)).sqrt() - player_radius,
                )),
                RayHit::Border(x, y) => res.push(ViewHit::Border(
                    ((player_x - x).powi(2) + (player_y - y).powi(2)).sqrt() - player_radius,
                )),
                RayHit::Player(x, y) => res.push(ViewHit::Enemy(
                    ((player_x - x).powi(2) + (player_y - y).powi(2)).sqrt() - player_radius,
                )),
            }
        }
        res
    }

    fn fire(self: &Arc<Self>) {
        let player = self.lock().unwrap();
        let weak_game = player.game.upgrade();
        if weak_game.is_none() {
            return;
        }
        let mutex_game = weak_game.unwrap();
        let mut game = mutex_game.lock().unwrap();
        game.missiles.push(Missile {
            x: player.x,
            y: player.y,
            direction: player.direction,
            id: get_id(),
            player_id: player.id,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        game::{Game, GameTrait},
        map::{Barrier, Map},
    };

    use super::{Player, PlayerTrait, ViewHit};
    use std::sync::{Arc, Mutex};

    const X: f64 = 50.0;
    const Y: f64 = 50.0;
    const R: f64 = 1.0;
    const DIRECTION: f64 = 0.0;
    const MAX_SPEED: f64 = 0.0;
    const VIEW_ANGEL: f64 = 60.0;
    const RAYS_AMOUNT: u16 = 7;

    fn get_player() -> Arc<Mutex<Player>> {
        Player::create_with_direction(X, Y, R, MAX_SPEED, VIEW_ANGEL, RAYS_AMOUNT, DIRECTION)
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
        let mut map = Map::new_without_seed(100.0, 100.0, 0, 0.0);
        map.barriers.push(Barrier {
            x: 50.0,
            y: 100.0,
            r: 10.0,
        });
        let game = Game::create(map);
        let p =
            Player::create_with_direction(50.0, 50.0, 10.0, MAX_SPEED, VIEW_ANGEL, 1, DIRECTION);
        let p2 = Player::create(100.0, 50.0, 10.0, MAX_SPEED, VIEW_ANGEL, 0);
        game.register_player(&p);
        game.register_player(&p2);

        assert_eq!(p.view().first().unwrap(), &ViewHit::Barrier(30.0));
        p.rotate(90.0);
        assert_eq!(p.view().first().unwrap(), &ViewHit::Enemy(30.0));
        p.rotate(90.0);
        assert_eq!(p.view().first().unwrap(), &ViewHit::Border(40.0));
    }

    #[test]
    fn test_fire() {
        let map = Map::new_without_seed(100.0, 100.0, 0, 0.0);
        let mutex_game = Game::create(map);
        let mutex_player = get_player();
        mutex_game.register_player(&mutex_player);

        mutex_player.fire();

        let game = mutex_game.lock().unwrap();
        let missile = game.missiles.first().unwrap();
        let player = mutex_player.lock().unwrap();

        assert_eq!(missile.player_id, player.id);
        assert_eq!(missile.direction, player.direction);
        assert_eq!(missile.x, player.x);
        assert_eq!(missile.y, player.y);
    }
}
