use std::ops::RangeInclusive;

use iced::widget::{column, container, Text};
use iced::Element;

use crate::criterion::{
    get_max, get_min,
    risk_condition::{bayes, dispersion_minimization, modal, probability_maximization},
};

use super::utils::generate_variants_block;
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
        let probability_maximization_block = if a.len() > 0 && a[0].len() > 0 {
            probability_maximization(&a, &p, Some(a[0][0]))
        } else {
            probability_maximization(&a, &p, None)
        };
        let modal_block = modal(&a, &p);

        let probability_maximization_slider = if probability_maximization_block.is_some() {
            let range = get_probability_maximization_slider_range(&a);
            SliderBlock::new(*range.start(), 1.0, range)
        } else {
            SliderBlock::new(0.0, 1.0, 0.0..=10.0) // default (not be used)
        };

        RiskConditionAnswerBlocks {
            a,
            p,
            bayes_block,
            dispersion_minimization_block,
            probability_maximization_block,
            modal_block,
            probability_maximization_slider,
        }
    }

    pub fn view(&self) -> Element<RiskConditionAnswerBlockMessage> {
        let mut content = column![
            gen_block::<RiskConditionAnswerBlockMessage>(
                "Байєса",
                self.bayes_block.0,
                &self.bayes_block.1
            ),
            gen_block(
                "Мінімізація дисперсії",
                self.dispersion_minimization_block.0,
                &self.dispersion_minimization_block.1
            ),
        ];

        if self.probability_maximization_block.is_some() {
            let probability_maximization = self.probability_maximization_block.as_ref().unwrap();

            content = content.push(column![column![
                Text::new("Максимізація ймовірнсоті").height(20),
                self.probability_maximization_slider
                    .view()
                    .map(move |message| RiskConditionAnswerBlockMessage::Alpha(message)),
                Text::new(format!("Z = {:.2}", probability_maximization.0)),
                generate_variants_block(&probability_maximization.1)
            ]
            .spacing(10)]);
        }

        if self.modal_block.is_some() {
            let modal = self.modal_block.as_ref().unwrap();
            content = content.push(gen_block("Модальний", modal.0, &modal.1));
        }

        container(content.spacing(40)).into()
    }

    pub fn update_probability_maximization_block(&mut self) {
        self.probability_maximization_block = probability_maximization(
            &self.a,
            &self.p,
            Some(self.probability_maximization_slider.value),
        );
    }
}

pub fn get_probability_maximization_slider_range(a: &Vec<Vec<f32>>) -> RangeInclusive<f32> {
    let min = get_min(&a.iter().map(|row| get_min(&row)).collect());
    let max = get_max(&a.iter().map(|row| get_max(&row)).collect());

    min..=max
}
