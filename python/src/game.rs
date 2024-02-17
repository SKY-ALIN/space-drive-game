use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[rustfmt::skip]
use space_drive_game_core::game::{
    Game        as _Game,
    GameTrait   as _GameTrait,
    GameStatus  as _GameStatus,
};

use super::map::Map;
use super::player::Player;

#[pyclass]
pub struct Game(Arc<Mutex<_Game>>);

#[pymethods]
impl Game {
    #[new]
    pub fn new(map: &Map) -> Self {
        Game(_Game::create(map.0.clone()))
    }

    pub fn register_player(&self, player: &Player) {
        self.0.register_player(&player.0);
    }

    fn process(&self, time: f64) {
        self.0.process(time);
    }

    fn get_missiles(&self) -> Vec<(f64, f64)> {
        self.0
            .lock()
            .unwrap()
            .missiles
            .iter()
            .map(|m| (m.x, m.y))
            .collect()
    }

    #[getter]
    pub fn status(&self) -> &str {
        match self.0.lock().unwrap().status {
            _GameStatus::On => "[ON]",
            _GameStatus::Over(_) => "[OVER]",
            _GameStatus::OverDraw => "[OVER]",
        }
    }
}
