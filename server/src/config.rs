use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const DEFAULT_MAP_WIDTH: f64 = 1500.0;
const DEFAULT_MAP_HEIGHT: f64 = 1000.0;
const DEFAULT_MAP_BARRIERS_AMOUNT: u8 = 50;
const DEFAULT_MAP_MAX_BARRIER_RADIUS: f64 = 50.0;
const DEFAULT_MAP_SEED: Option<u64> = None;
const DEFAULT_PLAYER_RADIUS: f64 = 5.0;
const DEFAULT_PLAYER_MAX_SPEED: f64 = 2.0;
const DEFAULT_PLAYER_VIEW_ANGLE: f64 = 30.0;
const DEFAULT_PLAYER_RAYS_AMOUNT: u16 = 13;
const DEFAULT_PLAYER_MISSILE_SPEED: f64 = 5.0;
const DEFAULT_PLAYERS_AMOUNT: usize = 2;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: SocketAddr,
    #[serde(default = "default_map_width")]
    pub map_width: f64,
    #[serde(default = "default_map_height")]
    pub map_height: f64,
    #[serde(default = "default_map_barriers_amount")]
    pub map_barriers_amount: u8,
    #[serde(default = "default_map_max_barrier_radius")]
    pub map_max_barrier_radius: f64,
    #[serde(default = "default_map_seed")]
    pub map_seed: Option<u64>,
    #[serde(default = "default_player_radius")]
    pub player_radius: f64,
    #[serde(default = "default_player_max_speed")]
    pub player_max_speed: f64,
    #[serde(default = "default_player_view_angle")]
    pub player_view_angle: f64,
    #[serde(default = "default_player_rays_amount")]
    pub player_rays_amount: u16,
    #[serde(default = "default_player_missile_speed")]
    pub player_missile_speed: f64,
    #[serde(default = "default_players_amount")]
    pub players_amount: usize,
}

fn default_host() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3333)
}

fn default_map_width() -> f64 {
    DEFAULT_MAP_WIDTH
}

fn default_map_height() -> f64 {
    DEFAULT_MAP_HEIGHT
}

fn default_map_barriers_amount() -> u8 {
    DEFAULT_MAP_BARRIERS_AMOUNT
}

fn default_map_max_barrier_radius() -> f64 {
    DEFAULT_MAP_MAX_BARRIER_RADIUS
}

fn default_map_seed() -> Option<u64> {
    DEFAULT_MAP_SEED
}

fn default_player_radius() -> f64 {
    DEFAULT_PLAYER_RADIUS
}

fn default_player_max_speed() -> f64 {
    DEFAULT_PLAYER_MAX_SPEED
}

fn default_player_view_angle() -> f64 {
    DEFAULT_PLAYER_VIEW_ANGLE
}

fn default_player_rays_amount() -> u16 {
    DEFAULT_PLAYER_RAYS_AMOUNT
}

fn default_player_missile_speed() -> f64 {
    DEFAULT_PLAYER_MISSILE_SPEED
}

fn default_players_amount() -> usize {
    DEFAULT_PLAYERS_AMOUNT
}

impl Config {
    pub fn new() -> Result<Config, envy::Error> {
        envy::from_env::<Config>()
    }
}
