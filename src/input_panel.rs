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

    pub fn get_x_y(&self) -> Result<(u32, u32), &str> {
        let parse_positive_integer = |x: &String| -> Result<u32, &str> {
            if let Ok(x) = x.parse::<u32>() {
                if x > 0 {
                    return Ok(x);
                } else {
                    return Err("Not positive integer");
                }
            } else {
                return Err("Not integer");
            }
        };

        match parse_positive_integer(&self.x_input.value) {
            Ok(x) => match parse_positive_integer(&self.y_input.value) {
                Ok(y) => Ok((x, y)),
                Err(_) => Err("Y is not positive integer"),
            },
            Err(_) => Err("X is not positive integer"),
        }
    }
}
