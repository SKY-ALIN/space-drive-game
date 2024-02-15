pub struct Config<'a> {
    pub host: &'a str,
    pub map_widht: f64,
    pub map_height: f64,
    pub map_barriers_amount: u8,
    pub map_max_barrier_radius: f64,
    pub map_seed: Option<u64>,
    pub player_radius: f64,
    pub player_max_speed: f64,
    pub player_view_angel: f64,
    pub player_rays_amount: u16,
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
