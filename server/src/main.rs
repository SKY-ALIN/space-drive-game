use log::{error, info};
use std::env;
use std::io;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

use space_drive_game_core::{Game, Map};

mod config;
mod handler;

use config::Config;
use handler::handle_stream;

#[derive(Debug)]
pub enum Error {
    EnvError(envy::Error),
    TCPListenerError(io::Error),
}

fn main() -> Result<(), Error> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();

    let config = Arc::new(Config::new()?);
    let listener = match TcpListener::bind(config.host) {
        Ok(l) => l,
        Err(e) => {
            error!("{e}");
            return Err(Error::TCPListenerError(e));
        }
    };
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
    let last_processing_time = Arc::new(Mutex::new(SystemTime::now()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let cloned_game_ref = Arc::clone(&game);
                let cloned_config_ref = Arc::clone(&config);
                let cloned_last_processing_time_ref = Arc::clone(&last_processing_time);
                thread::spawn(move || {
                    let ip = stream.peer_addr().unwrap();
                    info!("Open connection {}", ip);
                    let _ = handle_stream(stream, cloned_game_ref, cloned_config_ref, cloned_last_processing_time_ref);
                    info!("Close connection {}", ip);
                });
            }
            Err(e) => {
                error!("Client connection error: {e}");
            }
        }
    }

    info!("Server is shouting down");
    Ok(())
}
