use log::{error, info};
use std::env;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use space_drive_game_core::{Game, GameTrait, Map, Player};

struct Config<'a> {
    host: &'a str,
    map_widht: f64,
    map_height: f64,
    map_barriers_amount: u8,
    map_max_barrier_radius: f64,
    map_seed: Option<u64>,
    player_radius: f64,
    player_max_speed: f64,
    player_view_angel: f64,
    player_rays_amount: u16,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            host: "127.0.0.1:3333",
            map_widht: 1500.0,
            map_height: 1000.0,
            map_barriers_amount: 50,
            map_max_barrier_radius: 50.0,
            map_seed: None,
            player_radius: 5.0,
            player_max_speed: 2.0,
            player_view_angel: 30.0,
            player_rays_amount: 13,
        }
    }
}

fn handle_stream(stream: TcpStream, game: Arc<Mutex<Game>>, config: Arc<Config>) {
    let target = "player_name";
    info!(target: target, "Start processing {}", stream.peer_addr().unwrap());

    let unwraped_game = game.lock().unwrap();
    let coordinates = unwraped_game.map.get_free_point(config.player_radius);
    drop(unwraped_game);

    let player = Player::create(
        coordinates.0,
        coordinates.1,
        config.player_radius,
        config.player_max_speed,
        config.player_view_angel,
        config.player_rays_amount,
    );
    game.register_player(&player);

    stream.shutdown(Shutdown::Both).unwrap();
    info!(target: target, "Finish processing {}", stream.peer_addr().unwrap());
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
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
