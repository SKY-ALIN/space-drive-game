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
    pub speed: f64,
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub direction: f64,
    pub speed: f64,
    max_speed: f64,
    view_angle: f64,
    rays_amount: u16,
    game: Weak<Mutex<Game>>,
    pub id: usize,
    missile_speed: f64,
}

impl Player {
    pub fn create(
        x: f64,
        y: f64,
        r: f64,
        max_speed: f64,
        view_angle: f64,
        rays_amount: u16,
        missile_speed: f64,
    ) -> Arc<Mutex<Self>> {
        let player = Player {
            x,
            y,
            r,
            direction: rand::thread_rng().gen_range(-180f64..180f64),
            speed: 0.0,
            max_speed,
            view_angle,
            rays_amount,
            game: Weak::new(),
            id: get_id(),
            missile_speed,
        };
        Arc::new(Mutex::new(player))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_with_direction(
        x: f64,
        y: f64,
        r: f64,
        max_speed: f64,
        view_angle: f64,
        rays_amount: u16,
        direction: f64,
        missile_speed: f64,
    ) -> Arc<Mutex<Self>> {
        let player = Player {
            x,
            y,
            r,
            direction,
            speed: 0.0,
            max_speed,
            view_angle,
            rays_amount,
            game: Weak::new(),
            id: get_id(),
            missile_speed,
        };
        Arc::new(Mutex::new(player))
    }

    pub fn mount_game(&mut self, game: Arc<Mutex<Game>>) {
        self.game = Arc::downgrade(&game);
    }
}

pub trait PlayerTrait {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn get_direction(&self) -> f64;
    fn rotate(&mut self, angle: f64);
    fn get_speed(&self) -> f64;
    fn set_speed(&mut self, speed: f64);
    fn fire(&self);
}

pub trait ViewTrait {
    fn view(&self) -> Vec<ViewHit>;
}

impl PlayerTrait for Player {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn get_direction(&self) -> f64 {
        self.direction
    }

    fn rotate(&mut self, angle: f64) {
        self.direction += angle;
    }

    fn get_speed(&self) -> f64 {
        self.speed
    }

    fn set_speed(&mut self, speed: f64) {
        if speed <= self.max_speed {
            self.speed = speed;
        } else {
            self.speed = self.max_speed;
        }
    }

    fn fire(&self) {
        let weak_game = self.game.upgrade();
        if weak_game.is_none() {
            return;
        }
        let mutex_game = weak_game.unwrap();
        let mut game = mutex_game.lock().unwrap();
        game.missiles.push(Missile {
            x: self.x,
            y: self.y,
            direction: self.direction,
            id: get_id(),
            player_id: self.id,
            speed: self.missile_speed,
        })
    }
}

impl PlayerTrait for Arc<Mutex<Player>> {
    fn get_x(&self) -> f64 {
        self.lock().unwrap().get_x()
    }

    fn get_y(&self) -> f64 {
        self.lock().unwrap().get_y()
    }

    fn get_direction(&self) -> f64 {
        self.lock().unwrap().get_direction()
    }

    fn rotate(&mut self, angle: f64) {
        self.lock().unwrap().rotate(angle)
    }

    fn get_speed(&self) -> f64 {
        self.lock().unwrap().get_speed()
    }

    fn set_speed(&mut self, speed: f64) {
        self.lock().unwrap().set_speed(speed);
    }

    fn fire(&self) {
        self.lock().unwrap().fire();
    }
}

impl ViewTrait for Arc<Mutex<Player>> {
    fn view(&self) -> Vec<ViewHit> {
        let player = self.lock().unwrap();

        let weak_game = player.game.upgrade();
        if weak_game.is_none() {
            return Vec::new();
        }
        let game = weak_game.unwrap();

        // Get player's data

        let player_direction = player.direction;
        let player_radius = player.r;
        let player_x = player.x;
        let player_y = player.y;
        let player_rays_amount = player.rays_amount;
        let player_view_angle = player.view_angle;
        let player_id = player.id;
        drop(player);

        // Send rays and aggregate hits

        let mut res = Vec::new();
        for i in 0..player_rays_amount {
            let angle_offset = if player_rays_amount > 1 {
                player_view_angle / ((player_rays_amount - 1) as f64)
            } else {
                player_view_angle
            };
            let norm_i: f64 = (i as i16 - (player_rays_amount as i16 / 2)) as f64; // Example: if N_RAYS = 7 and i is [0;7), then norm_i will be -[3;3].
            let ray_direction = player_direction + norm_i * angle_offset;
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
}

#[cfg(test)]
mod tests {
    use crate::{
        game::{Game, GameTrait},
        map::{Barrier, Map},
    };

    use super::{Player, PlayerTrait, ViewHit, ViewTrait};
    use std::sync::{Arc, Mutex};

    const X: f64 = 50.0;
    const Y: f64 = 50.0;
    const R: f64 = 1.0;
    const DIRECTION: f64 = 0.0;
    const MAX_SPEED: f64 = 0.0;
    const VIEW_ANGLE: f64 = 60.0;
    const RAYS_AMOUNT: u16 = 7;
    const MISSILE_SPEED: f64 = 1.0;

    fn get_player() -> Arc<Mutex<Player>> {
        Player::create_with_direction(
            X,
            Y,
            R,
            MAX_SPEED,
            VIEW_ANGLE,
            RAYS_AMOUNT,
            DIRECTION,
            MISSILE_SPEED,
        )
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
        let mut p = get_player();
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
        let mut p = Player::create_with_direction(
            50.0,
            50.0,
            10.0,
            MAX_SPEED,
            VIEW_ANGLE,
            1,
            DIRECTION,
            MISSILE_SPEED,
        );
        let p2 = Player::create(100.0, 50.0, 10.0, MAX_SPEED, VIEW_ANGLE, 0, MISSILE_SPEED);
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
