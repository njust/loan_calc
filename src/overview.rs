use iced::{Element, Text, button, Command};
use crate::loan_view::{LoanView, LoanViewData};
use rust_decimal::Decimal;
use iced_native::{Column, Button};
use nfd::{Response, DialogType};
use std::path::PathBuf;
use std::error::Error;
use std::io::ErrorKind;

#[derive(Default)]
pub struct Overview {
    save_btn: button::State,
    load_btn: button::State,
}

#[derive(Debug, Clone)]
pub enum OverviewMessage {
    OpenSaveDlg(Vec<LoanViewData>),
    SaveDlgResult(std::result::Result<String, DlgErr>),
    OpenLoadDlg,
    LoadDlgResult(std::result::Result<Vec<LoanViewData>, DlgErr>),
}

#[derive(Debug, Clone)]
pub enum DlgErr {
    Canceled
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
            OverviewMessage::SaveDlgResult(res) => {
                if let Ok(r) = res {
                    println!("R: {}", r);
                }
                Command::none()
            }
            OverviewMessage::OpenLoadDlg => {
                return Command::perform(Overview::load(), OverviewMessage::LoadDlgResult)
            }
            OverviewMessage::LoadDlgResult(r) => {
                Command::none()
            }
        }
    }

    async fn load() -> std::result::Result<Vec<LoanViewData>, DlgErr> {
        match nfd::open_file_dialog(Some("lc"), None).map_err(|_| DlgErr::Canceled)? {
            Response::Okay(path) => {
                let content = std::fs::read(path).map_err(|e| DlgErr::Canceled)?;
                let data = serde_json::from_slice::<Vec<LoanViewData>>(&content).map_err(|e| DlgErr::Canceled)?;
                Ok(data)
            }
            Response::Cancel => Err(DlgErr::Canceled),
            Response::OkayMultiple(_) => Err(DlgErr::Canceled),
        }
    }

    async fn save(data: Vec<LoanViewData>) -> std::result::Result<String, DlgErr>  {
        match nfd::open_save_dialog(Some("lc"), None).map_err(|e| DlgErr::Canceled)? {
            Response::Okay(path) => {
                match serde_json::to_string(&data) {
                    Ok(r) => {
                        std::fs::write(&path, r).map_err(|e| DlgErr::Canceled);
                    }
                    Err(e) => {
                        eprintln!("E: {:?}", e);
                    }
                }
                Ok(path)
            }
            Response::OkayMultiple(path_list) => {
                Err(DlgErr::Canceled)
            }
            Response::Cancel => {
                Err(DlgErr::Canceled)
            }
        }
    }

}