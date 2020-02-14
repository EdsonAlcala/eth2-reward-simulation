# Simulation Assumptions

_version 1.0.0_

## The simulation state

This state can be seen in the same light as the beacon chain state.

```rust
pub struct State {
    // we keep the config at hand
    pub config: Config,

    pub validators: Vec<Validator>,
}

pub struct Validator {
    pub balance: u64,
    pub effective_balance: u64,

    // these flags may represent something slightly different
    //   than the actual specification.
    // is_active implies that the validator was considered "active"
    //   during the previous epoch.
    pub is_active: bool,
    pub is_slashed: bool,
}

```

## Startup

* All of the validators are active at epoch 0. (i.e. No gradually adding).
* All at 32 ETH in both balance and effective balance.

## Fixed probabilities of the system

### Online probability

The probability a validator was online during the previous epoch. We leverage on this number, and assume the _probability of inclusion_ for an attestation to be equal to this probability.

### Honesty probability

The probability a validator has to _behave honesty_. If a validator behaves in the latter way, it will comply with was expected of it by protocol.

## Epoch processing

Each cycle of this simulation corresponds to a representation of the _epoch processing_ that a validator has.

As for the [spec](https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#epoch-processing), the functionalities are:

```python
def process_epoch(state: BeaconState) -> None:
    process_justification_and_finalization(state)
    process_rewards_and_penalties(state)
    process_registry_updates(state)
    process_slashings(state)
    process_final_updates(state)
```

### Justification and Finalization

No operation of justification, not finalization is simulated by the system. There is a reliance on probabilities, which can be set by the users as initial parameters, to produce values such as the validators that matched the FFG target at the previous epoch.

### Rewards and Penalties

```
The core motivation of this simulation.
```

#### Head and FFG

##### Elegibility Check

```python
eligible_validator_indices = [
    ValidatorIndex(index) for index, v in enumerate(state.validators)
    if is_active_validator(v, previous_epoch) or (v.slashed and previous_epoch + 1 < v.withdrawable_epoch)
]
```

A validator is eligible in the simulation if its flag `is_active` is on.

##### Head and FFG

```python
for attestations in (matching_source_attestations, matching_target_attestations, matching_head_attestations):
    unslashed_attesting_indices = get_unslashed_attesting_indices(state, attestations)
    attesting_balance = get_total_balance(state, unslashed_attesting_indices)
    for index in eligible_validator_indices:
        if index in unslashed_attesting_indices:
            rewards[index] += get_base_reward(state, index) * attesting_balance // total_balance
        else:
            penalties[index] += get_base_reward(state, index)
```

The following conditions must be fullfilled:

* The validator needs to be online. (**A probability in the simulation**). If the validator is offline, while it satisfies the eligibility, it fails (three times) the condition of being in the `unslashed_attesting indices` sets, which are derived in turn (again, three times) from the `matching attestations` sets.
* The validator needs to behave honestly. (**A probability in the simulation**). If the validator is honest, it will cast the vote for the right elements (head, source, and target). Now, this simulation is not accounting for errors in the _view_ the validator possess to casts its vote.
* The validator needs to be *not* slashed. (**A property in the simulation**). A slashed validator cannot receive rewards. Moreover, a slashed validators does not withdraw from the beacon chain inmediately. In the period between slashing and withdrawing, the validator will be penalized each epoch.

If the conditions above are not met, the validator is penalized in the amount specified.

##### Proposer incentives

(PLEASE EDIT HERE)
We pick the 32 block proposers at the start of the epoch,
applying the effective balance bias on proposer choosing as in, the [Specs: Compute proposer index](https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#compute_proposer_index)
If we have less than 32 active validators, the simulation panics

##### Attester incentives

(PLEASE EDIT HERE)
![Expected Value of the attester incentive](https://user-images.githubusercontent.com/729830/74490271-e4a59b80-4ebf-11ea-84cb-e89a50ebcd97.png)
![Probability Tree](https://user-images.githubusercontent.com/729830/74490197-b0ca7600-4ebf-11ea-9137-4b5363fed6aa.png)

##### Inactivity Penaty

Lorem Ipsum (TODO).

### Registry Updates

Concerned with the adding and removing of validators. While deposits [are processed](https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#deposits) on the `process_deposit` stage of `process_block`, validators become eligible to activate in this stage. By the other hand, If a validator's balance drops under `EJECTION_BALANCE`, then `initiate_validator_exit()` is triggered.

**NOTE**: Implementation of these conditions is a goal of `v1.0.0` of this simulation. Not implemented yet.

### Slashings

Lorem Ipsum (TODO).

### Final Updates

On this stage we update effective balances with hysteriesis:

```python
# Update effective balances with hysteresis
for index, validator in enumerate(state.validators):
    balance = state.balances[index]
    HALF_INCREMENT = EFFECTIVE_BALANCE_INCREMENT // 2
    if balance < validator.effective_balance or validator.effective_balance + 3 * HALF_INCREMENT < balance:
        validator.effective_balance = min(balance - balance % EFFECTIVE_BALANCE_INCREMENT, MAX_EFFECTIVE_BALANCE)
```
