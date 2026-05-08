use crate::merino::level_editor::LevelEditor;

impl LevelEditor {
    pub fn show_canvas(&mut self, ui: &mut egui::Ui) {
        let desired_canvas_size = ui.available_size();
        let (_, response) =
            ui.allocate_exact_size(desired_canvas_size, egui::Sense::click_and_drag());

        let rect = response.rect;

        // update camera
        self.canvas_context.camera.update(ui.ctx(), &response);

        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, egui::Color32::BLACK);

        // edit objects
        self.edit_all_nodes(ui, rect);

        if response.hovered() {
            // add objects
            if self.canvas_context.current_add_target.is_some() {
                Self::add_object(
                    ui,
                    painter,
                    &response,
                    &mut self.file_context,
                    &mut self.canvas_context,
                );
            }

            // process inputs
            self.handle_mouse_inputs(ui);
            self.handle_keyboard_inputs(ui);
        }
    }
}
