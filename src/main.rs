use rust_decimal_macros::*;
use std::str::FromStr;

use iced::{Button, button, TextInput, Sandbox, Text, Element, Settings, Row, Column, text_input, Scrollable, Length};

use std::{
    error::Error
};
use rust_decimal::Decimal;

#[derive(Default)]
struct LoanCalc {
    state: LoanCalcState,
    amount: String,
    interest_rate: String,
    clearance_rate: String,
    monthly_rate: String,
    runtime_years: String,
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
}

#[derive(Debug, Clone)]
enum Message {
    AmountChanged(String),
    InterestRateChanged(String),
    ClearanceRateChanged(String),
    RuntimeChanged(String),
    Calc,
}

impl Sandbox for LoanCalc {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Loan calc")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::AmountChanged(amount) => {
                self.amount = amount;
            }
            Message::InterestRateChanged(rate) => {
                self.interest_rate = rate;
            }
            Message::ClearanceRateChanged(clearance) => {
                self.clearance_rate = clearance;
            }
            Message::Calc => {
                self.result.take();
                if let Ok(result) = self.calc() {
                    self.result = Some(result);
                }
            }
            Message::RuntimeChanged(rt) => {
                self.runtime_years = rt;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut col = Column::new()
            .padding(20)
            .spacing(5)
            .push(TextInput::new(&mut self.state.amount, "Amount", &self.amount, Message::AmountChanged))
            .push(TextInput::new(&mut self.state.interest_rate, "Interest Rate", &self.interest_rate, Message::InterestRateChanged))
            .push(TextInput::new(&mut self.state.clearance_rate, "Clearance Rate", &self.clearance_rate, Message::ClearanceRateChanged))
            .push(TextInput::new(&mut self.state.runtime_years, "Runtime years", &self.runtime_years, Message::RuntimeChanged))
            .push(Button::new(&mut self.state.calc_button, Text::new("Calc")).on_press(Message::Calc));

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
    fn view(&mut self) -> Element<Message> {
        Text::new(format!(
            "{} - {} - {} - {}", self.month, self.remaining, self.cleared_amount, self.paid_interest
        )).into()
    }
}

impl LoanCalc {
    fn calc(&mut self) -> Result<CalcResultOverview, Box<dyn Error>> {
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
    LoanCalc::run(Settings::default())
}
