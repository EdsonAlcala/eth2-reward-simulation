////////////////////////////////////////////////////////////////////////////////
//
// Deltas on validator properties computed at each epoch
//
////////////////////////////////////////////////////////////////////////////////

use std::fmt;

pub struct Deltas {
    pub head_ffg_reward: u64,
    pub head_ffg_penalty: u64,
    pub proposer_reward: u64,
    pub attester_reward: u64,
}

impl Deltas {
    pub fn new() -> Deltas {
        Deltas {
            head_ffg_reward: 0,
            head_ffg_penalty: 0,
            proposer_reward: 0,
            attester_reward: 0,
        }
    }
}

impl fmt::Display for Deltas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{};{};{};{};",
            self.head_ffg_reward, self.head_ffg_penalty, self.proposer_reward, self.attester_reward,
        )
    }
}
