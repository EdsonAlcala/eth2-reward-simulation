////////////////////////////////////////////////////////////////////////////////
//
// Output stores the outcomes from the simulation of an epoch
//
////////////////////////////////////////////////////////////////////////////////

use super::deltas::Deltas;
use std::env;

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

    pub fn print(&self, mode: &str) {
        if mode == "csv" {
            println!(
                "{};{};{};{};{};",
                "epoch number".to_string(),
                "head/ffg rewards".to_string(),
                "head/ffg penalties".to_string(),
                "proposer rewards".to_string(),
                "attester rewards".to_string(),
            );

            for row in &self.rows {
                println!(
                    "{};{};{};{};{};",
                    row.epoch_number,
                    row.deltas_head_ffg_rewards,
                    row.deltas_head_ffg_penalties,
                    row.deltas_proposer_rewards,
                    row.deltas_attester_rewards,
                );
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct OutputRow {
    pub epoch_number: i32,
    pub deltas_head_ffg_rewards: u64,
    pub deltas_head_ffg_penalties: u64,
    pub deltas_proposer_rewards: u64,
    pub deltas_attester_rewards: u64,

    /*
    pub deltas_inactivity: u64,

    pub number_of_validators: u64,
    pub number_of_active_validators: u64,

    pub total_balance: u64,
    pub total_effective_balance: u64,
    */
    pub time_elapsed: u128,
}

impl OutputRow {
    pub fn new() -> OutputRow {
        OutputRow {
            epoch_number: 0,
            deltas_head_ffg_rewards: 0,
            deltas_head_ffg_penalties: 0,
            deltas_proposer_rewards: 0,
            deltas_attester_rewards: 0,

            /*
                    deltas_inactivity: 0,

                    number_of_validators: 0,
                    number_of_active_validators: 0,

                    total_balance: 0,
                    total_effective_balance: 0,
            */
            time_elapsed: 0,
        }
    }

    pub fn update(&mut self, deltas: &Deltas) {
        let debug_output_row = !env::var("DEBUG_OUTPUT_ROW").is_err();

        if debug_output_row {
            println!("{}", deltas);
        }

        self.deltas_head_ffg_rewards += deltas.head_ffg_reward;
        self.deltas_head_ffg_penalties += deltas.head_ffg_penalty;
        self.deltas_proposer_rewards += deltas.proposer_reward;
        self.deltas_attester_rewards += deltas.attester_reward;
    }
}
