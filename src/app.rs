use crate::answer_block::risk_condition::{
    RiskConditionAnswerBlockMessage, RiskConditionAnswerBlocks,
};
use crate::answer_block::slider_block::{self};
use crate::answer_block::uncertainty::{UncertaintyAnswerBlocks, UncertaintyAnswerBlocksMessage};
use crate::input_panel::{InputPanel, InputPanelMessage};
use crate::table::cell::CellMessage;
use crate::table::table::{InputTable, InputTableMessage};
use crate::utils::{parse_data, parse_p};
use crate::value_component::ValueInputMessage;
use iced::widget::scrollable::Properties;
use iced::widget::{button, column, container, row, scrollable, Text};
use iced::{executor, Application, Command, Element, Length, Theme};

pub struct Criteria {
    input_panel: InputPanel,
    input_table: InputTable,
    generate_answer: bool,
    uncertainty_answer_block: Option<UncertaintyAnswerBlocks>,
    risk_condition_answer_block: Option<RiskConditionAnswerBlocks>,
    answer_generation_error_text: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    InputPanel(InputPanelMessage),
    InputTable(InputTableMessage),
    GenerateCriterionsButtonPressed,
    UncertaintyAnswerBlock(UncertaintyAnswerBlocksMessage),
    RiskConditionAnswerBlock(RiskConditionAnswerBlockMessage),
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
                input_table: InputTable::new(0, 0, false),
                generate_answer: false,
                uncertainty_answer_block: None,
                risk_condition_answer_block: None,
                answer_generation_error_text: String::new(),
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
                            self.input_table =
                                InputTable::new(x, y, self.input_panel.risk_condition_checked);
                        }
                        Err(message) => self.input_panel.custom_text = message.to_string(),
                    }
                    Command::none()
                }
                InputPanelMessage::RiskConditionChecked(risk_condition_checked) => {
                    self.input_panel.risk_condition_checked = risk_condition_checked;

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
                InputTableMessage::ProbabilityCellUpdate(cell_update_message) => {
                    match cell_update_message {
                        CellMessage::Update(_, col, input_value_message) => {
                            match input_value_message {
                                ValueInputMessage::ValueChanged(value) => {
                                    self.input_table.update_probability_cell(col, value);
                                    Command::none()
                                }
                            }
                        }
                    }
                }
            },
            Message::GenerateCriterionsButtonPressed => {
                self.generate_answer = true;

                let input_data = &self.input_table.get_data();
                let p = &self.input_table.get_p();

                if let Ok(parsed_input_data) = parse_data(&input_data) {
                    if self.input_panel.risk_condition_checked {
                        if let Ok(parsed_p) = parse_p(&p) {
                            self.uncertainty_answer_block = None;
                            self.risk_condition_answer_block =
                                Some(RiskConditionAnswerBlocks::new(parsed_input_data, parsed_p));

                            self.answer_generation_error_text = String::new();
                        }
                        else {
                            self.uncertainty_answer_block = None;
                            self.risk_condition_answer_block = None;
                            
                            self.answer_generation_error_text = "Перевірте заповнені ймовірності на коректність.".to_string();
                        }
                    } else {
                        self.uncertainty_answer_block =
                        Some(UncertaintyAnswerBlocks::new(parsed_input_data));
                        self.risk_condition_answer_block = None;

                        self.answer_generation_error_text = String::new();
                    }
                }
                else {
                    self.uncertainty_answer_block = None;
                    self.risk_condition_answer_block = None;

                    self.answer_generation_error_text = "Перевірте заповнену матрицю на коректність.".to_string();
                }

                Command::none()
            }
            Message::UncertaintyAnswerBlock(answer_block_message) => match answer_block_message {
                UncertaintyAnswerBlocksMessage::Alpha(hurwitz_block_message) => {
                    match hurwitz_block_message {
                        slider_block::SliderBlockMessage::AlphaChange(new_alpha) => {
                            if self.uncertainty_answer_block.is_some() {
                                self.uncertainty_answer_block
                                    .as_mut()
                                    .unwrap()
                                    .hurwitz_slider
                                    .value = new_alpha;

                                self.uncertainty_answer_block
                                    .as_mut()
                                    .unwrap()
                                    .update_hurwitz_block();
                            }

                            Command::none()
                        }
                    }
                }
                UncertaintyAnswerBlocksMessage::ProfitsLossesRadioChanged(radio_message) => match radio_message {
                    crate::answer_block::profits_losses_radio::ProfitsLossesRadioMessage::RadioSelected(new_choice) => {
                        self.uncertainty_answer_block.as_mut().unwrap().profits_losses_radio.update_selected_choise(new_choice);
                        self.uncertainty_answer_block.as_mut().unwrap().update_minimax();
                        Command::none()
                    },
                },
            },
            Message::RiskConditionAnswerBlock(answer_block_message) => match answer_block_message {
                RiskConditionAnswerBlockMessage::Alpha(probability_maximization_block_message) => {
                    match probability_maximization_block_message {
                        slider_block::SliderBlockMessage::AlphaChange(new_alpha) => {
                            if self.risk_condition_answer_block.is_some() {
                                self.risk_condition_answer_block
                                    .as_mut()
                                    .unwrap()
                                    .probability_maximization_slider
                                    .value = new_alpha;

                                self.risk_condition_answer_block
                                    .as_mut()
                                    .unwrap()
                                    .update_probability_maximization_block();
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
            .height(Length::Fixed(250.0)),
            row![self
                .input_table
                .view()
                .map(move |message| Message::InputTable(message))],
        ];

        if self.input_table.is_non_empty() {
            content = content.push(
                button("Визначити критерії").on_press(Message::GenerateCriterionsButtonPressed),
            )
        }

        if self.generate_answer {
            if self.uncertainty_answer_block.is_some() {
                content = content.push(row![self
                    .uncertainty_answer_block
                    .as_ref()
                    .unwrap()
                    .view()
                    .map(move |message| Message::UncertaintyAnswerBlock(message)),])
            }

            if self.risk_condition_answer_block.is_some() {
                content = content.push(row![self
                    .risk_condition_answer_block
                    .as_ref()
                    .unwrap()
                    .view()
                    .map(move |message| Message::RiskConditionAnswerBlock(message))])
            }

            if !self.answer_generation_error_text.is_empty() {
                content = content.push(row![
                    Text::new(self.answer_generation_error_text.clone())
                ])
            }
        }

        scrollable(container(
            content
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10)
                .spacing(5),
        ))
        .direction(scrollable::Direction::Vertical(Properties::default()))
        .into()
    }
}
