use std::sync::{Arc, Mutex, Weak};

use rand::prelude::*;

use super::game::Game;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub direction: f64,
    pub speed: f64,
    max_speed: f64,
    game: Option<Weak<Mutex<Game>>>,
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
            game: None,
        };
        Arc::new(Mutex::new(player))
    }

    pub fn create_with_direction(x: f64, y: f64, r: f64, max_speed: f64, direction: f64) -> Arc<Mutex<Self>> {
        let player = Player {
            x,
            y,
            r,
            direction,
            speed: 0.0,
            max_speed,
            game: None,
        };
        Arc::new(Mutex::new(player))
    }

    pub fn mount_game(&mut self, game: Arc<Mutex<Game>>) {
        self.game = Some(Arc::downgrade(&game));
    }
}

pub trait PlayerTrait {
    fn get_x(self: &Arc<Self>) -> f64;
    fn get_y(self: &Arc<Self>) -> f64;
    fn get_direction(self: &Arc<Self>) -> f64;
    fn rotate(self: &Arc<Self>, direction: f64);
    fn get_speed(self: &Arc<Self>) -> f64;
    fn set_speed(self: &Arc<Self>, speed: f64);
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
}

#[cfg(test)]
mod tests {
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
}
