////////////////////////////////////////////////////////////////////////////////
//
// - config: UI (or command line) params
// - state: simple beacon chain state (with a reference to config)
// - validator: simple validator object
//
////////////////////////////////////////////////////////////////////////////////

use std::fmt;

pub struct Deltas {
    pub head_ffg_reward: u64,
    pub head_ffg_penalty: u64,
    pub proposer_reward: u64,
    /*
    attester_reward: u64,
    inactivity_penalty: u64,
    */
}

impl Deltas {
    pub fn new() -> Deltas {
        Deltas {
            head_ffg_reward: 0,
            head_ffg_penalty: 0,
            proposer_reward: 0,
            /*
            attester_reward: 0,
            inactivity_penalty: 0,
            */
        }
    }
}

impl fmt::Display for Deltas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{};{};{}",
            self.head_ffg_reward, self.head_ffg_penalty, self.proposer_reward
        )
    }
}
