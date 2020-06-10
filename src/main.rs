mod style;
mod loan_view;
mod util;
mod overview;
mod form;

use crate::loan_view::{LoanView, LoanViewMessage, LoanViewData};

use serde::{Serialize, Deserialize};

use iced::{Button, button, Application, Text, Element, Settings, Row, Column, Length, Command, executor};
use crate::style::Icons;
use crate::overview::{Overview, OverviewMessage};
use crate::form::FormMessage;


#[derive(Debug, Clone, Serialize, Deserialize)]
enum LoanType {
    Annuity,
    BuildingSavings,
}

impl Default for LoanType {
    fn default() -> Self {
        LoanType::Annuity
    }
}

#[derive(Clone, Debug)]
enum AppMessage {
    LoanViewMessage(usize, LoanViewMessage),
    OverviewMessage(OverviewMessage),
    ShowOverview,
    SelectLoan(usize),
    AddLoan,
    DeleteLoan,
    FormMessage(FormMessage)
}

#[derive(Default)]
struct App {
    active: Option<usize>,
    loans: Vec<Box<LoanView>>,
    loan_tabs: Vec<Box<LoanTab>>,
    add_loan_btn: button::State,
    del_loan_btn: button::State,
    overview_btn: button::State,
    overview: Overview,
    title: String,
    test: form::Form,
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
    fn view(&mut self, active: bool) -> Element<AppMessage> {
        Button::new(&mut self.button, Text::new(&self.name))
            .on_press(AppMessage::SelectLoan(self.idx))
            .style(style::ButtonStyle{active})
            .into()
    }
}

const LOAN_DEFAULT_NAME: &'static str = "Loan";

impl App {
    fn add_loan(&mut self) {
        let idx = self.loan_tabs.len();
        let loan_name = format!("{} {}", LOAN_DEFAULT_NAME, idx + 1);
        self.loans.push(Box::new(LoanView::new(loan_name.clone())));
        self.loan_tabs.push(Box::new(LoanTab::new(loan_name, idx)));
        self.active = Some(idx);
    }

    fn add_loan_with_data(&mut self, data: LoanViewData) {
        let idx = self.loan_tabs.len();
        self.loan_tabs.push(Box::new(LoanTab::new(data.name.clone(), idx)));
        let mut loan_view = LoanView::new_with_data(data);
        loan_view.calc();
        self.loans.push(Box::new(loan_view));
        self.active = Some(idx);
    }

    fn delete_active_load(&mut self) {
        if let Some(active) = self.active.take() {
            for tab_idx in active..self.loan_tabs.len() {
                if let Some(tab) = self.loan_tabs.get_mut(tab_idx) {
                    if tab.idx > 0 {
                        tab.idx -= 1;
                    }
                }
            }

            self.loan_tabs.remove(active);
            self.loans.remove(active);
            if self.loans.len() > 0 {
                let next = if active == 0 {
                    0
                }else {
                    active -1
                };
                self.active = Some(next);
            }
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = Self::default();
        app.title = String::from("Loan calc");
        app.test.add("gerda");
        app.test.add("hudel");
        app.add_loan();
        (app, Command::none())
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::LoanViewMessage(idx, msg) => {
                if let LoanViewMessage::NameChanged(name) = &msg {
                    if let Some(tab) = self.loan_tabs.get_mut(idx) {
                        tab.name = name.to_owned();
                    }
                }
                if let Some(calc) = self.loans.get_mut(idx) {
                    calc.update(msg);
                }
            }
            AppMessage::SelectLoan(idx) => {
                self.active = Some(idx);
            }
            AppMessage::ShowOverview => {
                self.active = None;
            }
            AppMessage::AddLoan =>  {
                self.add_loan();
            }
            AppMessage::OverviewMessage(msg) => {
                if let OverviewMessage::LoadDlgResult(r) = &msg {
                    if let Ok(loaded) = r.clone() {
                        self.loans.clear();
                        self.loan_tabs.clear();
                        self.title = format!("Loan calc - {}", loaded.file);
                        for loan in loaded.data {
                            self.add_loan_with_data(loan);
                        }
                    }
                }else {
                    return self.overview.update(msg).map(|m| AppMessage::OverviewMessage(m));
                }
            }
            AppMessage::DeleteLoan => {
                self.delete_active_load();
            }
            AppMessage::FormMessage(m) => {
                self.test.update(m);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let active_tab = self.active.map(|i|i as i16).unwrap_or(-1);
        let mut buttons = Row::new()
            .width(Length::Fill)
            .push(
                Button::new(&mut self.overview_btn, Text::new("Overview"))
                    .style(style::ButtonStyle{active: !self.active.is_some()})
                    .on_press(AppMessage::ShowOverview)
            );

        buttons = self.loan_tabs.iter_mut().enumerate().map( |(idx, loan)| {
            loan.view(idx as i16 == active_tab)
        }).fold(buttons, |acc, tab| {
            acc.push(tab)
        });
        buttons = buttons.push(
            Button::new(&mut self.add_loan_btn, Icons::add_icon())
                .style(style::IconButtonStyle{})
                .on_press(AppMessage::AddLoan)
        );

        let mut col = Column::new()
            .push(
                Row::new()
                    .width(Length::Fill)
                    .push(buttons)
                    .push(
                        Button::new(&mut self.del_loan_btn, Icons::delete_icon())
                            .on_press(AppMessage::DeleteLoan)
                            .style(style::IconButtonStyle{})
                    )
            );

        if let Some(idx) = self.active {
            if let Some(active) = self.loans.get_mut(idx) {
                col = col.push(active.view().map(move |m| AppMessage::LoanViewMessage(idx, m)));
            }
        }else {
            col = col.push(self.overview.view(&self.loans).map(|m| AppMessage::OverviewMessage(m)));
        }
        col = col.push(
            self.test.view().map(|m| AppMessage::FormMessage(m))
        );
        col.into()
    }
}

fn main() {
    App::run(Settings::default())
}
