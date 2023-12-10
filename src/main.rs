use iced::{Application, Settings};

mod app;
mod constants;
mod criterion;
mod input_panel;
mod table;
mod value_component;

pub fn main() -> iced::Result {
    app::Criteria::run(Settings::default())
}
