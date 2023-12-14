use iced::{
    widget::{column, Radio},
    Element,
};

use crate::constants::{DEFAULT_PROFITS_LOSSES_CHOISE, RADIO_SIZE};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Choise {
    Profits,
    Losses,
}

pub struct ProfitsLossesRadio {
    selected_choice: Option<Choise>,
}

#[derive(Clone, Debug, Copy)]
pub enum ProfitsLossesRadioMessage {
    RadioSelected(Choise),
}

impl ProfitsLossesRadio {
    pub fn new() -> Self {
        ProfitsLossesRadio {
            selected_choice: Some(DEFAULT_PROFITS_LOSSES_CHOISE),
        }
    }

    pub fn view(&self) -> Element<ProfitsLossesRadioMessage> {
        column![
            Radio::new(
                "Прибутки",
                Choise::Profits,
                self.selected_choice,
                ProfitsLossesRadioMessage::RadioSelected
            )
            .size(RADIO_SIZE),
            Radio::new(
                "Збитки",
                Choise::Losses,
                self.selected_choice,
                ProfitsLossesRadioMessage::RadioSelected
            )
            .size(RADIO_SIZE),
        ]
        .spacing(10)
        .into()
    }

    pub fn get_selected_choise(&self) -> Choise {
        self.selected_choice.unwrap()
    }

    pub fn update_selected_choise(&mut self, new_choise: Choise) {
        self.selected_choice = Some(new_choise);
    }
}
