use crate::{LoanType};
use rust_decimal_macros::*;
use std::str::FromStr;

use iced::{Button, button, TextInput, Text, Element, Row, Column, text_input, Scrollable, Length};

use std::{
    error::Error
};
use rust_decimal::Decimal;
use crate::style::ButtonStyle;

#[derive(Default)]
pub struct LoanView {
    state: LoanViewState,
    name: String,
    amount: String,
    interest_rate: String,
    clearance_rate: String,
    runtime_years: String,
    loan_type: LoanType,
    pub result: Option<CalcResultOverview>
}

#[derive(Default)]
struct LoanViewState {
    name: text_input::State,
    amount: text_input::State,
    interest_rate: text_input::State,
    clearance_rate: text_input::State,
    calc_button: button::State,
    result_scroller: iced::scrollable::State,
    runtime_years: text_input::State,
    annuity_btn: button::State,
    building_savings_btn: button::State,
}

#[derive(Debug, Clone)]
pub enum LoanViewMessage {
    NameChanged(String),
    AmountChanged(String),
    InterestRateChanged(String),
    ClearanceRateChanged(String),
    RuntimeChanged(String),
    ChangeTypeToAnnuity,
    ChangeTypeToBuildingSavings,
    Calc,
}


#[derive(Default, Debug)]
pub struct CalcResultOverview {
    pub overall: CalcResult,
    pub monthly_rate: Decimal,
    months: Vec<Box<CalcResult>>
}

#[derive(Default, Debug)]
pub struct CalcResult {
    month: i32,
    pub remaining: Decimal,
    pub paid_interest: Decimal,
    pub cleared_amount: Decimal,
}

impl CalcResult {
    fn view(&mut self) -> Element<LoanViewMessage> {
        Text::new(format!(
            "{} - {} - {} - {}", self.month, self.remaining, self.cleared_amount, self.paid_interest
        )).into()
    }
}

impl LoanView {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }
    pub fn update(&mut self, message: LoanViewMessage) {
        match message {
            LoanViewMessage::AmountChanged(amount) => {
                self.amount = amount;
            }
            LoanViewMessage::InterestRateChanged(rate) => {
                self.interest_rate = rate;
            }
            LoanViewMessage::ClearanceRateChanged(clearance) => {
                self.clearance_rate = clearance;
            }
            LoanViewMessage::Calc => {
                self.result.take();
                let result = match self.loan_type {
                    LoanType::Annuity => self.calc_annuity(),
                    LoanType::BuildingSavings => self.calc_building_saving(),
                };
                if let Ok(result) = result {
                    self.result = Some(result);
                }
            }
            LoanViewMessage::RuntimeChanged(rt) => {
                self.runtime_years = rt;
            }
            LoanViewMessage::ChangeTypeToAnnuity => {
                self.loan_type = LoanType::Annuity;
            }
            LoanViewMessage::ChangeTypeToBuildingSavings => {
                self.loan_type = LoanType::BuildingSavings;
            }
            LoanViewMessage::NameChanged(name) => {
                self.name = name;
            }
        }
    }

    pub fn view(&mut self) -> Element<LoanViewMessage> {
        let mut col = Column::new()
            .padding(20)
            .spacing(5)
            .push(TextInput::new(&mut self.state.name, "Name", &self.name, LoanViewMessage::NameChanged))
            .push(TextInput::new(&mut self.state.amount, "Amount", &self.amount, LoanViewMessage::AmountChanged))
            .push(TextInput::new(&mut self.state.interest_rate, "Interest rate", &self.interest_rate, LoanViewMessage::InterestRateChanged))
            .push(TextInput::new(&mut self.state.clearance_rate, "Clearance rate", &self.clearance_rate, LoanViewMessage::ClearanceRateChanged))
            .push(TextInput::new(&mut self.state.runtime_years, "Runtime years", &self.runtime_years, LoanViewMessage::RuntimeChanged))
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.state.annuity_btn, Text::new("Annuity"))
                            .on_press(LoanViewMessage::ChangeTypeToAnnuity)
                            .style(ButtonStyle { active: matches!(self.loan_type, LoanType::Annuity)})
                    )
                    .push(
                        Button::new(&mut self.state.building_savings_btn, Text::new("Building savings"))
                            .on_press(LoanViewMessage::ChangeTypeToBuildingSavings)
                            .style(ButtonStyle { active: matches!(self.loan_type, LoanType::BuildingSavings)})
                    )
            )
            .push(Button::new(&mut self.state.calc_button, Text::new("Calc")).on_press(LoanViewMessage::Calc));

        if let Some(result) = self.result.as_mut() {
            col = col
                .push(
                    Text::new(
                        format!("Monthly rate: {}, Paid interest: {}, Cleared: {}, Open: {}",
                                result.monthly_rate.round_dp(2),
                                result.overall.paid_interest.round_dp(2),
                                result.overall.cleared_amount.round_dp(2),
                                result.overall.remaining.round_dp(2))
                    )
                );

            let r = result.months.iter_mut().map(|r| {
                r.view()
            }).fold(Column::new(), |acc, v| {
                acc.push(v)
            });

            col = col.push(
                Scrollable::new(&mut self.state.result_scroller)
                    .width(Length::Fill)
                    .spacing(2)
                    .padding(10)
                    .push(r))
        }

        col.into()
    }
}

