use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[derive(Copy, Clone, Debug)]
pub struct Barrier {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

#[derive(Clone, Debug)]
pub struct Map {
    pub width: f64,
    pub height: f64,
    pub barriers: Vec<Barrier>,
    pub seed: u64,
}

impl Map {
    pub fn new(
        width: f64,
        height: f64,
        barriers_amount: u8,
        max_barrier_radius: f64,
        seed: u64,
    ) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let perlin = Perlin::new(seed as u32);
        let noise_scale = 0.1; // Noise scale to adjust the "smoothness" of the noise

        let barriers = (0..barriers_amount).map(|_| {
                let x: f64 = rng.gen_range(0.0..width);
                let y: f64 = rng.gen_range(0.0..height);
                let noise_value = perlin.get([x * noise_scale, y * noise_scale]);
                let r = (noise_value / 2.0 + 0.5) * max_barrier_radius; // Noise normalization from -1..1 to 0..max_barrier_radius
                Barrier { x, y, r }
            }).collect();

        Map {
            width,
            height,
            barriers,
            seed,
        }
    }

    pub fn new_without_seed(
        width: f64,
        height: f64,
        barriers_amount: u8,
        max_barrier_radius: f64,
    ) -> Self {
        let seed: u64 = rand::random::<u64>();

        Self::new(width, height, barriers_amount, max_barrier_radius, seed)
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
    const SEED: u64 = 12345; // Fixed seed for reproducibility

    fn make_map() -> Map {
        Map::new(WIDTH, HEIGHT, BARRIERS_AMOUNT, MAX_BARRIER_RADIUS, SEED)
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
        assert!(m.barriers.len() as u8 >= BARRIERS_AMOUNT); // We check that the number of obstacles is not less than the specified one
        for b in m.barriers {
            assert!(b.x <= WIDTH);
            assert!(b.y <= HEIGHT);
            assert!(b.r <= MAX_BARRIER_RADIUS);
        }
    }

    #[test]
    fn test_generation_with_seed() {
        // Creating two cards with the same seed
        let map1 = Map::new_without_seed(WIDTH, HEIGHT, BARRIERS_AMOUNT, MAX_BARRIER_RADIUS);
        let map2 = Map::new(
            WIDTH,
            HEIGHT,
            BARRIERS_AMOUNT,
            MAX_BARRIER_RADIUS,
            map1.seed,
        );

        // We check that the number of obstacles is the same
        assert_eq!(map1.barriers.len(), map2.barriers.len());

        // We check that each obstacle is identical in position and size
        for (barrier1, barrier2) in map1.barriers.iter().zip(map2.barriers.iter()) {
            assert_eq!(barrier1.x, barrier2.x);
            assert_eq!(barrier1.y, barrier2.y);
            assert_eq!(barrier1.r, barrier2.r);
        }
    }
}
