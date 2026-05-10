use crate::merino::{
    game::mapbin::{MapDataNode, MapNodeType, NodeData, Params, Vec2f, Vec3f},
    level_editor::{CanvasContext, CanvasTarget, EditorCommand, LevelEditor, NodePath},
};

const SMALL_SQUARE_SIZE: f32 = 0.5;
const SQUARE_SIZE: f32 = 2.0;

pub const SELECTION_HIGHLIGHT: egui::Color32 =
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
        if canvas_context.can_view(self.node_type) {
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

                MapNodeType::MapPath => {
                    self.edit_mappath(
                        ui,
                        canvas_rect,
                        current_path,
                        commands,
                        canvas_context,
                        do_edit,
                        egui::Color32::BLUE,
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

                MapNodeType::MapTerrain => {
                    self.edit_point_node(
                        ui,
                        canvas_rect,
                        current_path,
                        commands,
                        canvas_context,
                        do_edit,
                        egui::Color32::from_rgb(120, 220, 120),
                        false,
                    );
                }
            }
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
        match &mut self.node_data {
            NodeData::MapObjSet {
                name,
                position,
                params,
                ..
            } => {
                Self::draw_point_node_with_params(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    color,
                    allow_dummy_terrain_filter,
                    self.node_type,
                    name.as_str(),
                    position,
                    params,
                );
            }

            NodeData::MapItemSet {
                name,
                position,
                params,
                ..
            } => {
                Self::draw_point_node_with_params(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    color,
                    allow_dummy_terrain_filter,
                    self.node_type,
                    name.as_str(),
                    position,
                    params,
                );
            }

            NodeData::MapEnemySet {
                name,
                position,
                params,
                ..
            } => {
                Self::draw_point_node_with_params(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    color,
                    allow_dummy_terrain_filter,
                    self.node_type,
                    name.as_str(),
                    position,
                    params,
                );
            }

            NodeData::MapLocator {
                name,
                position,
                params,
            } => {
                Self::draw_point_node_with_params(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    color,
                    allow_dummy_terrain_filter,
                    self.node_type,
                    name.as_str(),
                    position,
                    params,
                );
            }

            NodeData::MapTerrain {
                collision_type,
                position,
                params,
                ..
            } => {
                Self::draw_point_node_with_params(
                    ui,
                    canvas_rect,
                    current_path,
                    commands,
                    canvas_context,
                    do_edit,
                    color,
                    allow_dummy_terrain_filter,
                    self.node_type,
                    collision_type.as_str(),
                    position,
                    params,
                );
            }

            _ => {}
        }
    }

    fn draw_point_node_with_params<const N: usize>(
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        commands: &mut Vec<EditorCommand>,
        canvas_context: &mut CanvasContext,
        do_edit: bool,
        color: egui::Color32,
        allow_dummy_terrain_filter: bool,
        node_type: MapNodeType,
        name: &str,
        position: &mut Vec3f,
        params: &Params<N>,
    ) {
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

        let square = Self::make_handle_rect(
            egui::Pos2::new(screen_pos.x, screen_pos.y - SQUARE_SIZE * 2.0),
            SQUARE_SIZE,
            canvas_context,
        );

        let mut has_image = false;
        if let Some((tex, rotation)) =
            canvas_context
                .image_bank
                .resolve_image_for_node(ui.ctx(), node_type, name, params)
        {
            draw_rotated_image(&painter, tex.id(), square, rotation, egui::Color32::WHITE);
            has_image = true;
        }

        if !(has_image && !canvas_context.display_squares_for_images) {
            painter.rect_stroke(
                square,
                0.0,
                egui::Stroke::new(1.0, color),
                egui::StrokeKind::Middle,
            );
        }

        if !do_edit {
            return;
        }

        let response = ui.interact(
            canvas_rect.intersect(square),
            egui::Id::new(&current_path),
            egui::Sense::click_and_drag(),
        );

        // clicked
        Self::handle_selection(
            current_path,
            response.clicked_by(egui::PointerButton::Primary),
            ui.input(|i| i.modifiers.shift),
            commands,
            canvas_context,
        );

        // marquee
        if canvas_context.rect_in_marquee(square) {
            commands.push(EditorCommand::add_to_selection(current_path.clone()));
        }

        if response.dragged_by(egui::PointerButton::Primary) {
            let mut pos2 = Vec2f::from(*position);

            Self::drag_vec2f(&mut pos2, &response, canvas_context);

            position.x = pos2.x;
            position.y = pos2.y;
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

        let start_handle = Self::make_handle_rect(start_screen, SMALL_SQUARE_SIZE, canvas_context);

        let end_handle = Self::make_handle_rect(end_screen, SMALL_SQUARE_SIZE, canvas_context);

        let start_resp =
            Self::interact_handle(ui, canvas_rect, start_handle, current_path, "start");

        let end_resp = Self::interact_handle(ui, canvas_rect, end_handle, current_path, "end");

        if canvas_context.selected_node_paths.contains(&current_path) {
            color = egui::Color32::LIGHT_RED;
        }

        painter.rect_filled(start_handle, 0.3, color);
        painter.rect_filled(end_handle, 0.3, color);

        let responses = [&start_resp, &end_resp];

        // clicked
        Self::handle_selection(
            current_path,
            responses.iter().any(|r| r.clicked()),
            ui.input(|i| i.modifiers.shift),
            commands,
            canvas_context,
        );

        // marquee
        if canvas_context.rect_in_marquee(rect) {
            commands.push(EditorCommand::add_to_selection(current_path.clone()));
        }

        if start_resp.dragged_by(egui::PointerButton::Primary) {
            Self::drag_vec2f(bounds_start, &start_resp, canvas_context);
        }

        if end_resp.dragged_by(egui::PointerButton::Primary) {
            Self::drag_vec2f(bounds_end, &end_resp, canvas_context);
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

        let start_rect = Self::make_handle_rect(draw_start, SMALL_SQUARE_SIZE, canvas_context);

        let end_rect = Self::make_handle_rect(draw_end, SMALL_SQUARE_SIZE, canvas_context);

        let start_resp = Self::interact_handle(ui, canvas_rect, start_rect, current_path, "start");

        let end_resp = Self::interact_handle(ui, canvas_rect, end_rect, current_path, "end");

        if canvas_context.selected_node_paths.contains(&current_path) {
            color = egui::Color32::LIGHT_RED;
        }

        painter.rect_filled(start_rect, 0.3, color);
        painter.rect_filled(end_rect, 0.3, color);

        let responses = [&start_resp, &end_resp];

        // clicked
        Self::handle_selection(
            current_path,
            responses.iter().any(|r| r.clicked()),
            ui.input(|i| i.modifiers.shift),
            commands,
            canvas_context,
        );

        // marquee
        if [&start_rect, &end_rect]
            .iter()
            .any(|rect| canvas_context.rect_in_marquee(**rect))
        {
            commands.push(EditorCommand::add_to_selection(current_path.clone()));
        }

        let mut dragged = false;

        if start_resp.dragged_by(egui::PointerButton::Primary) {
            Self::drag_vec2f(start, &start_resp, canvas_context);
            dragged = true;
        }

        if end_resp.dragged_by(egui::PointerButton::Primary) {
            Self::drag_vec2f(end, &end_resp, canvas_context);
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

    fn edit_mappath(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        commands: &mut Vec<EditorCommand>,
        canvas_context: &mut CanvasContext,
        do_edit: bool,
        mut color: egui::Color32,
    ) {
        let NodeData::MapPath { points, .. } = &mut self.node_data else {
            return;
        };

        if points.is_empty() {
            return;
        }

        let painter = ui.painter_at(canvas_rect);

        let screen_points: Vec<egui::Pos2> = points
            .iter()
            .map(|point| canvas_rect.min + canvas_context.camera.convert_to_camera((*point).into()))
            .collect();

        // draw lines
        for window in screen_points.windows(2) {
            painter.line_segment([window[0], window[1]], egui::Stroke::new(1.0, color));
        }

        let selected = Self::is_selected(current_path, canvas_context);

        if selected {
            color = egui::Color32::LIGHT_RED;
        }

        if !do_edit {
            return;
        }

        let mut any_clicked = false;

        let mut in_marquee = false;
        for (index, point) in points.iter_mut().enumerate() {
            let screen_point = screen_points[index];

            // marquee
            if canvas_context.point_in_marquee(screen_point) && !in_marquee {
                in_marquee = true;
                commands.push(EditorCommand::add_to_selection(current_path.clone()));
            }

            let handle_rect =
                Self::make_handle_rect(screen_point, SMALL_SQUARE_SIZE, canvas_context);

            let response = ui.interact(
                canvas_rect.intersect(handle_rect),
                egui::Id::new(&mut *current_path).with(index),
                egui::Sense::click_and_drag(),
            );

            if response.clicked() {
                any_clicked = true;
            }

            if response.dragged_by(egui::PointerButton::Primary) {
                Self::drag_vec2f(point, &response, canvas_context);
            }

            painter.rect_filled(handle_rect, 0.0, color);

            if selected {
                painter.rect_filled(handle_rect, 0.0, SELECTION_HIGHLIGHT);
            }
        }

        Self::handle_selection(
            current_path,
            any_clicked,
            ui.input(|i| i.modifiers.shift),
            commands,
            canvas_context,
        );
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
            color = egui::Color32::LIGHT_RED;
        }

        painter.rect_filled(center_handle, 0.0, color);
        painter.rect_filled(radius_handle, 0.0, color);

        // clicked
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

        // marquee
        // only selecting the center
        if canvas_context.point_in_marquee(center) {
            commands.push(EditorCommand::add_to_selection(current_path.clone()));
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

    fn is_selected(current_path: &NodePath, canvas_context: &CanvasContext) -> bool {
        canvas_context.selected_node_paths.contains(current_path)
    }

    fn handle_selection(
        current_path: &NodePath,
        response_clicked: bool,
        shift_held: bool,
        commands: &mut Vec<EditorCommand>,
        canvas_context: &mut CanvasContext,
    ) {
        if !response_clicked {
            return;
        }

        if let Some(CanvasTarget::Search(parent_path)) = &canvas_context.target {
            commands.push(EditorCommand::move_node(
                current_path.clone(),
                parent_path.clone(),
            ));
        } else {
            // not being looked for, just select
            if shift_held {
                // additive
                commands.push(EditorCommand::add_to_selection(current_path.clone()));
            } else {
                // replace
                commands.push(EditorCommand::select_node(current_path.clone()));
            }
        }
    }

    fn drag_world_delta(response: &egui::Response, canvas_context: &CanvasContext) -> egui::Vec2 {
        response.drag_delta() / canvas_context.camera.zoom
    }

    fn drag_vec2f(value: &mut Vec2f, response: &egui::Response, canvas_context: &CanvasContext) {
        let world_delta = Self::drag_world_delta(response, canvas_context);

        value.x += world_delta.x;
        value.y -= world_delta.y;
    }

    fn make_handle_rect(
        center: egui::Pos2,
        size: f32,
        canvas_context: &CanvasContext,
    ) -> egui::Rect {
        egui::Rect::from_center_size(center, egui::Vec2::splat(size * canvas_context.camera.zoom))
    }

    fn interact_handle(
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        rect: egui::Rect,
        current_path: &NodePath,
        suffix: &'static str,
    ) -> egui::Response {
        ui.interact(
            canvas_rect.intersect(rect),
            egui::Id::new(current_path).with(suffix),
            egui::Sense::click_and_drag(),
        )
    }
}

fn draw_rotated_image(
    painter: &egui::Painter,
    texture_id: egui::TextureId,
    rect: egui::Rect,
    rotation_degrees: f32,
    tint: egui::Color32,
) {
    use egui::{
        Pos2,
        epaint::{Mesh, Vertex},
    };

    let mut mesh = Mesh::with_texture(texture_id);

    let center = rect.center();

    let rotation = egui::emath::Rot2::from_angle(-rotation_degrees.to_radians());

    let rotate = |p: Pos2| center + rotation * (p - center);

    let p0 = rotate(rect.left_top());
    let p1 = rotate(rect.right_top());
    let p2 = rotate(rect.right_bottom());
    let p3 = rotate(rect.left_bottom());

    let uv0 = Pos2::new(0.0, 0.0);
    let uv1 = Pos2::new(1.0, 0.0);
    let uv2 = Pos2::new(1.0, 1.0);
    let uv3 = Pos2::new(0.0, 1.0);

    let base = mesh.vertices.len() as u32;

    mesh.vertices.extend_from_slice(&[
        Vertex {
            pos: p0,
            uv: uv0,
            color: tint,
        },
        Vertex {
            pos: p1,
            uv: uv1,
            color: tint,
        },
        Vertex {
            pos: p2,
            uv: uv2,
            color: tint,
        },
        Vertex {
            pos: p3,
            uv: uv3,
            color: tint,
        },
    ]);

    mesh.indices
        .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);

    painter.add(egui::Shape::mesh(mesh));
}
