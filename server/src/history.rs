use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use space_drive_game_core::{Game, Map};

#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "object")]
enum Object {
    Missile {
        x: f64,
        y: f64,
        direction: f64,
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

pub trait GetState {
    fn get_state(&self, time: &SystemTime) -> State;
}

impl GetState for Game {
    fn get_state(&self, time: &SystemTime) -> State {
        let time = time.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        let mut objects = Vec::new();

        for missile in self.missiles.iter() {
            objects.push(Object::Missile {
                x: missile.x,
                y: missile.y,
                direction: missile.direction,
            })
        }

        for player in self.players.iter() {
            let locked_player = player.lock().unwrap();
            objects.push(Object::Player {
                x: locked_player.x,
                y: locked_player.y,
                r: locked_player.r,
                direction: locked_player.direction,
                id: locked_player.id,
            })
        }

        State { time, objects }
    }
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
}

impl History {
    pub fn new(map: &Map) -> Self {
        History {
            map: map.into(),
            history: Vec::new(),
        }
    }

    pub fn add_state(&mut self, state: State) {
        self.history.push(state);
    }
}
