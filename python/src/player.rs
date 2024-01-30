use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[rustfmt::skip]
use space_drive_game_core::player::{
    Player      as _Player,
    PlayerTrait as _PlayerTrait,
    ViewHit     as _ViewHit,
};

#[pyclass]
pub struct Player(pub Arc<Mutex<_Player>>);

#[pymethods]
impl Player {
    #[new]
    #[pyo3(signature = (x, y, r, max_speed = 1.0, view_angel = 60.0, rays_amount = 7, direction = None))]
    pub fn new(
        x: f64,
        y: f64,
        r: f64,
        max_speed: f64,
        view_angel: f64,
        rays_amount: u16,
        direction: Option<f64>,
    ) -> Self {
        match direction {
            Some(d) => Player(_Player::create_with_direction(
                x,
                y,
                r,
                max_speed,
                d,
                view_angel,
                rays_amount,
            )),
            None => Player(_Player::create(x, y, r, max_speed, view_angel, rays_amount)),
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

    pub fn view(&self) -> Vec<(&str, f64)> {
        self.0
            .view()
            .into_iter()
            .map(|view_hit| match view_hit {
                _ViewHit::Border(distance) => ("[BORDER]", distance),
                _ViewHit::Barrier(distance) => ("[BARRIER]", distance),
                _ViewHit::Enemy(distance) => ("[ENEMY]", distance),
            })
            .collect()
    }
}
