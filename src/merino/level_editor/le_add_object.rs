use strum::IntoEnumIterator;

use crate::merino::{
    game::mapbin::{MapDataNode, NodeData, Vec3f},
    level_editor::{AddTarget, CanvasContext, FileContext, LevelEditor, NodeChildType},
};

impl LevelEditor {
    pub fn show_add_object(&mut self, ui: &mut egui::Ui) {
        // todo! search for object in database
        // show both the canon name and the display name

        // add object
        ui.label(egui::RichText::new("Add to Root Node").strong());
        for child_type in NodeChildType::iter() {
            if ui
                .add_sized(
                    [ui.available_width(), 30.0],
                    egui::Button::new(child_type.to_string()),
                )
                .clicked()
            {
                self.canvas_context.current_add_target = Some(AddTarget::root(child_type));
            }
        }
    }

    pub fn add_object(
        ui: &mut egui::Ui,
        painter: egui::Painter,
        response: &egui::Response,
        file_context: &mut FileContext,
        canvas_context: &mut CanvasContext,
    ) {
        // take the target
        let Some(add_target) = canvas_context.current_add_target.take() else {
            return;
        };

        let mut placed = false;

        if let Some(pointer_pos) = response.hover_pos() {
            // draw text
            painter.debug_text(
                pointer_pos - egui::Vec2::new(0.0, 10.0),
                egui::Align2::CENTER_BOTTOM,
                egui::Color32::WHITE,
                add_target.to_string(),
            );

            // draw crosshair
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

        // we already know the response is hovered
        if ui.ctx().input(|i| i.pointer.primary_released())
            && let Some(pointer_pos) = response.hover_pos()
        {
            // offset to make it align with the mouse
            let local_pos = (pointer_pos - response.rect.min).to_pos2();
            let version = file_context.mapdata.version;

            match &add_target {
                AddTarget::ToRoot(child_type) => {
                    Self::add_object_to_node(
                        version,
                        &mut file_context.mapdata.root,
                        *child_type,
                        local_pos,
                        canvas_context,
                    );
                    placed = true;
                }

                AddTarget::ToNode(child_type, path) => {
                    if let Some(node) = file_context.find_node_mut(path) {
                        Self::add_object_to_node(
                            version,
                            node,
                            *child_type,
                            local_pos,
                            canvas_context,
                        );
                        placed = true;
                    }
                }
            }
        }

        // put it back if still needed
        if placed {
            canvas_context.current_add_target = None;
        } else {
            canvas_context.current_add_target = Some(add_target);
        }
    }

    fn add_object_to_node(
        version: f32,
        parent_node: &mut MapDataNode,
        child_type: NodeChildType,
        pointer_pos: egui::Pos2,
        canvas_context: &CanvasContext,
    ) {
        match child_type {
            NodeChildType::MapObjSet => {
                let pos = canvas_context
                    .camera
                    .convert_from_camera(pointer_pos.to_vec2());

                let mut node_data = NodeData::default_map_obj_set(version);

                if let NodeData::MapObjSet { position, .. } = &mut node_data {
                    *position = pos.into();
                };

                let node = MapDataNode {
                    node_type: child_type.into(),
                    node_data,
                    ..Default::default()
                };

                parent_node
                    .children_mapobjset
                    .get_or_insert_with(Vec::new)
                    .push(node);
            }

            _ => {
                todo!()
            }
        }
    }
}

impl From<egui::Vec2> for Vec3f {
    fn from(value: egui::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: 0.0,
        }
    }
}
