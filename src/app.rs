use crate::input_panel::{InputPanel, InputPanelMessage};
use crate::value_component::ValueInputMessage;
use iced::widget::row;
use iced::{executor, Application, Command, Element, Length, Theme};

pub struct Criteria {
    input_panel: InputPanel,
}

#[derive(Clone, Debug)]
pub enum Message {
    InputPanel(InputPanelMessage),
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
                    ValueInputMessage::Update => Command::none(),
                },
                InputPanelMessage::YMessage(y_message) => match y_message {
                    ValueInputMessage::ValueChanged(value) => {
                        self.input_panel.update_y_input(value);
                        Command::none()
                    }
                    ValueInputMessage::Update => Command::none(),
                },
                InputPanelMessage::GenerateButtonPressed => {
                    // let (x, y) = self.input_panel.get_x_y();
                    match self.input_panel.get_x_y() {
                        Ok((x, y)) => {self.input_panel.custom_text = format!("x = {x}, y = {y}")},
                        Err(message) => self.input_panel.custom_text = message.to_string()
                    }
                    // self.input_panel.custom_text = format!("x = {x}, y = {y}");
                    Command::none()
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        row![self
            .input_panel
            .view()
            .map(move |message| Message::InputPanel(message))]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
