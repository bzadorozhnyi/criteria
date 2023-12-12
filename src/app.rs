use crate::answer_block::slider_block;
use crate::answer_block::uncertainty::{UncertaintyAnswerBlocks, UncertaintyAnswerBlocksMessage};
use crate::input_panel::{InputPanel, InputPanelMessage};
use crate::table::cell::CellMessage;
use crate::table::table::{InputTable, InputTableMessage};
use crate::utils::parse_data;
use crate::value_component::ValueInputMessage;
use iced::widget::{button, column, row};
use iced::{executor, Application, Command, Element, Length, Theme};

pub struct Criteria {
    input_panel: InputPanel,
    input_table: InputTable,
    generate_answer: bool,
    answer_block: Option<UncertaintyAnswerBlocks>,
}

#[derive(Clone, Debug)]
pub enum Message {
    InputPanel(InputPanelMessage),
    InputTable(InputTableMessage),
    GenerateCriterionsButtonPressed,
    AnswerBlock(UncertaintyAnswerBlocksMessage),
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
                generate_answer: false,
                answer_block: None,
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
            Message::GenerateCriterionsButtonPressed => {
                self.generate_answer = true;

                let input_data = &self.input_table.get_data();

                if let Ok(parsed_input_data) = parse_data(&input_data) {
                    self.answer_block = Some(UncertaintyAnswerBlocks::new(parsed_input_data));
                }

                Command::none()
            }
            Message::AnswerBlock(answer_block_message) => match answer_block_message {
                UncertaintyAnswerBlocksMessage::Alpha(hurwitz_block_message) => {
                    match hurwitz_block_message {
                        slider_block::SliderBlockMessage::AlphaChange(new_alpha) => {
                            if self.answer_block.is_some() {
                                self.answer_block.as_mut().unwrap().hurwitz_slider.value =
                                    new_alpha;

                                self.answer_block.as_mut().unwrap().update_hurwitz_block();
                            }

                            Command::none()
                        }
                    }
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let mut content = column![
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
            button("Evaluate criterions").on_press(Message::GenerateCriterionsButtonPressed),
        ];

        if self.generate_answer && self.answer_block.is_some() {
            content = content.push(
                self.answer_block
                    .as_ref()
                    .unwrap()
                    .view()
                    .map(move |message| Message::AnswerBlock(message)),
            )
        }

        content.width(Length::Fill).height(Length::Fill).into()
    }
}
