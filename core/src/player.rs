use rand::prelude::*;

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
