use std::sync::{Arc, Mutex};

use super::game::Game;

const DISTANCE_LIMIT: f64 = 0.01;

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
    let game = game.lock().unwrap();
    let map_x = game.map.width / 2.0;
    let map_y = game.map.height / 2.0;

    let mut next_x = x;
    let mut next_y = y;
    let mut min_distance = ignore_first_distance;

    loop {
        next_x += (direction * std::f64::consts::PI / 180.0).sin() * min_distance;
        next_y += (direction * std::f64::consts::PI / 180.0).cos() * min_distance;

        let border_dx = map_x - (next_x - map_x).abs();
        let border_dy = map_y - (next_y - map_y).abs();
        min_distance = if border_dx < border_dy {
            border_dx
        } else {
            border_dy
        };

        if min_distance <= DISTANCE_LIMIT {
            break RayHit::Border(next_x, next_y);
        }
    }
}
