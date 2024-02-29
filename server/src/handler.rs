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

use crate::config::Config;
use crate::history::History;

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

#[allow(clippy::too_many_arguments)]
pub fn handle_stream(
    stream: TcpStream,
    game: Arc<Mutex<Game>>,
    config: Arc<Config>,
    last_processing_time: Arc<Mutex<SystemTime>>,
    player_names: Arc<Mutex<HashMap<usize, (String, String)>>>,
    players_counter: Arc<AtomicUsize>,
    history: Arc<Mutex<History>>,
    winner_id: Arc<Mutex<Option<usize>>>,
) -> Result<(), serde_json::Error> {
    let ip = stream.peer_addr().unwrap();
    let mut conn = Connection(stream);
    let player_name = conn.receive::<PlayerName>()?.name;
    let target = format!("{} ({})", ip, player_name);
    let target = target.as_str();

    info!(target: target, "Wait for other players");
    let players_counter_val = players_counter.load(Ordering::Relaxed);
    players_counter.store(players_counter_val + 1, Ordering::SeqCst);
    while players_counter.load(Ordering::SeqCst) != config.players_amount {}

    let coordinates = game
        .lock()
        .unwrap()
        .map
        .get_free_point(config.player_radius);
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
    player_names
        .lock()
        .unwrap()
        .insert(player.get_id(), (player_name, ip.to_string()));
    info!(target: target, "Game started");

    conn.send(make_rasponse_from_view(player.view()));

    loop {
        let action = conn.receive::<Action>()?;

        let mut locked_player = player.lock().unwrap();

        match locked_player.status {
            PlayerStatus::Win => {
                info!(target: target, "Win");
                let mut locked_winner_id = winner_id.lock().unwrap();
                *locked_winner_id = Some(locked_player.id);
                conn.send(PlayerStatusSchema::Win);
                break;
            }
            PlayerStatus::KilledBy(killer_id) => {
                let locked_player_names = player_names.lock().unwrap();
                let (killer_name, killer_ip) = locked_player_names.get(&killer_id).unwrap().clone();
                drop(locked_player_names);
                info!(target: target, "Killed by {} ({})", killer_name, killer_ip);
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
        {
            let mut locked_game = game.lock().unwrap();
            locked_game.process(timedelta.unwrap().as_secs_f64());
            history.lock().unwrap().write_state(&locked_game, &now);
        }
    }

    info!(target: target, "Game over");
    conn.close();
    Ok(())
}
