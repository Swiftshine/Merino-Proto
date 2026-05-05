use std::path::PathBuf;

use crate::merino::game::mapbin::Mapbin;

mod le_io;

pub struct LevelEditor {
    // i/o
    file_open: bool,
    file_path: Option<PathBuf>,

    // files
    mapdata: Mapbin,
}

impl LevelEditor {
    pub fn new() -> Self {
        Self {
            file_open: false,
            file_path: None,
            mapdata: Mapbin::default(),
        }
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        // top panel
        egui::TopBottomPanel::top("le_top_panel").show(ui.ctx(), |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // file submenu
                if ui.button("Open").clicked() {
                    let _ = self.open_file(ui.ctx());
                }
            });
        });
    }
}
