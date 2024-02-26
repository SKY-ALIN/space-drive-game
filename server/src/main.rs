use log::{error, info};
use std::collections::HashMap;
use std::env;
use std::io;
use std::net::{Shutdown, TcpListener};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use space_drive_game_core::{Game, Map};

mod config;
mod handler;
mod history;

use config::Config;
use handler::handle_stream;
use history::History;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EnvError(#[from] envy::Error),
    #[error(transparent)]
    TCPListenerError(#[from] io::Error),
}

fn main() -> Result<(), Error> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    if env::var("RUST_LOG_STYLE").is_err() {
        env::set_var("RUST_LOG_STYLE", "always")
    }
    env_logger::init();

    let config = Arc::new(Config::new()?);

    let listener = TcpListener::bind(config.host)?;
    listener.set_nonblocking(true)?;
    info!("Server is running on {}", config.host);

    let map = match config.map_seed {
        Some(seed) => Map::new(
            config.map_width,
            config.map_height,
            config.map_barriers_amount,
            config.map_max_barrier_radius,
            seed,
        ),
        None => Map::new_without_seed(
            config.map_width,
            config.map_height,
            config.map_barriers_amount,
            config.map_max_barrier_radius,
        ),
    };
    let history = Arc::new(Mutex::new(History::new(&map)));
    let game = Game::create(map);
    let last_processing_time = Arc::new(Mutex::new(SystemTime::now()));
    let player_names: Arc<Mutex<HashMap<usize, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let players_counter = Arc::new(AtomicUsize::new(0));

    let term = Arc::new(AtomicBool::new(false));
    for sig in signal_hook::consts::TERM_SIGNALS {
        signal_hook::flag::register(*sig, Arc::clone(&term))?;
    }
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let players_counter_val = players_counter.load(Ordering::Relaxed);
                if players_counter_val >= config.players_amount {
                    let _ = stream.shutdown(Shutdown::Both);
                    continue;
                }

                let cloned_game_ref = Arc::clone(&game);
                let cloned_config_ref = Arc::clone(&config);
                let cloned_last_processing_time_ref = Arc::clone(&last_processing_time);
                let cloned_player_names_ref = Arc::clone(&player_names);
                let cloned_players_counter_ref = Arc::clone(&players_counter);
                let cloned_history_ref = Arc::clone(&history);
                thread::spawn(move || {
                    let ip = stream.peer_addr().unwrap();
                    info!("Open connection {}", ip);
                    let _ = handle_stream(
                        stream,
                        cloned_game_ref,
                        cloned_config_ref,
                        cloned_last_processing_time_ref,
                        cloned_player_names_ref,
                        cloned_players_counter_ref,
                        cloned_history_ref,
                    );
                    info!("Close connection {}", ip);
                });
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(50));
                if term.load(Ordering::Relaxed) {
                    info!("Server is shutting down");
                    break;
                }
            }
            Err(e) => {
                error!("Client connection error: {e}");
            }
        }
    }

    Ok(())
}
