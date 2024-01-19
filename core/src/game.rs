use super::map::Map;

pub struct Game {
    pub map: Map,
}

impl Game {
    pub fn new(map: Map) -> Self {
        Game { map }
    }
}
