use iced::{
    widget::{row, text_input, Text},
    Element, Length,
};

#[derive(Clone, Debug)]
pub struct ValueInput {
    pub value: String,
    _input: text_input::State,
}

#[derive(Clone, Debug)]
pub enum ValueInputMessage {
    ValueChanged(String),
    Update,
}

impl ValueInput {
    pub fn new() -> Self {
        ValueInput {
            value: String::new(),
            _input: text_input::State::new(),
        }
    }

    pub fn view(&self, label: &str) -> Element<ValueInputMessage> {
        row![
            Text::new(format!("{}:", label)).width(Length::Shrink),
            row![text_input("Value", &self.value)
                .on_input(|value| ValueInputMessage::ValueChanged(value))
                .on_submit((|| ValueInputMessage::Update)())]
            .padding(10)
        ]
        .align_items(iced::Alignment::Center)
        .padding(10)
        .into()
    }
}
