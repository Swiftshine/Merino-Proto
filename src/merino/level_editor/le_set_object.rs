use crate::merino::level_editor::LevelEditor;

impl LevelEditor {
    pub fn show_set_object(&mut self, ui: &mut egui::Ui) {
        if let Some(target) = &self.canvas_context.target
            && target.is_search()
        {
            ui.centered_and_justified(|ui| ui.label("Select a child node."));
        } else {
            ui.centered_and_justified(|ui| ui.label("No parent node selected."));
        }
    }
}
