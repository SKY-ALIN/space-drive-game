use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Barrier {
    pub x: u16,
    pub y: u16,
    pub r: u16,
}

#[derive(Clone)]
pub struct Map {
    pub width: u16,
    pub height: u16,
    pub barriers: Vec<Barrier>,
}

impl Map {
    pub fn new(width: u16, height: u16, barriers_amount: u8, max_barrier_radius: u16) -> Self {
        let barriers = (0..barriers_amount)
            .map(|_| Barrier {
                x: rand::thread_rng().gen_range(0..width),
                y: rand::thread_rng().gen_range(0..height),
                r: rand::thread_rng().gen_range(0..max_barrier_radius),
            })
            .collect();
        Map {
            width,
            height,
            barriers,
        }
    }

    pub fn get_free_point(&self) -> (u16, u16) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Map;

    const WIDTH: u16 = 1000;
    const HEIGHT: u16 = 1500;
    const BARRIERS_AMOUNT: u8 = 5;
    const MAX_BARRIER_RADIUS: u16 = 100;
    
    fn make_map() -> Map {
        Map::new(WIDTH, HEIGHT, BARRIERS_AMOUNT, MAX_BARRIER_RADIUS)
    }

    #[test]
    fn test_attrs() {
        let m = make_map();
        assert_eq!(m.width, WIDTH);
        assert_eq!(m.height, HEIGHT);
    }

    #[test]
    fn test_barriers() {
        let m = make_map();
        assert_eq!(m.barriers.len(), BARRIERS_AMOUNT as usize);
        for b in m.barriers {
            assert!(b.x <= WIDTH);
            assert!(b.y <= HEIGHT);
            assert!(b.r <= MAX_BARRIER_RADIUS);
        }
    }
}
