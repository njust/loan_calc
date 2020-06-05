use iced::{Element, Text};
use crate::loan_view::LoanView;
use rust_decimal::Decimal;

#[derive(Default)]
pub struct Overview {
}

#[derive(Debug, Clone)]
pub enum OverviewMessage {
}

impl Overview {
    pub fn view(&mut self, loans: &Vec<Box<LoanView>>) -> Element<OverviewMessage> {
        let mut monthly_rate = Decimal::new(0, 2);
        let mut remaining = Decimal::new(0, 2);
        let mut paid_interest = Decimal::new(0, 2);
        let mut cleared_amount = Decimal::new(0, 2);
        for loan in loans {
            if let Some(res) = &loan.result {
                monthly_rate += res.monthly_rate;
                remaining += res.overall.remaining;
                paid_interest += res.overall.paid_interest;
                cleared_amount += res.overall.cleared_amount;
            }
        }
        Text::new(format!("Monthly rate: {}, Remaining: {}, Paid interest: {}, Cleared: {}",
                          monthly_rate.round_dp(2),
                          remaining.round_dp(2),
                          paid_interest.round_dp(2),
                          cleared_amount.round_dp(2),
        )).into()
    }
}