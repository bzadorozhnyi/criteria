use iced::widget::column;
use iced::Element;

use crate::criterion::risk_condition::{
    bayes, dispersion_minimization, modal, probability_maximization,
};

use super::{
    slider_block::{SliderBlock, SliderBlockMessage},
    utils::gen_block,
};

pub struct RiskConditionAnswerBlocks {
    pub a: Vec<Vec<f32>>,
    pub p: Vec<f32>,
    bayes_block: (f32, Vec<usize>),
    dispersion_minimization_block: (f32, Vec<usize>),
    probability_maximization_block: Option<(f32, Vec<usize>)>,
    modal_block: Option<(f32, Vec<usize>)>,
    pub probability_maximization_slider: SliderBlock<f32>,
}

#[derive(Clone, Debug)]
pub enum RiskConditionAnswerBlockMessage {
    Alpha(SliderBlockMessage<f32>),
}

impl RiskConditionAnswerBlocks {
    pub fn new(a: Vec<Vec<f32>>, p: Vec<f32>) -> Self {
        let bayes_block = bayes(&a, &p);
        let dispersion_minimization_block = dispersion_minimization(&a, &p);
        let probability_maximization_block = probability_maximization(&a, &p, None);
        let modal_block = modal(&a, &p);

        RiskConditionAnswerBlocks {
            a,
            p,
            bayes_block,
            dispersion_minimization_block,
            probability_maximization_block,
            modal_block,
            probability_maximization_slider: SliderBlock::new(0.0, 1.0, 0.0..=10.0), // default (not be used)
        }
    }

    pub fn view(&self) -> Element<RiskConditionAnswerBlockMessage> {
        let probability_maximization_block = self.probability_maximization_block.as_ref().unwrap();

        let mut content = column![
            gen_block("Байєса", self.bayes_block.0, &self.bayes_block.1),
            gen_block(
                "Байєса",
                self.dispersion_minimization_block.0,
                &self.dispersion_minimization_block.1
            ),
            gen_block(
                "Байєса",
                probability_maximization_block.0,
                &probability_maximization_block.1
            )
        ];

        if self.modal_block.is_some() {
            let modal = self.modal_block.as_ref().unwrap();
            content = content.push(gen_block("Модальний", modal.0, &modal.1));
        }

        content.into()
    }
}
