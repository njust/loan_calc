mod style;
mod loan_view;

use crate::loan_view::{
    LoanView,
    LoanViewMessage
};

use iced::{Button, button, Sandbox, Text, Element, Settings, Row, Column};


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
    LoanCalcMessage(usize, LoanViewMessage),
    SelectLoan(usize),
    AddLoan
}

#[derive(Default)]
struct App {
    active: Option<usize>,
    loans: Vec<Box<LoanView>>,
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
                self.loans.push(Box::new(LoanView::default()));
                self.loan_tabs.push(Box::new(LoanTab::new(String::from("New"), self.loan_tabs.len())));
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let mut buttons = self.loan_tabs.iter_mut().enumerate().map(|(_idx, loan)| {
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

fn main() {
    App::run(Settings::default())
}
