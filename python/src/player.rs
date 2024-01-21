use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[rustfmt::skip]
use space_drive_game_core::player::{
    Player      as _Player,
    PlayerError as _PlayerError,
    PlayerTrait as _PlayerTrait,
};

#[pyclass]
pub struct Player(pub Arc<Mutex<_Player>>);

#[pymethods]
impl Player {
    #[new]
    pub fn new(x: f32, y: f32) -> Self {
        Player(_Player::create(x, y))
    }

    pub fn rotate(&mut self, direction: f32) -> PyResult<()> {
        match self.0.rotate(direction) {
            Ok(_) => Ok(()),
            Err(_PlayerError::NonExistentAngle) => Err(PyValueError::new_err("")),
        }
    }

    pub fn forward(&mut self) {
        self.0.forward()
    }

    #[getter]
    pub fn direction(&self) -> f32 {
        self.0.get_direction()
    }

    #[getter]
    pub fn x(&self) -> f32 {
        self.0.get_x()
    }

    #[getter]
    pub fn y(&self) -> f32 {
        self.0.get_y()
    }
}
