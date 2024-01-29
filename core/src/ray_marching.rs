use std::sync::{Arc, Mutex};

use super::game::Game;

pub enum RayHit {
    Border(f64, f64),
    Barrier(f64, f64),
    Player(f64, f64),
}

pub fn ray_marching(
    game: &Arc<Mutex<Game>>,
    x: f64,
    y: f64,
    direction: f64,
    ignore_first_distance: f64,
) -> RayHit {
    todo!()
}
