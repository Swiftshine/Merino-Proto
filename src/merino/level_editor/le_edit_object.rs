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
            ui,
            canvas_rect,
            &mut path,
            &mut self.editor_context.commands,
            &mut self.canvas_context,
        );
    }
}

impl MapDataNode {
    fn edit(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        commands: &mut Vec<EditorCommand>,
        canvas_context: &mut CanvasContext,
    ) {
        if !canvas_context.can_view(self.node_type) {
            return;
        }

        let do_edit = canvas_context.can_edit(self.node_type);

        match self.node_type {
            MapNodeType::MapSet => {
                self.edit_rect_node(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    egui::Color32::DARK_GREEN,
                );
            }

            MapNodeType::MapPolySet => {
                self.edit_mappolyset(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    egui::Color32::WHITE,
                );
            }

            MapNodeType::MapRect => {
                self.edit_rect_node(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    egui::Color32::from_rgb(0xC8, 0x74, 0xD9),
                );
            }

            MapNodeType::MapObjSet => {
                self.edit_point_node(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    egui::Color32::WHITE,
                    true,
                );
            }

            MapNodeType::MapItemSet => {
                self.edit_point_node(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    egui::Color32::GOLD,
                    false,
                );
            }

            MapNodeType::MapLocator => {
                self.edit_point_node(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    egui::Color32::LIGHT_BLUE,
                    false,
                );
            }

            MapNodeType::MapEnemySet => {
                self.edit_point_node(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    egui::Color32::LIGHT_RED,
                    false,
                );
            }

            MapNodeType::MapCircle => {
                self.edit_mapcircle(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    egui::Color32::LIGHT_GREEN,
                );
            }

            _ => {}
        }

        for (branch, index, child) in self.all_children_mut() {
            current_path.push((branch, index));

            child.edit(ui, canvas_rect, current_path, commands, canvas_context);

            current_path.pop();
        }
    }

    /* generic editors */

    fn edit_point_node(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        commands: &mut Vec<EditorCommand>,
        canvas_context: &mut CanvasContext,
        do_edit: bool,
        color: egui::Color32,
        allow_dummy_terrain_filter: bool,
    ) {
        let (name, position) = match &mut self.node_data {
            NodeData::MapObjSet { name, position, .. } => (name.as_str(), position),

            NodeData::MapItemSet { name, position, .. } => (name.as_str(), position),

            NodeData::MapEnemySet { name, position, .. } => (name.as_str(), position),

            _ => return,
        };

        if allow_dummy_terrain_filter
            && !(canvas_context.display_dummy_terrain || name != "dummy_terrain")
        {
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
            egui::Stroke::new(1.0, color),
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
            if let Some(CanvasTarget::Search(parent_path)) = &canvas_context.target {
                commands.push(EditorCommand::move_node(
                    current_path.clone(),
                    parent_path.clone(),
                ));
            } else {
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
            let text_pos = square.center_top() - egui::Vec2::new(0.0, 5.0);

            let galley =
                painter.layout_no_wrap(name.to_string(), egui::FontId::monospace(12.0), color);

            let text_rect = egui::Align2::CENTER_BOTTOM.anchor_size(text_pos, galley.size());

            painter.rect_filled(
                text_rect.expand(2.0),
                2.0,
                egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100),
            );

            painter.galley(text_rect.min, galley, color);
        }

        if selected {
            painter.rect_filled(square, 0.0, SELECTION_HIGHLIGHT);
        }
    }

