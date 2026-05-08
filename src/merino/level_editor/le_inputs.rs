use crate::merino::level_editor::LevelEditor;

impl LevelEditor {
    pub fn handle_keyboard_inputs(&mut self, ui: &mut egui::Ui) {
        let secondary_down = ui.input(|i| i.pointer.button_down(egui::PointerButton::Secondary));

        // pan reset handling
        if secondary_down && ui.input(|i| i.key_pressed(egui::Key::R)) {
            self.canvas_context.camera.reset();
        }

        // clear selections
        let escape_pressed = ui.input(|i| i.key_pressed(egui::Key::Escape));

        if escape_pressed {
            // only escape from one action at a time
            if self.canvas_context.current_add_object.is_some() {
                self.canvas_context.current_add_object = None;
            } else {
                self.canvas_context.selected_node_paths.clear();
            }
        }
    }
}
