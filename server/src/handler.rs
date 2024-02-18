use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use space_drive_game_core::{
    Game, GameTrait, Player, PlayerStatus as _PlayerStatus, PlayerTrait, ViewHit as _ViewHit,
    ViewTrait,
};

use crate::Config;

#[derive(Deserialize)]
struct PlayerName {
    name: String,
}

#[derive(Serialize)]
struct ViewHit {
    object: String,
    distance: f64,
}

#[derive(Serialize)]
struct View {
    view: Vec<ViewHit>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case", tag = "action")]
enum Action {
    Move { rotate: f64, speed: f64 },
    Fire,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "result")]
enum PlayerResult {
    Killed { by: String },
    Win,
}

struct Connection(TcpStream);

impl Connection {
    #[allow(dead_code)]
    fn send<T: Serialize>(&mut self, data: T) {
        debug!("{}", serde_json::to_string(&data).unwrap());
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

fn make_rasponse_from_view(view: Vec<_ViewHit>) -> View {
    View {
        view: view
            .into_iter()
            .map(|v| match v {
                _ViewHit::Barrier(d) => ViewHit {
                    object: "BARRIER".to_string(),
                    distance: d,
                },
                _ViewHit::Border(d) => ViewHit {
                    object: "BORDER".to_string(),
                    distance: d,
                },
                _ViewHit::Enemy(d) => ViewHit {
                    object: "ENEMY".to_string(),
                    distance: d,
                },
            })
            .collect(),
    }
}

pub fn handle_stream(
    stream: TcpStream,
    game: Arc<Mutex<Game>>,
    config: Arc<Config>,
    last_processing_time: Arc<Mutex<SystemTime>>,
    player_names: Arc<Mutex<HashMap<usize, String>>>,
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
        config.player_view_angle,
        config.player_rays_amount,
        config.player_missile_speed,
    );
    game.register_player(&player);

    let mut unwraped_player_names = player_names.lock().unwrap();
    unwraped_player_names.insert(player.get_id(), player_name);
    drop(unwraped_player_names);

    conn.send(make_rasponse_from_view(player.view()));

    loop {
        let action = conn.receive::<Action>()?;

        let mut locked_player = player.lock().unwrap();

        match locked_player.status {
            _PlayerStatus::Win => {
                conn.send(PlayerResult::Win);
                break;
            }
            _PlayerStatus::KilledBy(killer_id) => {
                let unwraped_player_names = player_names.lock().unwrap();
                let killer_name = unwraped_player_names.get(&killer_id).unwrap().clone();
                drop(unwraped_player_names);
                conn.send(PlayerResult::Killed { by: killer_name });
                break;
            }
            _ => {}
        }

        match action {
            Action::Fire => {
                info!(target: target, "Fire");
                locked_player.fire()
            }
            Action::Move { rotate, speed } => {
                info!(target: target, "Move rotate={}, speed={}", rotate, speed);
                locked_player.rotate(rotate);
                locked_player.set_speed(speed);
            }
        }
        drop(locked_player);

        conn.send(make_rasponse_from_view(player.view()));

        let now = SystemTime::now();
        let mut locked_last_processing_time = last_processing_time.lock().unwrap();
        let timedelta = now.duration_since(*locked_last_processing_time);
        *locked_last_processing_time = now;
        game.process(timedelta.unwrap().as_secs_f64());
    }

    conn.close();
    info!(target: target, "Finish processing");
    Ok(())
}
