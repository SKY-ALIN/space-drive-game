use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use rand::prelude::*;

#[pyclass]
pub struct Player {
    #[pyo3(get)]
    x: f32,
    #[pyo3(get)]
    y: f32,
    #[pyo3(get)]
    direction: f32,
}

#[pymethods]
impl Player {
    #[new]
    pub fn new(x: f32, y: f32) -> PyResult<Self> {
        Ok(Player {
            x,
            y,
            direction: rand::thread_rng().gen_range(0f32..359f32),
        })
    }

    pub fn rotate(&mut self, direction: f32) -> PyResult<()> {
        if direction < 0.0 || direction >= 360.0 {
            return Err(PyValueError::new_err(""))
        }
        self.direction = direction;
        Ok(())
    }

    pub fn forward(&mut self) {
        todo!()
    }
}
