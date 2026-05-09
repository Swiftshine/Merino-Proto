use crate::merino::{
    game::mapbin::{MapDataNode, MapNodeType, NodeData, Vec2f},
    level_editor::{CanvasContext, CanvasTarget, EditorCommand, LevelEditor, NodePath},
};

const SMALL_SQUARE_SIZE: f32 = 0.5;
const SQUARE_SIZE: f32 = 2.0;

const SELECTION_HIGHLIGHT: egui::Color32 =
    egui::Color32::from_rgba_unmultiplied_const(0xFF, 0xFF, 0xFF, 0x10);

/// A list of objects to not display.
// const DO_NOT_DISPLAY_LIST: [&'static str; 1] = ["dummy_terrain"];

impl LevelEditor {
    pub fn edit_all_nodes(&mut self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let mut path = Vec::new();

        self.file_context.mapdata.root.edit(
            &mut self.canvas_context,
            ui,
            canvas_rect,
            &mut path,
            &mut self.editor_context.commands,
        );
    }
}

impl MapDataNode {
    fn edit(
        &mut self,
        context: &mut CanvasContext,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        commands: &mut Vec<EditorCommand>,
    ) {
        if !context.can_view(self.node_type) {
            return;
        }

        let do_edit = context.can_edit(self.node_type);

        match self.node_type {
            // MapNodeType::MapSet => {
            //     self.edit_mapset(context, ui, canvas_rect, current_path);
            // }
            MapNodeType::MapPolySet => {
                self.edit_mappolyset(
                    context,
                    ui,
                    canvas_rect,
                    current_path,
                    egui::Color32::WHITE,
                    commands,
                    do_edit,
                );
            }

            MapNodeType::MapObjSet => {
                self.edit_mapobjset(context, commands, ui, canvas_rect, current_path, do_edit);
            }

            MapNodeType::MapLocator => {
                self.edit_maplocator(context, ui, canvas_rect, current_path, do_edit);
            }
            _ => {}
        }

        for (branch, index, child) in self.all_children_mut() {
            current_path.push((branch, index));
            child.edit(context, ui, canvas_rect, current_path, commands);
            current_path.pop();
        }
    }

    // fn edit_mapset(
    //     &mut self,
    //     context: &mut CanvasContext,
    //     ui: &mut egui::Ui,
    //     canvas_rect: egui::Rect,
    //     current_path: &mut NodePath,
    // ) {
    //     let NodeData::MapSet {
    //         bounds_start,
    //         bounds_end,
    //         ..
    //     } = &mut self.node_data
    //     else {
    //         return;
    //     };

    //     let painter = ui.painter_at(canvas_rect);

    //     // draw bounds
    //     let start = canvas_rect.min + context.camera.convert_to_camera(bounds_start.into());
    //     let end = canvas_rect.min + context.camera.convert_to_camera(bounds_end.into());

    //     let square = egui::Rect::from_points(&[start, end]);

    //     let square_response = ui.interact(
    //         canvas_rect.intersect(square),
    //         egui::Id::new(&current_path),
    //         egui::Sense::click_and_drag(),
    //     );

    //     painter.rect_stroke(
    //         egui::Rect::from_points(&[start, end]),
    //         0.0,
    //         egui::Stroke::new(1.0, egui::Color32::WHITE),
    //         egui::StrokeKind::Middle,
    //     );

    //     if square_response.clicked_by(egui::PointerButton::Primary) {
    //         // selected, add to selected object indices
    //         context.selected_node_paths.push(current_path.clone());
    //     } else if square_response.dragged_by(egui::PointerButton::Primary) {
    //         let world_delta = square_response.drag_delta() / context.camera.zoom;

    //         bounds_start.x += world_delta.x;
    //         bounds_start.y -= world_delta.y;
    //         bounds_end.x += world_delta.x;
    //         bounds_end.y -= world_delta.y;
    //     }

    //     let selected = context.selected_node_paths.contains(&current_path);

    //     if selected {
    //         painter.rect_filled(square, 0.0, SELECTION_HIGHLIGHT);
    //     }
    // }

