use rand::{rngs::StdRng, Rng, SeedableRng};
pub struct RNG(u64, u64, StdRng); // times seed stdRng

impl RNG {
    pub fn new(seed: u64) -> Self {
        RNG(0, seed, StdRng::seed_from_u64(seed))
    }

    pub fn seed(&self) -> u64 {
        self.1
    }

    pub fn random(&mut self) -> (u64, f64) {
        self.0 += 1;
        (self.0, self.2.gen::<f64>())
    }

    pub fn random_boolean(&mut self) -> (u64, bool) {
        let (times, value) = self.random();
        (times, if value > 0.5 { true } else { false })
    }

    pub fn random_range_i64(&mut self, min: i64, max: i64) -> (u64, i64) {
        let (times, value) = self.random();
        (times, (value * (max - min + 1) as f64) as i64 + min)
    }

    pub fn random_val_boolean(&mut self, val: f64) -> (u64, bool) {
        let (times, value) = self.random();
        (times, if value < val as f64 { true } else { false })
    }

    pub fn random_times(seed: u64, times: u64) -> (u64, f64) {
        if times < 1 {
            panic!("times must >= 1");
        }
        let mut rng = RNG::new(seed);
        let mut _times = times;
        loop {
            if _times <= 1 {
                return rng.random();
            }
            rng.random();
            _times -= 1;
        }
    }

    pub fn random_boolean_times(seed: u64, times: u64) -> (u64, bool) {
        let (times, value) = RNG::random_times(seed, times);
        (times, if value > 0.5 { true } else { false })
    }
}

impl Default for RNG {
    fn default() -> Self {
        RNG::new(rand::random::<u64>())
    }
}

use std::sync::Mutex;

lazy_static! {
    pub static ref RAND: Mutex<RNG> = Mutex::new(RNG::default());
}
