use strum::IntoEnumIterator;

use crate::merino::{
    game::mapbin::{MapDataNode, NodeData, Vec3f},
    level_editor::{CanvasContext, CanvasTarget, FileContext, LevelEditor, NodeChildType},
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
                self.canvas_context.target = Some(CanvasTarget::new_to_root(child_type));
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
        let Some(target) = canvas_context.target.take() else {
            return;
        };

        let mut placed = false;

        if let Some(pointer_pos) = response.hover_pos() {
            // draw text
            painter.debug_text(
                pointer_pos - egui::Vec2::new(0.0, 10.0),
                egui::Align2::CENTER_BOTTOM,
                egui::Color32::WHITE,
                target.to_string(),
            );

            // draw crosshair
            Self::draw_crosshair(painter, response);
        }

        // we already know the response is hovered
        if ui.ctx().input(|i| i.pointer.primary_released())
            && let Some(pointer_pos) = response.hover_pos()
        {
            // offset to make it align with the mouse
            let local_pos = (pointer_pos - response.rect.min).to_pos2();
            let version = file_context.mapdata.version;

            match &target {
                CanvasTarget::NewToRoot(child_type) => {
                    Self::add_new_object_to_node(
                        version,
                        &mut file_context.mapdata.root,
                        *child_type,
                        local_pos,
                        canvas_context,
                    );
                    placed = true;
                }

                CanvasTarget::NewToNode(child_type, path) => {
                    if let Some(node) = file_context.mapdata.root.find_node_mut(path) {
                        Self::add_new_object_to_node(
                            version,
                            node,
                            *child_type,
                            local_pos,
                            canvas_context,
                        );
                        placed = true;
                    }
                }

                _ => unreachable!(),
            }
        }

        // put it back if still needed
        if placed {
            canvas_context.target = None;
        } else {
            canvas_context.target = Some(target);
        }
    }

    fn add_new_object_to_node(
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
