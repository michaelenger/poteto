use core::ops::RangeBounds;
use fastrand;

pub struct Rng {
    rng: fastrand::Rng,
    seed_counter: u64,
    previous_result: u64,
}

impl Rng {
    pub fn new() -> Rng {
        Rng {
            rng: fastrand::Rng::with_seed(24601),
            seed_counter: 0,
            previous_result: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.seed_counter < u64::MAX {
            self.seed_counter += 1
        } else {
            self.seed_counter = 0
        }
    }

    pub fn u8(&mut self, range: impl RangeBounds<u8>) -> u8 {
        self.rng.seed(self.seed_counter + self.previous_result);

        let result = self.rng.u8(range);

        self.previous_result = result as u64;

        result
    }
}
