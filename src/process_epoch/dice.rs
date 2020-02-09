////////////////////////////////////////////////////////////////////////////////
//
// - Dice helps with randomness
//
////////////////////////////////////////////////////////////////////////////////

use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Dice {
    rng: ThreadRng,
}

impl Dice {
    pub fn new() -> Dice {
        Dice {
            rng: rand::thread_rng(),
        }
    }

    pub fn throw_dice(&mut self, probability: f32) -> bool {
        probability > self.rng.gen()
    }
}
