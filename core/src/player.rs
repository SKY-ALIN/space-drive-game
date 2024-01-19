use rand::prelude::*;

#[derive(PartialEq)]
pub enum PlayerError {
    NonExistentAngle,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub direction: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            direction: rand::thread_rng().gen_range(0f32..359f32),
        }
    }

    pub fn rotate(&mut self, direction: f32) -> Result<(), PlayerError> {
        if direction < 0.0 || direction >= 360.0 {
            return Err(PlayerError::NonExistentAngle);
        }
        self.direction = direction;
        Ok(())
    }

    pub fn forward(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Player, PlayerError};

    const X: f32 = 100.0;
    const Y: f32 = 200.0;

    fn get_player() -> Player {
        Player::new(X, Y)
    }

    #[test]
    fn test_attrs() {
        let p = get_player();
        assert_eq!(p.x, X);
        assert_eq!(p.y, Y);
        assert!(p.direction < 360.0);
    }

    #[test]
    fn test_rotation() {
        let mut p = get_player();
        assert!(p.rotate(180.0).is_ok());
        assert_eq!(p.direction, 180.0);
        assert!(p.rotate(1000.0).is_err_and(|x| x == PlayerError::NonExistentAngle));
    }
}