impl LoanView {
    fn calc_building_saving(&mut self) -> Result<CalcResultOverview, Box<dyn Error>> {
        let mut result = CalcResultOverview::default();

        let amount = self.amount.parse::<Decimal>()?;
        let interest_rate = Decimal::from_str(&self.interest_rate)? / dec!(100);
        let clearance_rate = Decimal::from_str(&self.clearance_rate)? / dec!(100);
        result.monthly_rate = amount * (interest_rate + clearance_rate ) / dec!(12);
        let runtime = self.runtime_years.parse::<i32>()?;

        for month in 0..=(runtime * 12) {
            let paid_interest_month = amount * interest_rate / dec!(12) as Decimal;
            result.overall.paid_interest += paid_interest_month;

            let saved_month = result.monthly_rate - paid_interest_month;
            result.overall.cleared_amount += saved_month;

            let remaining = amount - result.overall.cleared_amount;

            result.months.push(Box::new(CalcResult {
                month: month + 1,
                remaining: remaining.round_dp(2),
                cleared_amount: saved_month.round_dp(2),
                paid_interest: paid_interest_month.round_dp(2)
            }));

        }

        if let Some(last) = result.months.last() {
            result.overall.remaining = last.remaining.round_dp(2);
        }
        Ok(result)
    }

    fn calc_annuity(&mut self) -> Result<CalcResultOverview, Box<dyn Error>> {
        let mut result = CalcResultOverview::default();

        let amount = self.amount.parse::<Decimal>()?;
        let interest_rate = Decimal::from_str(&self.interest_rate)? / dec!(100);
        let clearance_rate = Decimal::from_str(&self.clearance_rate)? / dec!(100);
        result.monthly_rate = amount * (interest_rate + clearance_rate ) / dec!(12);
        let runtime = self.runtime_years.parse::<i32>()?;

        let mut remaining = amount;
        for month in 0..=(runtime * 12) {
            let paid_interest_month = remaining * interest_rate / dec!(12) as Decimal;
            result.overall.paid_interest += paid_interest_month;

            let cleared_month = result.monthly_rate - paid_interest_month;
            result.overall.cleared_amount += cleared_month;

            result.months.push(Box::new(CalcResult {
                month: month + 1,
                remaining: remaining.round_dp(2),
                cleared_amount: cleared_month.round_dp(2),
                paid_interest: paid_interest_month.round_dp(2)
            }));

            remaining -= cleared_month;
        }

        if let Some(last) = result.months.last() {
            result.overall.remaining = last.remaining.round_dp(2);
        }
        Ok(result)
    }
}