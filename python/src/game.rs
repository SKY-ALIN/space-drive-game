use pyo3::prelude::*;

use super::map::Map;

#[pyclass]
pub struct Game {
    map: &Map,
}

#[pymethods]
impl Game {
    #[new]
    pub fn new(map: &Map) -> Self {
        Game {
            map
        }
    }
}
