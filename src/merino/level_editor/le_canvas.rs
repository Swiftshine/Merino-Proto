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
            if let Some(target) = &self.canvas_context.target {
                if target.is_add() {
                    Self::add_object(
                        ui,
                        painter,
                        &response,
                        &mut self.file_context,
                        &mut self.canvas_context,
                    );
                } else if target.is_search() {
                    // child movement is handled elsewhere
                    Self::draw_crosshair(painter, &response);
                }
            }

            // process inputs
            self.handle_mouse_inputs(ui);
            self.handle_keyboard_inputs(ui);
        }
    }

    pub fn draw_crosshair(painter: egui::Painter, response: &egui::Response) {
        if let Some(pointer_pos) = response.hover_pos() {
            // circle
            painter.circle_filled(pointer_pos, 1.0, egui::Color32::GRAY);
            let crosshair_size = 10.0;

            // horizontal line
            painter.line_segment(
                [
                    pointer_pos - egui::vec2(crosshair_size, 0.0),
                    pointer_pos + egui::vec2(crosshair_size, 0.0),
                ],
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            );

            // vertical line
            painter.line_segment(
                [
                    pointer_pos - egui::vec2(0.0, crosshair_size),
                    pointer_pos + egui::vec2(0.0, crosshair_size),
                ],
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            );
        }
    }
}