    fn edit_mappolyset(
        &mut self,
        context: &mut CanvasContext,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        mut color: egui::Color32,
        commands: &mut Vec<EditorCommand>,
        do_edit: bool,
    ) {
        /* draw */

        let NodeData::MapPolySet {
            start,
            end,
            collision_normal,
            ..
        } = &mut self.node_data
        else {
            return;
        };

        let painter = ui.painter_at(canvas_rect);

        let draw_start = canvas_rect.min + context.camera.convert_to_camera(start.into());
        let draw_end = canvas_rect.min + context.camera.convert_to_camera(end.into());

        painter.line_segment([draw_start, draw_end], egui::Stroke::new(1.0, color));

        /* edit */
        if !do_edit {
            return;
        }

        let start_rect = egui::Rect::from_center_size(
            egui::Pos2::new(draw_start.x, draw_start.y - SQUARE_SIZE * 2.0),
            egui::Vec2::splat(SMALL_SQUARE_SIZE * context.camera.zoom),
        );

        let end_rect = egui::Rect::from_center_size(
            egui::Pos2::new(draw_end.x, draw_end.y - SQUARE_SIZE * 2.0),
            egui::Vec2::splat(SMALL_SQUARE_SIZE * context.camera.zoom),
        );

        let start_resp = ui.interact(
            canvas_rect.intersect(start_rect),
            egui::Id::new(&current_path).with("start"),
            egui::Sense::click_and_drag(),
        );

        let end_resp = ui.interact(
            canvas_rect.intersect(end_rect),
            egui::Id::new(&current_path).with("end"),
            egui::Sense::click_and_drag(),
        );

        if context.selected_node_paths.contains(&current_path) {
            color = egui::Color32::RED;
        }

        painter.rect_filled(start_rect, 0.3, color);
        painter.rect_filled(end_rect, 0.3, color);

        let responses = [&start_resp, &end_resp];

        if responses
            .iter()
            .any(|resp| resp.clicked_by(egui::PointerButton::Primary))
        {
            if let Some(CanvasTarget::Search(parent_path)) = &context.target {
                commands.push(EditorCommand::move_node(
                    current_path.clone(),
                    parent_path.clone(),
                ));
            } else {
                context.selected_node_paths.push(current_path.clone());
            }
        }

        // dragging
        let mut dragged = false;
        if start_resp.dragged_by(egui::PointerButton::Primary) {
            let world_delta = start_resp.drag_delta() / context.camera.zoom;
            start.x += world_delta.x;
            start.y -= world_delta.y;
            dragged = true;
        }

        if end_resp.dragged_by(egui::PointerButton::Primary) {
            let world_delta = end_resp.drag_delta() / context.camera.zoom;
            end.x += world_delta.x;
            end.y -= world_delta.y;
            dragged = true;
        }

        if dragged {
            // recalculate collision vector
            let direction = (end.x - start.x, end.y - start.y);
            let magnitude = f32::sqrt(direction.0.powf(2.0) + direction.1.powf(2.0));
            let normalized = (direction.0 / magnitude, direction.1 / magnitude);

            collision_normal.x = -normalized.1;
            collision_normal.y = normalized.0;
        }
    }

    fn edit_mapobjset(
        &mut self,
        canvas_context: &mut CanvasContext,
        commands: &mut Vec<EditorCommand>,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        do_edit: bool,
    ) {
        let NodeData::MapObjSet { name, position, .. } = &mut self.node_data else {
            return;
        };

        // check if we should process this at all
        if !(canvas_context.display_dummy_terrain || name.as_str() != "dummy_terrain") {
            return;
        }

        let painter = ui.painter_at(canvas_rect);

        let screen_pos = canvas_rect.min
            + canvas_context
                .camera
                .convert_to_camera(Vec2f::from(*position).into());

        let square = egui::Rect::from_center_size(
            egui::Pos2::new(screen_pos.x, screen_pos.y - SQUARE_SIZE * 2.0),
            egui::Vec2::splat(SQUARE_SIZE * canvas_context.camera.zoom),
        );

        painter.rect_stroke(
            square,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::WHITE),
            egui::StrokeKind::Middle,
        );

        if !do_edit {
            return;
        }

        let response = ui.interact(
            canvas_rect.intersect(square),
            egui::Id::new(&current_path),
            egui::Sense::click_and_drag(),
        );

        if response.clicked_by(egui::PointerButton::Primary) {
            // check if a child is being searched for
            if let Some(CanvasTarget::Search(parent_path)) = &canvas_context.target {
                // todo! make all selected nodes a child of the new parent
                // but for now we're just going to work with clicking on this one
                commands.push(EditorCommand::move_node(
                    current_path.clone(),
                    parent_path.clone(),
                ));
            } else {
                // not being looked for, just select
                canvas_context
                    .selected_node_paths
                    .push(current_path.clone());
            }
        } else if response.dragged_by(egui::PointerButton::Primary) {
            let world_delta = response.drag_delta() / canvas_context.camera.zoom;
            position.x += world_delta.x;
            position.y -= world_delta.y;
        }

        let selected = canvas_context.selected_node_paths.contains(current_path);

        if response.hovered() || selected {
            // display name above if hovered
            painter.debug_text(
                square.center_top() - egui::Vec2::new(0.0, 5.0),
                egui::Align2::CENTER_BOTTOM,
                egui::Color32::WHITE,
                name,
            );
        }

        if selected {
            painter.rect_filled(square, 0.0, SELECTION_HIGHLIGHT);
        }
    }

    fn edit_maplocator(
        &mut self,
        context: &mut CanvasContext,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        do_edit: bool,
    ) {
        let NodeData::MapLocator { name, position, .. } = &mut self.node_data else {
            return;
        };

        let painter = ui.painter_at(canvas_rect);

        let screen_pos = canvas_rect.min
            + context
                .camera
                .convert_to_camera(Vec2f::from(*position).into());

        let square = egui::Rect::from_center_size(
            egui::Pos2::new(screen_pos.x, screen_pos.y - SQUARE_SIZE * 2.0),
            egui::Vec2::splat(SQUARE_SIZE * context.camera.zoom),
        );

        painter.rect_stroke(
            square,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
            egui::StrokeKind::Middle,
        );

        if !do_edit {
            return;
        }

        let response = ui.interact(
            canvas_rect.intersect(square),
            egui::Id::new(&current_path),
            egui::Sense::click_and_drag(),
        );

        if response.clicked_by(egui::PointerButton::Primary) {
            context.selected_node_paths.push(current_path.clone());
        } else if response.dragged_by(egui::PointerButton::Primary) {
            let world_delta = response.drag_delta() / context.camera.zoom;
            position.x += world_delta.x;
            position.y -= world_delta.y;
        }

        let selected = context.selected_node_paths.contains(current_path);

        if response.hovered() || selected {
            // display name above if hovered
            painter.debug_text(
                square.center_top() - egui::Vec2::new(0.0, 5.0),
                egui::Align2::CENTER_BOTTOM,
                egui::Color32::LIGHT_BLUE,
                name,
            );
        }

        if selected {
            painter.rect_filled(square, 0.0, SELECTION_HIGHLIGHT);
        }
    }
}
