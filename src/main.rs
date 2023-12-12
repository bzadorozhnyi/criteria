use iced::{Application, Settings};

mod answer_block;
mod app;
mod constants;
mod criterion;
mod input_panel;
mod table;
mod value_component;
mod utils;

pub fn main() -> iced::Result {
    app::Criteria::run(Settings::default())
}
