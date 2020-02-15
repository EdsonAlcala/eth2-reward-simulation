////////////////////////////////////////////////////////////////////////////////
//
// A simplified Eth2 validator
//
////////////////////////////////////////////////////////////////////////////////

use super::config;
use std::cmp;

pub struct Validator {
    pub balance: u64,
    pub effective_balance: u64,

    // these flags may represent something slightly different
    //   than the actual specification.
    // is_active implies that the validator was considered "active"
    //   during the previous epoch.
    // is_slashed self-explains.
    pub is_active: bool,
    pub is_slashed: bool,
}

impl Validator {
    pub fn get_base_reward(&self, sqrt_total_active_balance: u64) -> u64 {
        self.effective_balance * config::BASE_REWARD_FACTOR
            / sqrt_total_active_balance
            / config::BASE_REWARDS_PER_EPOCH
    }

    pub fn update_effective_balance(&mut self) {
        let half_increment = config::EFFECTIVE_BALANCE_INCREMENT / 2;

        if self.balance < self.effective_balance
            || self.effective_balance + 3 * half_increment < self.balance
        {
            self.effective_balance = cmp::min(
                self.balance - self.balance % config::EFFECTIVE_BALANCE_INCREMENT,
                config::MAX_EFFECTIVE_BALANCE,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_base_reward() {
        // ?
    }

    struct Case {
        validator: Validator,
        expected_result: u64,
    }

    fn eth_to_gwei(eth_number: f64) -> u64 {
        (eth_number * 1_000_000_000 as f64) as u64
    }

    fn prepare_case(balance: f64, effective_balance: f64, expected_result: f64) -> Case {
        Case {
            validator: Validator {
                balance: eth_to_gwei(balance),
                effective_balance: eth_to_gwei(effective_balance),
                is_active: true,
                is_slashed: false,
            },
            expected_result: eth_to_gwei(expected_result),
        }
    }

    #[test]
    fn test_update_effective_balance() {
        let mut cases = vec![];

        // balance below (or equal to) 24. effective balance 24
        cases.push(prepare_case(23.0, 24.0, 23.0));
        cases.push(prepare_case(23.5, 24.0, 23.0));
        cases.push(prepare_case(24.0, 24.0, 24.0));

        // balance above 24. effective balance 24
        cases.push(prepare_case(24.5, 24.0, 24.0));
        cases.push(prepare_case(25.0, 24.0, 24.0));
        cases.push(prepare_case(25.5, 24.0, 24.0));
        cases.push(prepare_case(25.500001, 24.0, 25.0));
        cases.push(prepare_case(26.0, 24.0, 26.0));

        // balance below (or equal to) 32. effective balance 32
        cases.push(prepare_case(31.0, 32.0, 31.0));
        cases.push(prepare_case(31.5, 32.0, 31.0));
        cases.push(prepare_case(32.0, 32.0, 32.0));

        // balance above 32. effective balance 32
        cases.push(prepare_case(32.5, 32.0, 32.0));
        cases.push(prepare_case(33.0, 32.0, 32.0));
        cases.push(prepare_case(33.5, 32.0, 32.0));
        cases.push(prepare_case(34.0, 32.0, 32.0));

        // effective balance 31. balance above 31
        cases.push(prepare_case(31.5, 31.0, 31.0));
        cases.push(prepare_case(32.0, 31.0, 31.0));
        cases.push(prepare_case(32.5, 31.0, 31.0));
        cases.push(prepare_case(32.500001, 31.0, 32.0));

        for mut case in cases {
            case.validator.update_effective_balance();
            assert_eq!(case.expected_result, case.validator.effective_balance);
        }
    }
}
