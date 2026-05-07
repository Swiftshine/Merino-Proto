use crate::merino::level_editor::LevelEditor;

impl LevelEditor {
    pub fn show_canvas(&mut self, ui: &mut egui::Ui) {
        let desired_canvas_size = ui.available_size();
        let (rect, response) =
            ui.allocate_exact_size(desired_canvas_size, egui::Sense::click_and_drag());

        // update camera
        self.canvas_context.camera.update(ui.ctx(), &response);

        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, egui::Color32::BLACK);

        // edit objects
        self.edit_all_nodes(ui, rect);

        // process inputs
        self.handle_inputs(ui);

        // edit fields
        // can only edit the field of 1 selected object at a time
        if self.canvas_context.selected_node_paths.len() == 1 {
            let path = self.canvas_context.selected_node_paths[0].clone();
            self.edit_node_properties(ui, path);
        }
    }
}
