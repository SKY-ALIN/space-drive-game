use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use space_drive_game_core::{Game, Map};

#[derive(Serialize)]
struct Player {
    id: usize,
    ip: String,
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "object")]
enum Object {
    Missile {
        x: f64,
        y: f64,
        direction: f64,
        id: usize,
        player_id: usize,
    },
    Player {
        x: f64,
        y: f64,
        r: f64,
        direction: f64,
        id: usize,
    },
}

#[derive(Serialize)]
pub struct State {
    time: f64,
    objects: Vec<Object>,
}

#[derive(Serialize)]
struct Barrier {
    x: f64,
    y: f64,
    r: f64,
}

#[derive(Serialize)]
struct MapState {
    width: f64,
    height: f64,
    barriers: Vec<Barrier>,
    seed: u64,
}

impl From<&Map> for MapState {
    fn from(value: &Map) -> Self {
        MapState {
            width: value.width,
            height: value.height,
            barriers: value
                .barriers
                .iter()
                .map(|b| Barrier {
                    x: b.x,
                    y: b.y,
                    r: b.r,
                })
                .collect(),
            seed: value.seed,
        }
    }
}

#[derive(Serialize)]
pub struct History {
    map: MapState,
    history: Vec<State>,
    players: Vec<Player>,
    winner: Option<Player>,
}

impl History {
    pub fn new(map: &Map) -> Self {
        History {
            map: map.into(),
            history: Vec::new(),
            players: Vec::new(),
            winner: None,
        }
    }

    pub fn write_state(&mut self, game: &Game, time: &SystemTime) {
        let time = time.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        let mut objects = Vec::new();

        for missile in game.missiles.iter() {
            objects.push(Object::Missile {
                x: missile.x,
                y: missile.y,
                direction: missile.direction,
                id: missile.id,
                player_id: missile.player_id,
            })
        }

        for player in game.players.iter() {
            let locked_player = player.lock().unwrap();
            objects.push(Object::Player {
                x: locked_player.x,
                y: locked_player.y,
                r: locked_player.r,
                direction: locked_player.direction,
                id: locked_player.id,
            })
        }
        self.history.push(State { time, objects });
    }

    pub fn add_player(&mut self, id: &usize, name: &str, ip: &str) {
        self.players.push(Player {
            id: *id,
            ip: ip.to_string(),
            name: name.to_string(),
        })
    }

    pub fn set_winner(&mut self, id: &usize, name: &str, ip: &str) {
        self.winner = Some(Player {
            id: *id,
            ip: ip.to_string(),
            name: name.to_string(),
        });
    }
}
