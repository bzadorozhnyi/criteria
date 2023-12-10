use crate::constants::CELL_WIDTH;
use crate::table::cell::Cell;
use iced::widget::scrollable::Properties;
use iced::widget::{column, scrollable, Column, Row};
use iced::{Element, Length};

use super::cell::CellMessage;

pub struct InputTable {
    data: Vec<Vec<Cell>>,
}

#[derive(Clone, Debug)]
pub enum InputTableMessage {
    CellUpdate(CellMessage),
}

impl InputTable {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut matrix = Vec::with_capacity(rows);
        for row in 0..rows {
            let mut matrix_row = Vec::with_capacity(cols);
            for col in 0..cols {
                matrix_row.push(Cell::new(row, col));
            }
            matrix.push(matrix_row);
        }
        InputTable { data: matrix }
    }

    pub fn view(&self) -> Element<InputTableMessage> {
        let mut content_vec = Vec::new();

        for row in &self.data {
            let row_elements: Vec<_> = row
                .iter()
                .map(|cell| {
                    cell.view()
                        .map(move |message| InputTableMessage::CellUpdate(message))
                        .into()
                })
                .collect();

            content_vec.push(Row::with_children(row_elements).width(Length::Fill).into());
        }

        let columns_count = if self.data.is_empty() {
            0
        } else {
            self.data[0].len()
        } as f32;

        scrollable(column![
            column![Column::with_children(content_vec)].max_width(CELL_WIDTH * columns_count),
        ])
        .direction(scrollable::Direction::Both {
            vertical: Properties::default(),
            horizontal: Properties::default(),
        })
        .into()
    }

    pub fn update_cell(&mut self, row: usize, col: usize, value: String) {
        self.data[row][col].input.value = value;
    }
}
