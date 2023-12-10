use iced::{
    widget::{row, text_input},
    Element,
};

#[derive(Clone, Debug)]
pub struct ValueInput {
    pub value: String,
    _input: text_input::State,
}

#[derive(Clone, Debug)]
pub enum ValueInputMessage {
    ValueChanged(String),
}

impl ValueInput {
    pub fn new() -> Self {
        ValueInput {
            value: String::new(),
            _input: text_input::State::new(),
        }
    }

    pub fn view(&self) -> Element<ValueInputMessage> {
        row![text_input("Value", &self.value)
            .on_input(|value| ValueInputMessage::ValueChanged(value))]
        .padding(10)
        .into()
    }
}
