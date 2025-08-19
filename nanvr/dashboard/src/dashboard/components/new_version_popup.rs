use std::{path::PathBuf, process::Command};

use eframe::egui::{self, Context, OpenUrl, Ui};
use gui_shared::ModalButton;
use net_packets::{PathValuePair, ServerRequest};
use shared::{NANVR_GH_REPO_PATH, NANVR_LOW_NAME, NANVR_NAME};

pub enum CloseAction {
    Close,
    CloseWithRequest(ServerRequest),
}

pub struct NewVersionPopup {
    version: String,
    message: String,
    launcher_path: Option<PathBuf>,
}

impl NewVersionPopup {
    pub fn new(version: String, message: String) -> Self {
        let mut launcher_path = None;

        let layout = crate::get_filesystem_layout();
        if let Some(path) = layout.launcher_exe()
            && path.exists()
        {
            launcher_path = Some(path);
        }

        Self {
            version,
            message,
            launcher_path,
        }
    }

    pub fn ui(&self, context: &Context, shutdown_nanvr_cb: impl Fn()) -> Option<CloseAction> {
        let no_remind_button =
            ModalButton::Custom("Don't remind me again for this version".to_string());

        let result = gui_shared::modal(
            context,
            &format!("New {NANVR_NAME} version available"),
            Some(|ui: &mut Ui| {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);

                    ui.vertical(|ui| {
                        ui.heading(format!("{NANVR_NAME} {}", self.version));

                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 5.0;
                            ui.style_mut().spacing.button_padding = egui::vec2(10.0, 4.0);

                            ui.heading("You can download this version using the launcher:");

                            if let Some(path) = &self.launcher_path {
                                if ui.button("Open Launcher").clicked()
                                    && Command::new(path).spawn().is_ok()
                                {
                                    shutdown_nanvr_cb();
                                }
                            } else if ui.button("Download Launcher").clicked() {
                                let base_url =
                                    format!("https://github.com/{NANVR_GH_REPO_PATH}/releases/latest/download/");
                                let file = format!("{NANVR_LOW_NAME}_launcher_linux.tar.gz");

                                context.open_url(OpenUrl::new_tab(format!("{base_url}{file}")));
                            }
                        });

                        ui.add_space(10.0);

                        ui.label(&self.message);
                        ui.hyperlink_to(
                            "Releases page",
                            format!("https://github.com/{NANVR_GH_REPO_PATH}/releases"),
                        );
                    });

                    ui.add_space(10.0);
                });
            }),
            &[no_remind_button.clone(), ModalButton::Close],
            Some(490.0),
        );

        if let Some(button) = result {
            if button == no_remind_button {
                Some(CloseAction::CloseWithRequest(ServerRequest::SetValues(
                    vec![PathValuePair {
                        path: net_packets::parse_path(
                            "session_settings.extra.new_version_popup.content.hide_while_version",
                        ),
                        value: serde_json::Value::String(self.version.clone()),
                    }],
                )))
            } else {
                Some(CloseAction::Close)
            }
        } else {
            None
        }
    }
}
