use iced::{
    widget::{button, Text},
    Element, Length,
};

use crate::value_component::{ValueInput, ValueInputMessage};

pub struct InputPanel {
    x_input: ValueInput,
    y_input: ValueInput,
    pub custom_text: String,
}

#[derive(Clone, Debug)]
pub enum InputPanelMessage {
    XMessage(ValueInputMessage),
    YMessage(ValueInputMessage),
    GenerateButtonPressed,
}

impl InputPanel {
    pub fn new() -> Self {
        InputPanel {
            x_input: ValueInput::new(),
            y_input: ValueInput::new(),
            custom_text: "".to_string(),
        }
    }

    pub fn view(&self) -> Element<InputPanelMessage> {
        iced::widget::column![
            iced::widget::row![iced::widget::column![
                self.x_input
                    .view("X")
                    .map(move |message| InputPanelMessage::XMessage(message)),
                Text::new(format!("Current X: {}", self.x_input.value))
            ],],
            iced::widget::row![iced::widget::column![
                self.y_input
                    .view("Y")
                    .map(move |message| InputPanelMessage::YMessage(message)),
                Text::new(format!("Current Y: {}", self.y_input.value))
            ],],
            button("Text").on_press(InputPanelMessage::GenerateButtonPressed),
            Text::new(&self.custom_text)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn update_x_input(&mut self, value: String) {
        self.x_input.value = value;
    }

    pub fn update_y_input(&mut self, value: String) {
        self.y_input.value = value;
    }

    pub fn get_x_y(&self) -> (String, String) {
        return (self.x_input.value.clone(), self.y_input.value.clone());
    }
}
