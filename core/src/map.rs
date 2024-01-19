use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Barrier {
    pub x: u16,
    pub y: u16,
    pub r: u16,
}

#[derive(Clone)]
pub struct Map {
    pub width: u16,
    pub height: u16,
    pub barriers: Vec<Barrier>,
}

impl Map {
    pub fn new(width: u16, height: u16, barriers_amount: u8, max_barrier_radius: u16) -> Self {
        let barriers = (0..barriers_amount)
            .map(|_| Barrier {
                x: rand::thread_rng().gen_range(0..width),
                y: rand::thread_rng().gen_range(0..height),
                r: rand::thread_rng().gen_range(0..max_barrier_radius),
            })
            .collect();
        Map {
            width,
            height,
            barriers,
        }
    }

    pub fn get_free_point(&self) -> (u16, u16) {
        todo!()
    }
}