    fn edit_rect_node(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        commands: &mut Vec<EditorCommand>,
        canvas_context: &mut CanvasContext,
        do_edit: bool,
        mut color: egui::Color32,
    ) {
        let Some((bounds_start, bounds_end)) = self.rect_bounds_mut() else {
            return;
        };

        let painter = ui.painter_at(canvas_rect);

        let start = canvas_rect.min + canvas_context.camera.convert_to_camera(bounds_start.into());

        let end = canvas_rect.min + canvas_context.camera.convert_to_camera(bounds_end.into());

        let rect = egui::Rect::from_two_pos(start, end);

        painter.rect_stroke(
            rect,
            0.0,
            egui::Stroke::new(1.0, color),
            egui::StrokeKind::Middle,
        );

        if !do_edit {
            return;
        }

        let start_screen =
            canvas_rect.min + canvas_context.camera.convert_to_camera(bounds_start.into());

        let end_screen =
            canvas_rect.min + canvas_context.camera.convert_to_camera(bounds_end.into());

        let start_handle = egui::Rect::from_center_size(
            start_screen,
            egui::Vec2::splat(SMALL_SQUARE_SIZE * canvas_context.camera.zoom),
        );

        let end_handle = egui::Rect::from_center_size(
            end_screen,
            egui::Vec2::splat(SMALL_SQUARE_SIZE * canvas_context.camera.zoom),
        );

        let start_resp = ui.interact(
            canvas_rect.intersect(start_handle),
            egui::Id::new(&current_path).with("start"),
            egui::Sense::click_and_drag(),
        );

        let end_resp = ui.interact(
            canvas_rect.intersect(end_handle),
            egui::Id::new(&current_path).with("end"),
            egui::Sense::click_and_drag(),
        );

        if canvas_context.selected_node_paths.contains(&current_path) {
            color = egui::Color32::RED;
        }

        painter.rect_filled(start_handle, 0.3, color);
        painter.rect_filled(end_handle, 0.3, color);

        let responses = [&start_resp, &end_resp];

        if responses
            .iter()
            .any(|r| r.clicked_by(egui::PointerButton::Primary))
        {
            if let Some(CanvasTarget::Search(parent_path)) = &canvas_context.target {
                commands.push(EditorCommand::move_node(
                    current_path.clone(),
                    parent_path.clone(),
                ));
            } else {
                canvas_context
                    .selected_node_paths
                    .push(current_path.clone());
            }
        }

        if start_resp.dragged_by(egui::PointerButton::Primary) {
            let world_delta = start_resp.drag_delta() / canvas_context.camera.zoom;
            bounds_start.x += world_delta.x;
            bounds_start.y -= world_delta.y;
        }

        if end_resp.dragged_by(egui::PointerButton::Primary) {
            let world_delta = end_resp.drag_delta() / canvas_context.camera.zoom;
            bounds_end.x += world_delta.x;
            bounds_end.y -= world_delta.y;
        }

        let selected = canvas_context.selected_node_paths.contains(current_path);

        if selected {
            painter.rect_filled(rect, 0.0, SELECTION_HIGHLIGHT);
        }
    }

    /* specific editors */

