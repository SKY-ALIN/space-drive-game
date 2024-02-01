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
    player_id: usize,
) -> RayHit {
    let game = game.lock().unwrap();
    let map_x = game.map.width / 2.0;
    let map_y = game.map.height / 2.0;

    let mut next_x = x;
    let mut next_y = y;

    loop {
        // Find the min distance to borders and check the limit

        let border_dx = map_x - (next_x - map_x).abs();
        let border_dy = map_y - (next_y - map_y).abs();
        let mut min_distance = if border_dx < border_dy {
            border_dx
        } else {
            border_dy
        };

        if min_distance <= DISTANCE_LIMIT {
            break RayHit::Border(next_x, next_y);
        }

        // Find the min distance to barriers and check the limit

        for barrier in game.map.barriers.iter() {
            let barrier_distance =
                ((next_x - barrier.x).powi(2) + (next_y - barrier.y).powi(2)).sqrt() - barrier.r;
            if barrier_distance < min_distance {
                min_distance = barrier_distance;
            }
        }

        if min_distance <= DISTANCE_LIMIT {
            break RayHit::Barrier(next_x, next_y);
        }

        // Find the min distance to players and check the limit

        for player in game.players.iter() {
            let player = player.lock().unwrap();
            if player.id == player_id {
                continue;
            }

            let player_distance =
                ((next_x - player.x).powi(2) + (next_y - player.y).powi(2)).sqrt() - player.r;
            if player_distance < min_distance {
                min_distance = player_distance;
            }
        }

        if min_distance <= DISTANCE_LIMIT {
            break RayHit::Player(next_x, next_y);
        }

        // Update for the next iteration

        next_x += (direction * std::f64::consts::PI / 180.0).sin() * min_distance;
        next_y += (direction * std::f64::consts::PI / 180.0).cos() * min_distance;
    }
}
