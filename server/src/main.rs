use log::{error, info};
use std::env;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use space_drive_game_core::{Game, Map};

mod config;
mod handler;

use config::Config;
use handler::handle_stream;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();

    let config = Arc::new(Config::default());
    let listener = TcpListener::bind(config.host).unwrap();
    info!("Server is running on {}", config.host);

    let map = match config.map_seed {
        Some(seed) => Map::new(
            config.map_widht,
            config.map_height,
            config.map_barriers_amount,
            config.map_max_barrier_radius,
            seed,
        ),
        None => Map::new_without_seed(
            config.map_widht,
            config.map_height,
            config.map_barriers_amount,
            config.map_max_barrier_radius,
        ),
    };
    let game = Game::create(map);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New connection: {}", stream.peer_addr().unwrap());
                let cloned_game_ref = Arc::clone(&game);
                let cloned_config_ref = Arc::clone(&config);
                thread::spawn(move || handle_stream(stream, cloned_game_ref, cloned_config_ref));
            }
            Err(e) => {
                error!("Client connection error: {e}");
            }
        }
    }
}
