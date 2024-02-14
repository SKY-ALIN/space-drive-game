use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use space_drive_game_core::{Game, GameTrait, Map, Player};

struct Config {
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

impl Default for Config {
    fn default() -> Self {
        Self {
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
    println!("Start processing {}", stream.peer_addr().unwrap());

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
    println!("Finish processing {}", stream.peer_addr().unwrap());
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");

    let config = Arc::new(Config::default());
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
                println!("New connection: {}", stream.peer_addr().unwrap());
                let cloned_game_ref = Arc::clone(&game);
                let cloned_config_ref = Arc::clone(&config);
                thread::spawn(move || handle_stream(stream, cloned_game_ref, cloned_config_ref));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
