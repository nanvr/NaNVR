use egui::RichText;

pub fn button_text(text: &str) -> RichText {
    RichText::new(text).size(15.0)
}
