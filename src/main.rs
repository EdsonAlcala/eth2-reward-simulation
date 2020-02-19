////////////////////////////////////////////////////////////////////////////////
//
// simple simulator of rewards and penalties for Phase 0
//
////////////////////////////////////////////////////////////////////////////////

mod process_epoch;
mod types;

use process_epoch::process_epoch;
use types::*;

fn main() {
    let mut state = State::new();
    let mut output = Output::new();

    for i in 0..state.config.epochs {
        state = process_epoch(state, i, &mut output);
    }

    // TODO: Use a command line option here
    if true {
        let only_per_month = output.get_rows_by_month(&state.config);

        for record in only_per_month {
            println!(
                "Month number: {}, Total Network Rewards {}",
                record.month_number, record.network_percentage_net_rewards
            );
        }
    } else {
        output.print("csv");
    }
}
