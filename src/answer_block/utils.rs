use iced::{
    widget::{column, Text},
    Element,
};

pub fn generate_variants_block(indeces: &Vec<usize>) -> Text<'static> {
    return Text::new(format!(
        "Варіанти: {}",
        indeces
            .iter()
            .map(|index| format!("Z_{}", index + 1))
            .collect::<Vec<_>>()
            .join(", ")
    ));
}

pub fn gen_block<T: 'static>(
    title: &'static str,
    answer_value: f32,
    indeces: &Vec<usize>,
) -> Element<'static, T> {
    column![
        Text::new(title).height(20),
        Text::new(format!("Z = {}", answer_value)),
        generate_variants_block(indeces)
    ]
    .spacing(10)
    .into()
}
