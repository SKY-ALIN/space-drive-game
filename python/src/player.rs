use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[rustfmt::skip]
use space_drive_game_core::player::{
    Player      as _Player,
    PlayerTrait as _PlayerTrait,
};

#[pyclass]
pub struct Player(pub Arc<Mutex<_Player>>);

#[pymethods]
impl Player {
    #[new]
    pub fn new(x: f64, y: f64, max_speed: f64, direction: Option<f64>) -> Self {
        match direction {
            Some(d) => Player(_Player::create_with_direction(x, y, max_speed, d)),
            None => Player(_Player::create(x, y, max_speed)),
        }
    }

    pub fn rotate(&mut self, direction: f64) {
        self.0.rotate(direction);
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.0.set_speed(speed);
    }

    #[getter]
    pub fn direction(&self) -> f64 {
        self.0.get_direction()
    }

    #[getter]
    pub fn speed(&self) -> f64 {
        self.0.get_speed()
    }

    #[getter]
    pub fn x(&self) -> f64 {
        self.0.get_x()
    }

    #[getter]
    pub fn y(&self) -> f64 {
        self.0.get_y()
    }
}
