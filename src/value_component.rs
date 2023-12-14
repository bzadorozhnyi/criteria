use iced::{
    widget::{row, text_input},
    Element,
};

#[derive(Clone, Debug)]
pub struct ValueInput {
    pub value: String,
    pub placeholder: String,
    _input: text_input::State,
}

#[derive(Clone, Debug)]
pub enum ValueInputMessage {
    ValueChanged(String),
}

impl ValueInput {
    pub fn new(placeholder: String) -> Self {
        ValueInput {
            value: String::new(),
            placeholder,
            _input: text_input::State::new(),
        }
    }

    pub fn view(&self) -> Element<ValueInputMessage> {
        row![text_input(&self.placeholder, &self.value)
            .on_input(|value| ValueInputMessage::ValueChanged(value))]
        .padding(10)
        .into()
    }
}
