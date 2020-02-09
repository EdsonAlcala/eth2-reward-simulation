////////////////////////////////////////////////////////////////////////////////
//
// Output stores the outcomes from the simulation of an epoch
//
////////////////////////////////////////////////////////////////////////////////

use super::deltas::Deltas;

pub struct Output {
    pub rows: Vec<OutputRow>,
}

impl Output {
    pub fn new() -> Output {
        let rows = vec![];

        Output { rows: rows }
    }

    pub fn push(&mut self, row: OutputRow) {
        self.rows.push(row);
    }

    pub fn print(self) {
        // TODO
        // ACA VAMOS
    }
}

#[derive(Copy, Clone)]
pub struct OutputRow {
    pub deltas_ffg_source_rewards: u64,
    pub deltas_ffg_source_penalties: u64,
    /*
    pub deltas_ffg_target: u64,
    pub deltas_lmd_head: u64,
    pub deltas_proposer: u64,
    pub deltas_attester: u64,
    pub deltas_inactivity: u64,

    pub number_of_validators: u64,
    pub number_of_active_validators: u64,
    pub number_of_ejected_validators: u64,

    pub total_balance: u64,
    pub total_effective_balance: u64,
    */
    pub time_elapsed: u128,
}

impl OutputRow {
    pub fn new() -> OutputRow {
        OutputRow {
            deltas_ffg_source_rewards: 0,
            deltas_ffg_source_penalties: 0,

            /*
                    deltas_ffg_target: 0,
                    deltas_lmd_head: 0,
                    deltas_proposer: 0,
                    deltas_attester: 0,
                    deltas_inactivity: 0,

                    number_of_validators: 0,
                    number_of_active_validators: 0,
                    number_of_ejected_validators: 0,

                    total_balance: 0,
                    total_effective_balance: 0,
            */
            time_elapsed: 0,
        }
    }

    pub fn update(&mut self, deltas: &Deltas) {
        self.deltas_ffg_source_rewards += deltas.ffg_source_reward;
        self.deltas_ffg_source_penalties += deltas.ffg_source_penalty;
    }
}
