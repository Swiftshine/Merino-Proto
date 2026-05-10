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

    pub fn show_canvas_settings(&mut self, ui: &mut egui::Ui) {
        // visibility/editability settings
        ui.label(
            egui::RichText::new("Visibility/Editability Settings")
                .strong()
                .underline(),
        )
        .on_hover_text(
            "Disabling any of these values will deselect every node of the corresponding type.",
        );
        let mut changed = false;
        for (node_type, settings) in self.canvas_context.node_edit_settings.iter_mut() {
            ui.horizontal(|ui| {
                ui.label(node_type.to_string());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        changed |= ui
                            .add_enabled(
                                settings.visible,
                                egui::Checkbox::new(&mut settings.editable, "Editable"),
                            )
                            .changed();
                        changed |= ui.checkbox(&mut settings.visible, "Visible").changed();
                    });
                });
            });
        }

        if changed {
            self.canvas_context.prune_invalid_selections();
        }

        ui.label(egui::RichText::new("Other Canvas settings").strong());

        ui.checkbox(
            &mut self.canvas_context.display_dummy_terrain,
            "Display \"dummy_terrain\"",
        );
        ui.checkbox(
            &mut self.canvas_context.display_squares_for_images,
            "Display squares for object images",
        );
    }
}