    fn edit_mappolyset(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        commands: &mut Vec<EditorCommand>,
        canvas_context: &mut CanvasContext,
        do_edit: bool,
        mut color: egui::Color32,
    ) {
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

        let draw_start = canvas_rect.min + canvas_context.camera.convert_to_camera(start.into());

        let draw_end = canvas_rect.min + canvas_context.camera.convert_to_camera(end.into());

        painter.line_segment([draw_start, draw_end], egui::Stroke::new(1.0, color));

        if !do_edit {
            return;
        }

        let start_rect = egui::Rect::from_center_size(
            egui::Pos2::new(draw_start.x, draw_start.y - SQUARE_SIZE * 2.0),
            egui::Vec2::splat(SMALL_SQUARE_SIZE * canvas_context.camera.zoom),
        );

        let end_rect = egui::Rect::from_center_size(
            egui::Pos2::new(draw_end.x, draw_end.y - SQUARE_SIZE * 2.0),
            egui::Vec2::splat(SMALL_SQUARE_SIZE * canvas_context.camera.zoom),
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

        if canvas_context.selected_node_paths.contains(&current_path) {
            color = egui::Color32::LIGHT_RED;
        }

        painter.rect_filled(start_rect, 0.3, color);
        painter.rect_filled(end_rect, 0.3, color);

        let responses = [&start_resp, &end_resp];

        if responses
            .iter()
            .any(|resp| resp.clicked_by(egui::PointerButton::Primary))
        {
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
        }

        let mut dragged = false;

        if start_resp.dragged_by(egui::PointerButton::Primary) {
            let world_delta = start_resp.drag_delta() / canvas_context.camera.zoom;

            start.x += world_delta.x;
            start.y -= world_delta.y;

            dragged = true;
        }

        if end_resp.dragged_by(egui::PointerButton::Primary) {
            let world_delta = end_resp.drag_delta() / canvas_context.camera.zoom;

            end.x += world_delta.x;
            end.y -= world_delta.y;

            dragged = true;
        }

        if dragged {
            let direction = (end.x - start.x, end.y - start.y);

            let magnitude = f32::sqrt(direction.0.powf(2.0) + direction.1.powf(2.0));

            let normalized = (direction.0 / magnitude, direction.1 / magnitude);

            collision_normal.x = -normalized.1;
            collision_normal.y = normalized.0;
        }
    }

    fn edit_mapcircle(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        commands: &mut Vec<EditorCommand>,
        canvas_context: &mut CanvasContext,
        do_edit: bool,
        mut color: egui::Color32,
    ) {
        let NodeData::MapCircle {
            position, radius, ..
        } = &mut self.node_data
        else {
            return;
        };

        let painter = ui.painter_at(canvas_rect);

        let center = canvas_rect.min + canvas_context.camera.convert_to_camera((*position).into());

        let screen_radius = *radius * canvas_context.camera.zoom;

        painter.circle_stroke(center, screen_radius, egui::Stroke::new(1.0, color));

        if !do_edit {
            return;
        }

        let center_handle = egui::Rect::from_center_size(
            center,
            egui::Vec2::splat(SMALL_SQUARE_SIZE * canvas_context.camera.zoom),
        );

        let radius_handle_pos = egui::Pos2::new(center.x + screen_radius, center.y);

        let radius_handle = egui::Rect::from_center_size(
            radius_handle_pos,
            egui::Vec2::splat(SMALL_SQUARE_SIZE * canvas_context.camera.zoom),
        );

        let center_resp = ui.interact(
            canvas_rect.intersect(center_handle),
            egui::Id::new(&current_path).with("center"),
            egui::Sense::click_and_drag(),
        );

        let radius_resp = ui.interact(
            canvas_rect.intersect(radius_handle),
            egui::Id::new(&current_path).with("radius"),
            egui::Sense::click_and_drag(),
        );

        if canvas_context.selected_node_paths.contains(current_path) {
            color = egui::Color32::RED;
        }

        painter.rect_filled(center_handle, 0.0, color);
        painter.rect_filled(radius_handle, 0.0, color);

        if center_resp.clicked() || radius_resp.clicked() {
            if let Some(CanvasTarget::Search(parent_path)) = &canvas_context.target {
                commands.push(EditorCommand::move_node(
                    current_path.clone(),
                    parent_path.clone(),
                ));
            } else {
                canvas_context
                    .selected_node_paths
                    .push(current_path.clone());
            }
        }

        if center_resp.dragged_by(egui::PointerButton::Primary) {
            let world_delta = center_resp.drag_delta() / canvas_context.camera.zoom;

            position.x += world_delta.x;
            position.y -= world_delta.y;
        }

        if radius_resp.dragged_by(egui::PointerButton::Primary) {
            let pointer = ui.input(|i| i.pointer.hover_pos());

            if let Some(pointer) = pointer {
                let dx = pointer.x - center.x;
                let dy = pointer.y - center.y;

                *radius = (dx * dx + dy * dy).sqrt() / canvas_context.camera.zoom;
            }
        }

        if canvas_context.selected_node_paths.contains(current_path) {
            painter.circle_filled(center, screen_radius, SELECTION_HIGHLIGHT);
        }
    }

    /* helpers */

    fn rect_bounds_mut(&mut self) -> Option<(&mut Vec2f, &mut Vec2f)> {
        match &mut self.node_data {
            NodeData::MapRect {
                bounds_start,
                bounds_end,
                ..
            } => Some((bounds_start, bounds_end)),

            NodeData::MapSet {
                bounds_start,
                bounds_end,
                ..
            } => Some((bounds_start, bounds_end)),

            _ => None,
        }
    }
}
