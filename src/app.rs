use crate::input_panel::{InputPanel, InputPanelMessage};
use crate::table::cell::CellMessage;
use crate::table::table::{InputTable, InputTableMessage};
use crate::value_component::ValueInputMessage;
use iced::widget::{column, row};
use iced::{executor, Application, Command, Element, Length, Theme};

pub struct Criteria {
    input_panel: InputPanel,
    input_table: InputTable,
}

#[derive(Clone, Debug)]
pub enum Message {
    InputPanel(InputPanelMessage),
    InputTable(InputTableMessage),
}

impl Application for Criteria {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        (
            Criteria {
                input_panel: InputPanel::new(),
                input_table: InputTable::new(0, 0),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Criteria")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::InputPanel(input_panel_message) => match input_panel_message {
                InputPanelMessage::XMessage(x_message) => match x_message {
                    ValueInputMessage::ValueChanged(value) => {
                        self.input_panel.update_x_input(value);
                        Command::none()
                    }
                },
                InputPanelMessage::YMessage(y_message) => match y_message {
                    ValueInputMessage::ValueChanged(value) => {
                        self.input_panel.update_y_input(value);
                        Command::none()
                    }
                },
                InputPanelMessage::GenerateButtonPressed => {
                    match self.input_panel.get_x_y() {
                        Ok((x, y)) => {
                            self.input_panel.custom_text = format!("x = {x}, y = {y}");
                            println!("x = {x}, y = {y}");
                            self.input_table = InputTable::new(x, y);
                        }
                        Err(message) => self.input_panel.custom_text = message.to_string(),
                    }
                    Command::none()
                }
            },
            Message::InputTable(input_table_message) => match input_table_message {
                InputTableMessage::CellUpdate(cell_update_message) => match cell_update_message {
                    CellMessage::Update(row, col, input_value_message) => match input_value_message
                    {
                        ValueInputMessage::ValueChanged(value) => {
                            self.input_table.update_cell(row, col, value);
                            Command::none()
                        }
                    },
                },
            },
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![self
                .input_panel
                .view()
                .map(move |message| Message::InputPanel(message))]
            .height(Length::FillPortion(2)),
            row![self
                .input_table
                .view()
                .map(move |message| Message::InputTable(message))]
            .height(Length::FillPortion(3)),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
