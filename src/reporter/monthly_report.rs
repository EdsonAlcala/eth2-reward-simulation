struct MonthlyReport {
    rows: Vec<MonthlyReportRow>
}

impl MonthlyReport {
    load_data()
    print()
    
}

#[derive(Copy, Clone, Serialize)]
pub struct MonthlyReportRow {
    pub month_number: u32,
    pub network_percentage_rewards: f64,
    pub network_percentage_penalties: f64,
    pub network_percentage_net_rewards: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_base_reward() {


    }
}