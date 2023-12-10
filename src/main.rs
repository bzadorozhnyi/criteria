use iced::{Application, Settings};

mod app;
mod input_panel;
mod table;
mod value_component;
mod constants;

pub fn main() -> iced::Result {
    app::Criteria::run(Settings::default())
}
