use crate::merino::{
    game::mapbin::{MapDataNode, MapNodeType, NodeData, Vec2f},
    level_editor::{CanvasContext, CanvasTarget, EditorCommand, LevelEditor, NodePath},
};

const SQUARE_SIZE: f32 = 2.0;

const SELECTION_HIGHLIGHT: egui::Color32 =
    egui::Color32::from_rgba_unmultiplied_const(0xFF, 0xFF, 0xFF, 0x10);

/// A list of objects to not display.
// const DO_NOT_DISPLAY_LIST: [&'static str; 1] = ["dummy_terrain"];

impl LevelEditor {
    pub fn edit_all_nodes(&mut self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let mut path = Vec::new();
        let mut commands = Vec::new();

        self.file_context.mapdata.root.edit(
            &mut self.canvas_context,
            ui,
            canvas_rect,
            &mut path,
            &mut commands,
        );

        for command in commands {
            match command {
                EditorCommand::MoveNode { child, new_parent } => {
                    self.file_context.move_node(&child, &new_parent);
                }
            }
        }
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
        match self.node_type {
            // MapNodeType::MapSet => {
            //     self.edit_mapset(context, ui, canvas_rect, current_path);
            // }
            MapNodeType::MapPolySet => {
                self.edit_mappolyset(context, ui, canvas_rect, current_path);
            }

            MapNodeType::MapObjSet => {
                self.edit_mapobjset(context, commands, ui, canvas_rect, current_path);
            }

            MapNodeType::MapLocator => {
                self.edit_maplocator(context, ui, canvas_rect, current_path);
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
        _current_path: &mut NodePath,
    ) {
        let NodeData::MapPolySet { start, end, .. } = &mut self.node_data else {
            return;
        };

        let painter = ui.painter_at(canvas_rect);

        let draw_start = canvas_rect.min + context.camera.convert_to_camera(start.into());
        let draw_end = canvas_rect.min + context.camera.convert_to_camera(end.into());

        painter.line_segment(
            [draw_start, draw_end],
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );
    }

    fn edit_mapobjset(
        &mut self,
        canvas_context: &mut CanvasContext,
        commands: &mut Vec<EditorCommand>,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
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

        let response = ui.interact(
            canvas_rect.intersect(square),
            egui::Id::new(&current_path),
            egui::Sense::click_and_drag(),
        );

        painter.rect_stroke(
            square,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::WHITE),
            egui::StrokeKind::Middle,
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

        let response = ui.interact(
            canvas_rect.intersect(square),
            egui::Id::new(&current_path),
            egui::Sense::click_and_drag(),
        );

        painter.rect_stroke(
            square,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
            egui::StrokeKind::Middle,
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
