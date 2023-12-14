use iced::Element;
use iced::{widget::column, Length};

use crate::constants::CELL_WIDTH;
use crate::value_component::{ValueInput, ValueInputMessage};

pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub input: ValueInput,
}

#[derive(Clone, Debug)]
pub enum CellMessage {
    Update(usize, usize, ValueInputMessage),
}

impl Cell {
    pub fn new(row: usize, col: usize) -> Self {
        Cell {
            row,
            col,
            input: ValueInput::new("Значення".to_string()),
        }
    }

    pub fn view(&self) -> Element<CellMessage> {
        column![self
            .input
            .view()
            .map(move |message| CellMessage::Update(self.row, self.col, message))]
        .width(Length::Fixed(CELL_WIDTH))
        .into()
    }
}
