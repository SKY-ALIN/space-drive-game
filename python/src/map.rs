use pyo3::prelude::*;

use space_drive_game_core::map::Map as _Map;

#[pyclass]
pub struct Map(pub _Map);

#[pymethods]
impl Map {
    #[new]
    pub fn new(
        width: u16,
        height: u16,
        barriers_amount: u8,
        max_barrier_radius: u16,
    ) -> PyResult<Self> {
        Ok(Map(_Map::new(
            width,
            height,
            barriers_amount,
            max_barrier_radius,
        )))
    }

    pub fn get_barriers(&self) -> Vec<(u16, u16, u16)> {
        self.0.barriers.iter().map(|b| (b.x, b.y, b.r)).collect()
    }

    pub fn get_free_point(&self) -> (u16, u16) {
        self.0.get_free_point()
    }

    #[getter]
    pub fn width(&self) -> u16 {
        self.0.width
    }

    #[getter]
    pub fn height(&self) -> u16 {
        self.0.height
    }
}
