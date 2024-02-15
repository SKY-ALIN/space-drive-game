use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};

use space_drive_game_core::{Game, GameTrait, Player};

use crate::Config;

#[derive(Deserialize)]
struct PlayerName {
    name: String,
}

struct Connection(TcpStream);

impl Connection {
    #[allow(dead_code)]
    fn send<T: Serialize>(&mut self, data: T) {
        debug!("{:?}", serde_json::to_string(&data).unwrap());
        let _ = self
            .0
            .write_all(serde_json::to_string(&data).unwrap().as_bytes());
    }

    fn receive<'a, T: Deserialize<'a>>(&mut self) -> Result<T, serde_json::Error> {
        let mut de = serde_json::Deserializer::from_reader(&self.0);
        T::deserialize(&mut de)
    }

    fn close(&self) {
        self.0.shutdown(Shutdown::Both).unwrap();
    }
}

pub fn handle_stream(
    stream: TcpStream,
    game: Arc<Mutex<Game>>,
    config: Arc<Config>,
) -> Result<(), serde_json::Error> {
    let ip = stream.peer_addr().unwrap();
    let mut conn = Connection(stream);
    let player_name = conn.receive::<PlayerName>()?.name;
    let target = format!("{} ({})", ip, player_name);
    let target = target.as_str();

    info!(target: target, "Start processing");

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

    conn.close();
    info!(target: target, "Finish processing");
    Ok(())
}
