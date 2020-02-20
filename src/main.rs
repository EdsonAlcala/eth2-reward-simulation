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

    if state.config.report_type == "monthly" {
        output.print_monthly_report(&state.config);
    } else if state.config.report_type == "epoch" {
        output.print_epoch_report(&state.config);
    }

    // Output::write_to_json(&monthly_report);

}
