mod style;

use rust_decimal_macros::*;
use std::str::FromStr;

use iced::{Button, button, TextInput, Sandbox, Text, Element, Settings, Row, Column, text_input, Scrollable, Length};

use std::{
    error::Error
};
use rust_decimal::Decimal;
use crate::style::ButtonStyle;

enum LoanType {
    Annuity,
    BuildingSavings,
}

impl Default for LoanType {
    fn default() -> Self {
        LoanType::Annuity
    }
}

#[derive(Default)]
struct LoanCalc {
    state: LoanCalcState,
    amount: String,
    interest_rate: String,
    clearance_rate: String,
    monthly_rate: String,
    runtime_years: String,
    loan_type: LoanType,
    result: Option<CalcResultOverview>
}

#[derive(Default)]
struct LoanCalcState {
    amount: text_input::State,
    interest_rate: text_input::State,
    clearance_rate: text_input::State,
    calc_button: button::State,
    result_scroller: iced::scrollable::State,
    runtime_years: text_input::State,
    monthly_rate: String,
    annuity_btn: button::State,
    building_savings_btn: button::State,
}

#[derive(Debug, Clone)]
enum LoanCalcMessage {
    AmountChanged(String),
    InterestRateChanged(String),
    ClearanceRateChanged(String),
    RuntimeChanged(String),
    ChangeTypeToAnnuity,
    ChangeTypeToBuildingSavings,
    Calc,
}

#[derive(Clone, Debug)]
enum AppMessage {
    LoanCalcMessage(usize, LoanCalcMessage),
    SelectLoan(usize),
    AddLoan
}

#[derive(Default)]
struct App {
    active: Option<usize>,
    loans: Vec<Box<LoanCalc>>,
    loan_tabs: Vec<Box<LoanTab>>,
    add_loan_btn: button::State,
}

struct LoanTab {
    idx: usize,
    name: String,
    button: button::State,
}

impl LoanTab {
    fn new(name: String, idx: usize) -> Self {
        Self {
            idx,
            name,
            button: button::State::default(),
        }
    }
    fn view(&mut self) -> Element<AppMessage> {
        Button::new(&mut self.button, Text::new(&self.name))
            .on_press(AppMessage::SelectLoan(self.idx))
            .into()
    }
}

impl Sandbox for App {
    type Message = AppMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Loan calc")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::LoanCalcMessage(idx, msg) => {
                if let Some(calc) = self.loans.get_mut(idx) {
                    calc.update(msg);
                }
            }
            AppMessage::SelectLoan(idx) => {
                self.active = Some(idx);
            }
            AppMessage::AddLoan =>  {
                self.loans.push(Box::new(LoanCalc::default()));
                self.loan_tabs.push(Box::new(LoanTab::new(String::from("New"), self.loan_tabs.len())));
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let mut buttons = self.loan_tabs.iter_mut().enumerate().map(|(idx, loan)| {
            loan.view()
        }).fold(Row::new(), |acc, tab| {
            acc.push(tab)
        });
        buttons = buttons.push(
            Button::new(&mut self.add_loan_btn, Text::new("Add"))
                .on_press(AppMessage::AddLoan)
        );
        let mut col = Column::new()
            .push(buttons);

        if let Some(idx) = self.active {
            if let Some(active) = self.loans.get_mut(idx) {
                col = col.push(active.view().map(move |m| AppMessage::LoanCalcMessage(idx, m)));
            }
        }
        col.into()
    }
}

impl LoanCalc {
    fn update(&mut self, message: LoanCalcMessage) {
        match message {
            LoanCalcMessage::AmountChanged(amount) => {
                self.amount = amount;
            }
            LoanCalcMessage::InterestRateChanged(rate) => {
                self.interest_rate = rate;
            }
            LoanCalcMessage::ClearanceRateChanged(clearance) => {
                self.clearance_rate = clearance;
            }
            LoanCalcMessage::Calc => {
                self.result.take();
                let result = match self.loan_type {
                    LoanType::Annuity => self.calc_annuity(),
                    LoanType::BuildingSavings => self.calc_building_saving(),
                };
                if let Ok(result) = result {
                    self.result = Some(result);
                }
            }
            LoanCalcMessage::RuntimeChanged(rt) => {
                self.runtime_years = rt;
            }
            LoanCalcMessage::ChangeTypeToAnnuity => {
                self.loan_type = LoanType::Annuity;
            }
            LoanCalcMessage::ChangeTypeToBuildingSavings => {
                self.loan_type = LoanType::BuildingSavings;
            }
        }
    }

    fn view(&mut self) -> Element<LoanCalcMessage> {
        let mut col = Column::new()
            .padding(20)
            .spacing(5)
            .push(TextInput::new(&mut self.state.amount, "Amount", &self.amount, LoanCalcMessage::AmountChanged))
            .push(TextInput::new(&mut self.state.interest_rate, "Interest Rate", &self.interest_rate, LoanCalcMessage::InterestRateChanged))
            .push(TextInput::new(&mut self.state.clearance_rate, "Clearance Rate", &self.clearance_rate, LoanCalcMessage::ClearanceRateChanged))
            .push(TextInput::new(&mut self.state.runtime_years, "Runtime years", &self.runtime_years, LoanCalcMessage::RuntimeChanged))
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.state.annuity_btn, Text::new("Annuity"))
                            .on_press(LoanCalcMessage::ChangeTypeToAnnuity)
                            .style(ButtonStyle { active: matches!(self.loan_type, LoanType::Annuity)})
                    )
                    .push(
                        Button::new(&mut self.state.building_savings_btn, Text::new("Building savings"))
                            .on_press(LoanCalcMessage::ChangeTypeToBuildingSavings)
                            .style(ButtonStyle { active: matches!(self.loan_type, LoanType::BuildingSavings)})
                    )
            )
            .push(Button::new(&mut self.state.calc_button, Text::new("Calc")).on_press(LoanCalcMessage::Calc));

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

#[derive(Default, Debug)]
struct CalcResultOverview {
    overall: CalcResult,
    monthly_rate: Decimal,
    months: Vec<Box<CalcResult>>
}

#[derive(Default, Debug)]
struct CalcResult {
    month: i32,
    remaining: Decimal,
    paid_interest: Decimal,
    cleared_amount: Decimal,
}

impl CalcResult {
    fn view(&mut self) -> Element<LoanCalcMessage> {
        Text::new(format!(
            "{} - {} - {} - {}", self.month, self.remaining, self.cleared_amount, self.paid_interest
        )).into()
    }
}

impl LoanCalc {
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

fn main() {
    App::run(Settings::default())
}
