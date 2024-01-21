use std::sync::{Arc, Mutex, Weak};

use rand::prelude::*;

use super::game::Game;

#[derive(PartialEq)]
pub enum PlayerError {
    NonExistentAngle,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub direction: f32,
    game: Option<Weak<Mutex<Game>>>,
}

impl Player {
    pub fn create(x: f32, y: f32) -> Arc<Mutex<Self>> {
        let player = Player {
            x,
            y,
            direction: rand::thread_rng().gen_range(0f32..359f32),
            game: None,
        };
        Arc::new(Mutex::new(player))
    }

    pub fn mount_game(&mut self, game: Arc<Mutex<Game>>) {
        self.game = Some(Arc::downgrade(&game));
    }
}

pub trait PlayerTrait {
    fn get_x(self: &Arc<Self>) -> f32;
    fn get_y(self: &Arc<Self>) -> f32;
    fn get_direction(self: &Arc<Self>) -> f32;
    fn rotate(self: &Arc<Self>, direction: f32) -> Result<(), PlayerError>;
    fn forward(self: &Arc<Self>);
}

impl PlayerTrait for Mutex<Player> {
    fn get_x(self: &Arc<Self>) -> f32 {
        self.lock().unwrap().x
    }

    fn get_y(self: &Arc<Self>) -> f32 {
        self.lock().unwrap().y
    }

    fn get_direction(self: &Arc<Self>) -> f32 {
        self.lock().unwrap().direction
    }

    fn rotate(self: &Arc<Self>, direction: f32) -> Result<(), PlayerError> {
        if direction < 0.0 || direction >= 360.0 {
            return Err(PlayerError::NonExistentAngle);
        }
        self.lock().unwrap().direction = direction;
        Ok(())
    }

    fn forward(self: &Arc<Self>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Player, PlayerError, PlayerTrait};
    use std::sync::{Arc, Mutex};

    const X: f32 = 100.0;
    const Y: f32 = 200.0;

    fn get_player() -> Arc<Mutex<Player>> {
        Player::create(X, Y)
    }

    #[test]
    fn test_attrs() {
        let p = get_player();
        assert_eq!(p.get_x(), X);
        assert_eq!(p.get_y(), Y);
        assert!(p.get_direction() < 360.0);
    }

    #[test]
    fn test_rotation() {
        let p = get_player();
        assert!(p.rotate(180.0).is_ok());
        assert_eq!(p.get_direction(), 180.0);
        assert!(p
            .rotate(1000.0)
            .is_err_and(|x| x == PlayerError::NonExistentAngle));
    }
}
