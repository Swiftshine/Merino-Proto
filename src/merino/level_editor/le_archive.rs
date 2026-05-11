use crate::merino::level_editor::LevelEditor;
use crate::merino::reader::read_level;

// const VALID_FILE_TYPES: [&'static str; 1] = [
//     "mapbin",
//     // i would include others here if this wasn't the prototype version
//     // "bson"
//     // "mappath"
// ];

impl LevelEditor {
    pub fn show_archive(&mut self, ui: &mut egui::Ui) {
        if self.file_context.archive_contents.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("Archive has no files.");
            });

            return;
        }

        for (name, contents) in self.file_context.archive_contents.iter() {
            let is_mapbin = name.ends_with(".mapbin");

            // allow the name to be selected if its what we're looking for
            // otherwise, display the name, but dont allow the user to click on it

            if is_mapbin {
                let is_selected = self.file_context.current_archive_file == Some(name.clone());

                if ui.selectable_label(is_selected, name).clicked() {
                    self.file_context.current_archive_file = Some(name.clone());

                    if let Ok(mapdata) = read_level(contents) {
                        self.file_context.mapdata = mapdata;
                        self.io_context.file_open = true;
                    }
                }
            } else {
                // non-clickable label
                ui.label(name);
            }
        }
    }
}
