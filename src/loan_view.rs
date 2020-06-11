use crate::{LoanType};
use rust_decimal_macros::*;
use std::str::FromStr;

use iced::{Button, button, Text, Element, Row, Column, Scrollable, Length};

use serde::{Deserialize, Serialize};

use std::{
    error::Error
};
use rust_decimal::Decimal;
use crate::{
    style::ButtonStyle,
    form,
};
use crate::form::{FormMessage, FormTextInputMessage};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoanViewData {
    pub name: String,
    amount: String,
    interest_rate: String,
    clearance_rate: String,
    runtime_years: String,
    loan_type: LoanType,
}

#[derive(Default)]
pub struct LoanView {
    state: LoanViewState,
    pub data: LoanViewData,
    pub result: Option<CalcResultOverview>
}

#[derive(Copy, Clone, Debug)]
pub enum LoanFormData {
    None,
    Name,
    Amount,
    InterestRate,
    ClearanceRate,
    RuntimeYears,
}

impl Default for LoanFormData {
    fn default() -> Self {
        LoanFormData::None
    }
}

#[derive(Default)]
struct LoanViewState {
    form: form::Form<LoanFormData>,
    calc_button: button::State,
    result_scroller: iced::scrollable::State,
    annuity_btn: button::State,
    building_savings_btn: button::State,
}

#[derive(Debug, Clone)]
pub enum LoanViewMessage {
    ChangeTypeToAnnuity,
    ChangeTypeToBuildingSavings,
    Calc,
    LoanForm(FormMessage<LoanFormData>),
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
        let form = form::Form::new()
            .push(LoanFormData::Name,"Name", Some(name.clone()))
            .push(LoanFormData::Amount, "Amount", None)
            .push(LoanFormData::InterestRate, "Interest rate", None)
            .push(LoanFormData::ClearanceRate,"Clearance rate", None)
            .push(LoanFormData::RuntimeYears,"Runtime", None);

        Self {
            state: LoanViewState {
                form,
                ..Default::default()
            },
            data: LoanViewData {
                name,
                ..LoanViewData::default()
            },
            result: None
        }
    }
    pub fn new_with_data(data: LoanViewData) -> Self {
        Self {
            state: Default::default(),
            data,
            result: None
        }
    }
    pub fn update(&mut self, message: LoanViewMessage) {
        match message {
            LoanViewMessage::Calc => {
                self.result.take();
                let result = match self.data.loan_type {
                    LoanType::Annuity => self.calc_annuity(),
                    LoanType::BuildingSavings => self.calc_building_saving(),
                };
                if let Ok(result) = result {
                    self.result = Some(result);
                }
            }
            LoanViewMessage::ChangeTypeToAnnuity => {
                self.data.loan_type = LoanType::Annuity;
            }
            LoanViewMessage::ChangeTypeToBuildingSavings => {
                self.data.loan_type = LoanType::BuildingSavings;
            }
            LoanViewMessage::LoanForm(m) => {
                if let FormMessage::TextInputMessage(i, _idx, FormTextInputMessage::InputChanged(value) ) = &m {
                    match i {
                        LoanFormData::Name => self.data.name = value.clone(),
                        LoanFormData::Amount => self.data.amount = value.clone(),
                        LoanFormData::InterestRate => self.data.interest_rate = value.clone(),
                        LoanFormData::ClearanceRate => self.data.clearance_rate = value.clone(),
                        LoanFormData::RuntimeYears => self.data.runtime_years = value.clone(),
                        _ => ()
                    }
                }
                self.state.form.update(m);
            },
        }
    }

    pub fn view(&mut self) -> Element<LoanViewMessage> {
        let mut col = Column::new()
            .padding(20)
            .spacing(5)
            .push(self.state.form.view().map(|m| LoanViewMessage::LoanForm(m)))
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.state.annuity_btn, Text::new("Annuity"))
                            .on_press(LoanViewMessage::ChangeTypeToAnnuity)
                            .style(ButtonStyle { active: matches!(self.data.loan_type, LoanType::Annuity)})
                    )
                    .push(
                        Button::new(&mut self.state.building_savings_btn, Text::new("Building savings"))
                            .on_press(LoanViewMessage::ChangeTypeToBuildingSavings)
                            .style(ButtonStyle { active: matches!(self.data.loan_type, LoanType::BuildingSavings)})
                    )
            )
            .push(Button::new(&mut self.state.calc_button, Text::new("Calc")).on_press(LoanViewMessage::Calc));

        if let Some(result) = self.result.as_mut() {
            col = col
                .push(
                    Text::new(
                        format!("Monthly rate: {}\nPaid interest: {}\nRemaining: {}\nCleared: {}",
                                result.monthly_rate.round_dp(2),
                                result.overall.paid_interest.round_dp(2),
                                result.overall.remaining.round_dp(2),
                                result.overall.cleared_amount.round_dp(2),
                            )
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
    pub fn calc(&mut self) {
        let r = match self.data.loan_type {
            LoanType::Annuity => self.calc_annuity(),
            LoanType::BuildingSavings => self.calc_building_saving()
        };
        if let Ok(r) = r {
            self.result = Some(r);
        }
    }
    fn calc_building_saving(&mut self) -> Result<CalcResultOverview, Box<dyn Error>> {
        let mut result = CalcResultOverview::default();

        let amount = self.data.amount.parse::<Decimal>()?;
        let interest_rate = Decimal::from_str(&self.data.interest_rate)? / dec!(100);
        let clearance_rate = Decimal::from_str(&self.data.clearance_rate)? / dec!(100);
        result.monthly_rate = amount * (interest_rate + clearance_rate ) / dec!(12);
        let runtime = self.data.runtime_years.parse::<i32>()?;

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

        let amount = self.data.amount.parse::<Decimal>()?;
        let interest_rate = Decimal::from_str(&self.data.interest_rate)? / dec!(100);
        let clearance_rate = Decimal::from_str(&self.data.clearance_rate)? / dec!(100);
        result.monthly_rate = amount * (interest_rate + clearance_rate ) / dec!(12);
        let runtime = self.data.runtime_years.parse::<i32>()?;

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