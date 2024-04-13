use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[rustfmt::skip]
use space_drive_game_core::player::{
    Player          as _Player,
    PlayerTrait     as _PlayerTrait,
    PlayerStatus    as _PlayerStatus,
    ViewHit         as _ViewHit,
    ViewTrait       as _ViewTrait,
};

#[pyclass]
pub struct Player(pub Arc<Mutex<_Player>>);

#[pymethods]
impl Player {
    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = (x, y, r, max_speed = 1.0, view_angle = 60.0, rays_amount = 7, missile_speed = 1.0, direction = None))]
    pub fn new(
        x: f64,
        y: f64,
        r: f64,
        max_speed: f64,
        view_angle: f64,
        rays_amount: u16,
        missile_speed: f64,
        direction: Option<f64>,
    ) -> Self {
        match direction {
            Some(d) => Player(_Player::new_with_direction(
                x,
                y,
                r,
                max_speed,
                view_angle,
                rays_amount,
                d,
                missile_speed,
            )),
            None => Player(_Player::new(
                x,
                y,
                r,
                max_speed,
                view_angle,
                rays_amount,
                missile_speed,
            )),
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        self.0.rotate(angle);
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

    #[getter]
    pub fn status(&self) -> &str {
        match self.0.lock().unwrap().status {
            _PlayerStatus::Win => "[WIN]",
            _PlayerStatus::InGame => "[INGAME]",
            _PlayerStatus::KilledBy(_) => "[DEAD]",
        }
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

    pub fn fire(&self) {
        self.0.fire()
    }
}
