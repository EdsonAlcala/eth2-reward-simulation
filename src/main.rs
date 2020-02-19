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
    let monthly_report_bool = false;
    if monthly_report_bool {
        output.print_monthly_report(&state.config);
    } else {
        output.print_epoch_report("csv");
    }
}
