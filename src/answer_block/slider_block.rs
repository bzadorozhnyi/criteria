use std::{fmt::Display, ops::RangeInclusive};

use iced::{
    widget::{column, container, slider, text},
    Element,
};

#[derive(Clone, Debug)]
pub struct SliderBlock<
    T: Clone + Display + Copy + From<u8> + PartialOrd + num_traits::cast::FromPrimitive,
> {
    pub value: T,
    step: T,
    range: RangeInclusive<T>,
}

#[derive(Clone, Debug)]
pub enum SliderBlockMessage<T> {
    AlphaChange(T),
}

impl<T: Clone + Display + Copy + From<u8> + PartialOrd + num_traits::cast::FromPrimitive>
    SliderBlock<T>
where
    f64: From<T>,
{
    pub fn new(start_value: T, step: T, range: RangeInclusive<T>) -> Self {
        SliderBlock {
            value: start_value,
            step,
            range,
        }
    }

    pub fn view(&self) -> Element<SliderBlockMessage<T>> {
        let slider = container(
            slider(
                self.range.clone(),
                self.value,
                SliderBlockMessage::AlphaChange,
            )
            .step(self.step)
            .width(500),
        );

        let text = text(format!("{:.2}", self.value));

        container(column![slider, text]).into()
    }
}
