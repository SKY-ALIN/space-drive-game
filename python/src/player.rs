use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[rustfmt::skip]
use space_drive_game_core::player::{
    Player      as _Player,
    PlayerError as _PlayerError,
};

#[pyclass]
pub struct Player(_Player);

#[pymethods]
impl Player {
    #[new]
    pub fn new(x: f32, y: f32) -> PyResult<Self> {
        Ok(Player(_Player::new(x, y)))
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
        self.0.direction
    }

    #[getter]
    pub fn x(&self) -> f32 {
        self.0.x
    }

    #[getter]
    pub fn y(&self) -> f32 {
        self.0.y
    }
}
