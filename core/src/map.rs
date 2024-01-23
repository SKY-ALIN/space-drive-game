use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Barrier {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

#[derive(Clone)]
pub struct Map {
    pub width: f64,
    pub height: f64,
    pub barriers: Vec<Barrier>,
}

impl Map {
    pub fn new(width: f64, height: f64, barriers_amount: u8, max_barrier_radius: f64) -> Self {
        let barriers = (0..barriers_amount)
            .map(|_| Barrier {
                x: rand::thread_rng().gen_range(0.0..width),
                y: rand::thread_rng().gen_range(0.0..height),
                r: rand::thread_rng().gen_range(0.0..max_barrier_radius),
            })
            .collect();
        Map {
            width,
            height,
            barriers,
        }
    }

    pub fn get_free_point(&self, r: f64) -> (f64, f64) {
        'outer: loop {
            let x = rand::thread_rng().gen_range(r..self.width-r);
            let y = rand::thread_rng().gen_range(r..self.height-r);

            for barrier in self.barriers.iter() {
                // If there are no collisions, return `x` and `y`
                let distance = ((x - barrier.x).powi(2) + (y - barrier.y).powi(2)).sqrt();
                if distance >= (r + barrier.r) {
                    break 'outer (x, y);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Map;

    const WIDTH: f64 = 1000.0;
    const HEIGHT: f64 = 1500.0;
    const BARRIERS_AMOUNT: u8 = 5;
    const MAX_BARRIER_RADIUS: f64 = 100.0;

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
