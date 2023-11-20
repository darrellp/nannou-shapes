use rand::{Rng, SeedableRng, distributions::uniform::SampleUniform};
use rand_chacha::ChaCha8Rng;
use std::{cmp::PartialOrd, sync::Mutex};
use lazy_static::lazy_static; // 1.4.0


lazy_static! {
    static ref RNG: Mutex<MyRandom> = Mutex::new(MyRandom::new());
}

pub struct MyRandom
{
    rng: ChaCha8Rng,
}

impl MyRandom {
    fn new() -> Self {
        Self {
            rng: ChaCha8Rng::from_entropy(),
        }
    }

    pub fn seed_from_u64(seed: u64) {
        RNG.lock().unwrap().rng = ChaCha8Rng::seed_from_u64(seed);
    }

    //pub fn range_gen<T: PartialOrd + SampleUniform>(&self, low: T, high: T) -> T
    fn from_range_local<T: SampleUniform + PartialOrd>(&mut self, low: T, high: T) -> T
    {
        self.rng.gen_range(low..high)
    }

    pub fn from_range<T: SampleUniform + PartialOrd>(low: T, high: T) -> T
    {
        RNG.lock().unwrap().from_range_local(low, high)
    }

    fn get_float_local(&mut self) -> f32 {
        const RESOLUTION: i32 = 100_000;
        const RESOLUTION_F: f32 = RESOLUTION as f32;

        self.from_range_local(0, RESOLUTION) as f32 / RESOLUTION_F
    }

    pub fn get_float() -> f32
    {
        RNG.lock().unwrap().get_float_local()
    }
}

