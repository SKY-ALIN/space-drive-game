use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use space_drive_game_core::{
    Game, GameTrait, Player, PlayerStatus, PlayerTrait, RegisterPlayer, ViewHit, ViewTrait,
};

use crate::Config;

#[derive(Serialize, Deserialize)]
struct PlayerName {
    name: String,
}

#[derive(Serialize)]
struct ViewHitSchema {
    object: String,
    distance: f64,
}

#[derive(Serialize)]
struct ViewSchema {
    view: Vec<ViewHitSchema>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action")]
enum Action {
    Move { rotate: f64, speed: f64 },
    Fire,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "result")]
enum PlayerStatusSchema {
    Killed { by: String },
    Win,
}

struct Connection(TcpStream);

impl Connection {
    #[allow(dead_code)]
    fn send<T: Serialize>(&mut self, data: T) {
        let str_data = serde_json::to_string(&data).unwrap();
        debug!("{}", str_data);
        let _ = self.0.write_all(str_data.as_bytes());
    }

    fn receive<'a, T: Deserialize<'a> + Serialize>(&mut self) -> Result<T, serde_json::Error> {
        let mut de = serde_json::Deserializer::from_reader(&self.0);
        let res = T::deserialize(&mut de);
        match &res {
            Ok(data) => debug!("{}", serde_json::to_string(data).unwrap()),
            Err(e) => {
                let ip = self.0.peer_addr().unwrap();
                if e.is_eof() {
                    warn!("{} suddenly closed its connection", ip);
                } else {
                    error!("Invalid data from {}, err: {}", ip, e);
                }
            }
        }
        res
    }

    fn close(&self) {
        self.0.shutdown(Shutdown::Both).unwrap();
    }
}

fn make_rasponse_from_view(view: Vec<ViewHit>) -> ViewSchema {
    ViewSchema {
        view: view
            .into_iter()
            .map(|v| match v {
                ViewHit::Barrier(d) => ViewHitSchema {
                    object: "BARRIER".to_string(),
                    distance: d,
                },
                ViewHit::Border(d) => ViewHitSchema {
                    object: "BORDER".to_string(),
                    distance: d,
                },
                ViewHit::Enemy(d) => ViewHitSchema {
                    object: "ENEMY".to_string(),
                    distance: d,
                },
            })
            .collect(),
    }
}

pub fn handle_stream(
    stream: TcpStream,
    mut game: Arc<Mutex<Game>>,
    config: Arc<Config>,
    last_processing_time: Arc<Mutex<SystemTime>>,
    player_names: Arc<Mutex<HashMap<usize, String>>>,
    players_counter: Arc<AtomicUsize>,
) -> Result<(), serde_json::Error> {
    let ip = stream.peer_addr().unwrap();
    let mut conn = Connection(stream);
    let player_name = conn.receive::<PlayerName>()?.name;
    let target = format!("{} ({})", ip, player_name);
    let target = target.as_str();
    info!(target: target, "Player registered");

    while players_counter.load(Ordering::SeqCst) != config.players_amount {}

    info!(target: target, "Start processing");
    let locked_game = game.lock().unwrap();
    let coordinates = locked_game.map.get_free_point(config.player_radius);
    drop(locked_game);

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

    let mut locked_player_names = player_names.lock().unwrap();
    locked_player_names.insert(player.get_id(), player_name);
    drop(locked_player_names);

    conn.send(make_rasponse_from_view(player.view()));

    loop {
        let action = conn.receive::<Action>()?;

        let mut locked_player = player.lock().unwrap();

        match locked_player.status {
            PlayerStatus::Win => {
                conn.send(PlayerStatusSchema::Win);
                break;
            }
            PlayerStatus::KilledBy(killer_id) => {
                let locked_player_names = player_names.lock().unwrap();
                let killer_name = locked_player_names.get(&killer_id).unwrap().clone();
                drop(locked_player_names);
                conn.send(PlayerStatusSchema::Killed { by: killer_name });
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
