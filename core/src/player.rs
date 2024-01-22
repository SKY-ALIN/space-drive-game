use std::sync::{Arc, Mutex, Weak};

use rand::prelude::*;

use super::game::Game;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub direction: f64,
    game: Option<Weak<Mutex<Game>>>,
}

impl Player {
    pub fn create(x: f64, y: f64) -> Arc<Mutex<Self>> {
        let player = Player {
            x,
            y,
            direction: rand::thread_rng().gen_range(-180f64..180f64),
            game: None,
        };
        Arc::new(Mutex::new(player))
    }

    pub fn create_with_direction(x: f64, y: f64, direction: f64) -> Arc<Mutex<Self>> {
        let player = Player {
            x,
            y,
            direction,
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
    fn forward(self: &Arc<Self>);
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

    fn forward(self: &Arc<Self>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Player, PlayerTrait};
    use std::sync::{Arc, Mutex};

    const X: f64 = 100.0;
    const Y: f64 = 200.0;
    const DIRECTION: f64 = 0.0;

    fn get_player() -> Arc<Mutex<Player>> {
        Player::create_with_direction(X, Y, DIRECTION)
    }

    #[test]
    fn test_attrs() {
        let p = get_player();
        assert_eq!(p.get_x(), X);
        assert_eq!(p.get_y(), Y);
        assert_eq!(p.get_direction(), DIRECTION);
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
