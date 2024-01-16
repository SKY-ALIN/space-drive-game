use pyo3::prelude::*;
use rand::prelude::*;

struct Barrier {
    x: u16,
    y: u16,
    r: u16,
}

#[pyclass]
pub struct Map {
    #[pyo3(get)]
    width: u16,
    #[pyo3(get)]
    height: u16,
    barriers: Vec<Barrier>,
}

#[pymethods]
impl Map {
    #[new]
    pub fn new(
        width: u16,
        height: u16,
        barriers_amount: u8,
        max_barrier_radius: u16,
    ) -> PyResult<Self> {
        let barriers = (0..barriers_amount)
            .map(|_| Barrier {
                x: rand::thread_rng().gen_range(0..width),
                y: rand::thread_rng().gen_range(0..height),
                r: rand::thread_rng().gen_range(0..max_barrier_radius),
            })
            .collect();
        Ok(Map {
            width,
            height,
            barriers,
        })
    }

    pub fn get_barriers(&self) -> Vec<(u16, u16, u16)> {
        self.barriers.iter().map(|b| (b.x, b.y, b.r)).collect()
    }

    pub fn get_free_point(&self) -> (u16, u16) {
        todo!()
    }
}
