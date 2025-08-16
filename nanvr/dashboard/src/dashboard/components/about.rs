use eframe::egui::{self, Frame, RichText, ScrollArea, Ui};
use gui_shared::theme;
use shared::{NANVR_NAME, NANVR_VERSION};

pub fn about_tab_ui(ui: &mut Ui) {
    ui.label(RichText::new(format!("{NANVR_NAME} streamer v{}", *NANVR_VERSION)).size(30.0));
    ui.add_space(10.0);
    ui.hyperlink_to("Visit us on GitHub", "https://github.com/nanvr/NaNVR");
    ui.hyperlink_to(
        "Latest release",
        "https://github.com/nanvr/NaNVR/releases/latest",
    );
    ui.add_space(10.0);
    ui.label("License:");
    Frame::group(ui.style())
        .fill(theme::DARKER_BG)
        .inner_margin(egui::vec2(15.0, 12.0))
        .show(ui, |ui| {
            ScrollArea::new([false, true])
                .id_salt("license_scroll")
                .show(ui, |ui| ui.label(include_str!("../../../../../LICENSE")))
        });
}
