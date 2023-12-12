use crate::criterion::{hurwitz, maximax, minimax, savage};
use iced::{
    widget::{column, Text},
    Element,
};

use super::{
    slider_block::{SliderBlock, SliderBlockMessage},
    utils::generate_variants_block,
};

pub struct UncertaintyAnswerBlocks {
    pub a: Vec<Vec<f32>>,
    maximax_block: (f32, Vec<usize>),
    minimax_block: (f32, Vec<usize>),
    hurwitz_block: (f32, Vec<usize>),
    savage_block: (f32, Vec<usize>),
    pub hurwitz_slider: SliderBlock::<f32>,
}

#[derive(Clone, Debug)]
pub enum UncertaintyAnswerBlocksMessage {
    Alpha(SliderBlockMessage<f32>),
}

impl UncertaintyAnswerBlocks {
    pub fn new(a: Vec<Vec<f32>>) -> Self {
        let (maximax_answer, maximax_indeces) = maximax(&a);
        let (minimax_answer, minimax_indeces) = minimax(&a);
        let (hurwitz_answer, hurwitz_indeces) = hurwitz(&a, 0.5);
        let (savage_answer, savage_indeces) = savage(&a);

        UncertaintyAnswerBlocks {
            a,
            maximax_block: (maximax_answer, maximax_indeces),
            minimax_block: (minimax_answer, minimax_indeces),
            hurwitz_block: (hurwitz_answer, hurwitz_indeces),
            hurwitz_slider: SliderBlock::new(0.5, 0.01, 0.0..=1.0),
            savage_block: (savage_answer, savage_indeces),
        }
    }

    pub fn view(&self) -> Element<UncertaintyAnswerBlocksMessage> {
        column![
            gen_block(
                "Максімакс".to_string(),
                self.maximax_block.0,
                &self.maximax_block.1,
            ),
            gen_block(
                "Мінімакс".to_string(),
                self.minimax_block.0,
                &self.minimax_block.1,
            ),
            self.hurwitz_slider
                .view()
                .map(move |message| UncertaintyAnswerBlocksMessage::Alpha(message)),
            gen_block(
                "Hurwitz".to_string(),
                self.hurwitz_block.0,
                &self.hurwitz_block.1
            ),
            gen_block(
                "Севіджа".to_string(),
                self.savage_block.0,
                &self.savage_block.1,
            )
        ]
        .into()
    }

    pub fn update_hurwitz_block(&mut self) {
        self.hurwitz_block = hurwitz(&self.a, self.hurwitz_slider.value);
    }
}

fn gen_block(
    title: String,
    answer_value: f32,
    indeces: &Vec<usize>,
) -> Element<'static, UncertaintyAnswerBlocksMessage> {
    column![
        Text::new(title).height(20),
        Text::new(format!("Z = {}", answer_value)),
        generate_variants_block(indeces)
    ]
    .spacing(10)
    .into()
}