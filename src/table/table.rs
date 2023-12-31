use crate::constants::{CELL_WIDTH, TEXT_CELL_WIDTH};
use crate::table::cell::Cell;
use iced::widget::scrollable::Properties;
use iced::widget::{column, row, scrollable, Column, Row, Text};
use iced::{Element, Length};

use super::cell::CellMessage;

pub struct InputTable {
    data: Vec<Vec<Cell>>,
    p: Vec<Cell>,
    pub risk_condition: bool,
}

#[derive(Clone, Debug)]
pub enum InputTableMessage {
    CellUpdate(CellMessage),
    ProbabilityCellUpdate(CellMessage),
}

impl InputTable {
    pub fn new(rows: usize, cols: usize, risk_condition: bool) -> Self {
        let mut matrix = Vec::with_capacity(rows);
        for row in 0..rows {
            let mut matrix_row = Vec::with_capacity(cols);
            for col in 0..cols {
                matrix_row.push(Cell::new(row, col));
            }
            matrix.push(matrix_row);
        }
        let mut p = Vec::with_capacity(cols);
        for col in 0..cols {
            p.push(Cell::new(0, col));
        }

        InputTable {
            data: matrix,
            p,
            risk_condition,
        }
    }

    pub fn view(&self) -> Element<InputTableMessage> {
        let mut data_vec = Vec::new();

        let mut heading: Vec<Element<InputTableMessage>> = Vec::new();
        heading.push(
            column![Text::new("")]
                .align_items(iced::Alignment::Center)
                .width(TEXT_CELL_WIDTH)
                .padding(10)
                .into(),
        );

        if self.is_non_empty() {
            for col in 1..=self.data[0].len() {
                heading.push(
                    column![Text::new(format!("y{col}"))]
                        .align_items(iced::Alignment::Center)
                        .width(CELL_WIDTH)
                        .padding(10)
                        .into(),
                );
            }
        }

        let heading_row = Row::with_children(heading).align_items(iced::Alignment::Center);

        for (row_index, row) in self.data.iter().enumerate() {
            let row_elements: Vec<_> = row
                .iter()
                .map(|cell| {
                    cell.view()
                        .map(move |message| InputTableMessage::CellUpdate(message))
                        .into()
                })
                .collect();

            data_vec.push(
                row![
                    column![Text::new(format!("x{}", row_index + 1))]
                        .align_items(iced::Alignment::Center)
                        .width(TEXT_CELL_WIDTH)
                        .padding(10),
                    Row::with_children(row_elements).width(Length::Fill)
                ]
                .align_items(iced::Alignment::Center)
                .into(),
            );
        }

        let columns_count = if self.data.is_empty() {
            0
        } else {
            self.data[0].len()
        } as f32;

        let mut content = column![Column::with_children(data_vec)]
            .max_width(CELL_WIDTH * columns_count + TEXT_CELL_WIDTH);
        if self.risk_condition {
            let p_table = Row::with_children(
                self.p
                    .iter()
                    .map(|cell| {
                        cell.view()
                            .map(move |message| InputTableMessage::ProbabilityCellUpdate(message))
                    })
                    .collect(),
            );

            content = column![
                column![row![
                    column![column![Text::new("p")]
                        .align_items(iced::Alignment::Center)
                        .width(TEXT_CELL_WIDTH)
                        .padding(10),],
                    p_table
                ]]
                .max_width(CELL_WIDTH * columns_count + TEXT_CELL_WIDTH),
                content
            ]
        }

        scrollable(column![
            column![heading_row].max_width(CELL_WIDTH * columns_count + TEXT_CELL_WIDTH),
            content.spacing(5),
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

    pub fn update_probability_cell(&mut self, index: usize, value: String) {
        self.p[index].input.value = value;
    }

    pub fn get_data(&self) -> Vec<Vec<String>> {
        self.data
            .iter()
            .map(|row| row.iter().map(|cell| cell.input.value.clone()).collect())
            .collect()
    }

    pub fn get_p(&self) -> Vec<String> {
        self.p.iter().map(|cell| cell.input.value.clone()).collect()
    }

    pub fn is_non_empty(&self) -> bool {
        self.data.len() > 0 && self.data[0].len() > 0
    }
}
