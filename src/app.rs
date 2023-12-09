use crate::value_component::{ValueInput, ValueInputMessage};
use iced::widget::{button, Text};
use iced::{executor, Application, Command, Element, Length, Theme};

pub struct Criteria {
    x_input: ValueInput,
    y_input: ValueInput,
    custom_text: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    XMessage(ValueInputMessage),
    YMessage(ValueInputMessage),
    GenerateButtonPressed,
}

impl Application for Criteria {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        (
            Criteria {
                x_input: ValueInput::new(),
                y_input: ValueInput::new(),
                custom_text: "".to_string(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Criteria")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::XMessage(x_message) => match x_message {
                ValueInputMessage::ValueChanged(value) => {
                    self.x_input.value = value;
                    Command::none()
                }
                ValueInputMessage::Update => Command::none(),
            },
            Message::YMessage(y_message) => match y_message {
                ValueInputMessage::ValueChanged(value) => {
                    self.y_input.value = value;
                    Command::none()
                }
                ValueInputMessage::Update => Command::none(),
            },
            Message::GenerateButtonPressed => {
                self.custom_text =
                    format!("x = {}, y = {}", self.x_input.value, self.y_input.value);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        iced::widget::column![
            iced::widget::row![iced::widget::column![
                self.x_input
                    .view("X")
                    .map(move |message| Message::XMessage(message)),
                Text::new(format!("Current X: {}", self.x_input.value))
            ],],
            iced::widget::row![iced::widget::column![
                self.y_input
                    .view("Y")
                    .map(move |message| Message::YMessage(message)),
                Text::new(format!("Current Y: {}", self.y_input.value))
            ],],
            button("Text").on_press(Message::GenerateButtonPressed),
            Text::new(&self.custom_text)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
