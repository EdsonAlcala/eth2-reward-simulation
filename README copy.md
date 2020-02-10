


=================================

### TODO!

* REFACTOR

^^^^ 

-------
* You need to have deltas instead of just adding, to show. Maybe?
* add histeriesis on effective balances
  * https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#final-updates
* Do the csv for just the 3 ffg rewards -> have some output already!
* Have a single output mode (i.e. what is the issuance of the network, ROI for a single validators on an initial stake)
* add the proposer inclusion reward
* add the attester inclusion reward
* do the inactivity penalty
* do the slasher reward/punishment
  * https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#slash_validator
  * https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#slashings
* have a mechanism to remove slashed validators over time
* move this todo in this very file
* add running instructions for the sim
* Edit this README -? This is a busy people / repository
* Do the blogpost, delete the busy people in the `kb` repo
  * https://docs.google.com/document/d/10_z2YudaBBWfqgIFAip44TA6PJ049R0NPAyAsGBsMH8/edit

=================================



#### Contents

* Running this simulation
* [Rewards and penalties on Eth2 for busy people](#rewards-and-penalties-on-eth2)
* Notes

#### Rewards and penalties on Eth2

* Rewards and Penalties are computed at the first [slot](#slot) of each [epoch](#epoch) (except at genesis!).

##### FFG rewards

* ???

##### Proposer and attester rewards

* ???

##### Penalties

* ???

--------------------------------------------------------------------------------

## Running a simulation

### Overview

* ???

### The gauges

* ???

### Links for the simulation

* TODO: Add gist code here

### An annotated spreadsheet with results

* ???


## Notes

* _Caveat Lector_. While this reference is being written on 07.FEB.2020, it risks being obsolete in the future!
* Thanks to the Pukara Team for reviewing: Edson Alcala, Sylvain Laurent, Tim Lowe.
* Thanks for additional reviewing to @benjaminion.

### Extra notes

#### Chances to be a proposer

* We check in a candidate validator for the ratio `effective_balance / MAX_EFFECTIVE_BALANCE` to be greater than a random number between 0 and 1. If the condition is not met, we recompute the candidate.
