use pyo3::prelude::*;

use space_drive_game_core::game::Game as _Game;

use super::map::Map;

#[pyclass]
pub struct Game(_Game);

#[pymethods]
impl Game {
    #[new]
    pub fn new(map: &Map) -> Self {
        Game(_Game::new(map.0.clone()))
    }
}
