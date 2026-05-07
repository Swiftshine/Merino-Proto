use crate::merino::level_editor::LevelEditor;

impl LevelEditor {
    pub fn handle_keyboard_inputs(&mut self, ui: &mut egui::Ui) {
        let secondary_down = ui.input(|i| i.pointer.button_down(egui::PointerButton::Secondary));

        // pan reset handling
        if secondary_down && ui.input(|i| i.key_pressed(egui::Key::R)) {
            self.canvas_context.camera.reset();
        }

        // clear selections
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.canvas_context.selected_node_paths.clear();
        }
    }
}
