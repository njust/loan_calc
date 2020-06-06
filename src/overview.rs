use iced::{Element, Text, button, Command};
use crate::loan_view::{LoanView, LoanViewData};
use rust_decimal::Decimal;
use iced_native::{Column, Button};
use nfd::{Response};
use std::{
    result::Result,
};

const FILE_EXT: &'static str = "lc";

#[derive(Default)]
pub struct Overview {
    save_btn: button::State,
    load_btn: button::State,
}

#[derive(Debug, Clone)]
pub struct LoadResult {
    pub file: String,
    pub data: Vec<LoanViewData>,
}

#[derive(Debug, Clone)]
pub enum OverviewMessage {
    OpenSaveDlg(Vec<LoanViewData>),
    SaveDlgResult(Result<(), OverviewErr>),
    OpenLoadDlg,
    LoadDlgResult(Result<LoadResult, OverviewErr>),
}

#[derive(Debug, Clone)]
pub enum OverviewErr {
    ShowDlgFailed,
    Canceled,
    LoadFileFailed,
    WriteFileFailed,
    DeserializeFailed,
    SerializeFailed,
    MultipleFilesSelected,
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
        Column::new()
            .push(Text::new(format!("Monthly rate: {}, Remaining: {}, Paid interest: {}, Cleared: {}",
                          monthly_rate.round_dp(2),
                          remaining.round_dp(2),
                          paid_interest.round_dp(2),
                          cleared_amount.round_dp(2),
        ))).push(
            Button::new(&mut self.save_btn, Text::new("Save"))
                .on_press(OverviewMessage::OpenSaveDlg(loans.iter().map(|l| l.data.clone()).collect::<Vec<LoanViewData>>()))
        ).push(
            Button::new(&mut self.load_btn, Text::new("Load"))
                .on_press(OverviewMessage::OpenLoadDlg)
        )
            .into()
    }

    pub fn update(&mut self, msg: OverviewMessage) -> Command<OverviewMessage> {
        match msg {
            OverviewMessage::OpenSaveDlg(l) => {
                return Command::perform(Overview::save(l), OverviewMessage::SaveDlgResult);
            }
            OverviewMessage::OpenLoadDlg => {
                return Command::perform(Overview::load(), OverviewMessage::LoadDlgResult);
            }
            _ => Command::none()
        }
    }

    async fn load() -> Result<LoadResult, OverviewErr> {
        match nfd::open_file_dialog(Some(FILE_EXT), None).map_err(|_| OverviewErr::ShowDlgFailed)? {
            Response::Okay(path) => {
                let content = std::fs::read(&path).map_err(|_| OverviewErr::LoadFileFailed)?;
                let data = serde_json::from_slice::<Vec<LoanViewData>>(&content).map_err(|_| OverviewErr::DeserializeFailed)?;
                Ok(LoadResult {
                    data,
                    file: path
                })
            }
            Response::Cancel => Err(OverviewErr::Canceled),
            Response::OkayMultiple(_) => Err(OverviewErr::MultipleFilesSelected),
        }
    }

    async fn save(data: Vec<LoanViewData>) -> Result<(), OverviewErr>  {
        match nfd::open_save_dialog(Some(FILE_EXT), None).map_err(|_| OverviewErr::ShowDlgFailed)? {
            Response::Okay(path) => {
                let json = serde_json::to_string(&data).map_err(|_| OverviewErr::SerializeFailed)?;
                std::fs::write(&path, json).map_err(|_| OverviewErr::WriteFileFailed).map_err(|_| OverviewErr::WriteFileFailed)?;
                Ok(())
            }
            Response::OkayMultiple(_) => Err(OverviewErr::MultipleFilesSelected),
            Response::Cancel => Err(OverviewErr::Canceled)
        }
    }
}