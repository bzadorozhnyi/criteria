use iced::{Application, Settings};

mod app;
mod input_panel;
mod value_component;

pub fn main() -> iced::Result {
    app::Criteria::run(Settings::default())
}
