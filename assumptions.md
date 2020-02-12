## Simulation Assumptions

_version 1.0.0_

### Beacon chain state properties

* validators

### Validator properties

* balance
* effective_balance
* is_active
* is_slashed

### Validators start

* All of them active at epoch 0 (no gradually adding)
* All at 32 ETH balance and effective balance

### Fixed probabilities

* Online probability
  * The probability a validator has to be online during the epoch.
  * For simplicity, we model the "inclusion probability" on this very number.
* Honesty probability
  * The probability a validator has to "behave honesty". i.e. cast the right vote.

### Epoch processing

* No real consensus operations (justification and finalization)
  * `State::get_matching_balance`
    * All active validators, less the slashed ones, vote for the right source/target/head.
* By spec functionality
  * process_justification_and_finalization
    * NOT SIMULATED
  * process_rewards_and_penalties
    * head/FFG rewards/penalties
    * proposer and attestator microincentives
    * inactivity penalty
  * process_registry_updates
    * exit due to EJECTION_BALANCE
  * process_slashings
    * NOT IMPLEMENTED
  * NOTE: We are omitting slasher whistleblower/proposal rewards (for now!)
  * process_final_updates
    * update effective balances with hysteresis

### Process rewards and penalties

* Proposer chosing
  * If we have less than 32 active validators, the simulation panics
  * We apply the effective balance bias on proposer choosing as in,
    * https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#compute_proposer_index

### Expected value of the inclusion probability tree

* The "non head/FFG reward" incentive for attesters is
  * `(base_reward - proposer_reward_amount) / inclusion_delay`
  * For example
    * If the proposer reward is `base_reward / 8`, and the attester got its vote included 2 slots after it was cast,
    * then the attester reward is `(base_reward * 7 / 8) / 2`.
  * If the probability inclusion for an attested vote is P,
    * then the expected value, which is the sum of all the payouts in its probability tree is,
    * `E = (base_reward * 7 / 8) * P * SUM(i=1 to infinity)(((1-P^(i-1))/i))`,
    * which can be reduced to the finite term `(base_reward * 7/8)*(P*ln(P))/(P-1)`.
  * As the former probability is given as a starting condition, we can pre-compute this value at startup.

