use iced::{
    widget::{button, checkbox, column, row, Text},
    Element, Length,
};

use crate::value_component::{ValueInput, ValueInputMessage};

pub struct InputPanel {
    x_input: ValueInput,
    y_input: ValueInput,
    pub custom_text: String,
    pub risk_condition_checked: bool,
}

#[derive(Clone, Debug)]
pub enum InputPanelMessage {
    XMessage(ValueInputMessage),
    YMessage(ValueInputMessage),
    GenerateButtonPressed,
    RiskConditionChecked(bool),
}

impl InputPanel {
    pub fn new() -> Self {
        InputPanel {
            x_input: ValueInput::new(),
            y_input: ValueInput::new(),
            custom_text: "".to_string(),
            risk_condition_checked: false,
        }
    }

    pub fn view(&self) -> Element<InputPanelMessage> {
        column![
            row![
                column![Text::new(format!("Кількість рядків: "))],
                column![self
                    .x_input
                    .view()
                    .map(move |message| InputPanelMessage::XMessage(message)),],
            ]
            .align_items(iced::Alignment::Center),
            row![
                column![Text::new(format!("Кількість стовпців: "))],
                column![self
                    .y_input
                    .view()
                    .map(move |message| InputPanelMessage::YMessage(message))],
            ]
            .align_items(iced::Alignment::Center),
            checkbox(
                "В умовах ризику",
                self.risk_condition_checked,
                InputPanelMessage::RiskConditionChecked
            ),
            button("Генерувати").on_press(InputPanelMessage::GenerateButtonPressed),
            Text::new(&self.custom_text)
        ]
        .spacing(5)
        .padding(10)
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

    pub fn get_x_y(&self) -> Result<(usize, usize), &str> {
        let parse_positive_integer = |x: &String| -> Result<usize, &str> {
            if let Ok(x) = x.parse::<usize>() {
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
                Err(_) => Err("Кількість стовпців має бути додатнім цілим числом."),
            },
            Err(_) => Err("Кількість рядків має бути додатнім цілим числом."),
        }
    }
}
