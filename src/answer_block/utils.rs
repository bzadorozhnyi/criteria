use iced::widget::Text;

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
