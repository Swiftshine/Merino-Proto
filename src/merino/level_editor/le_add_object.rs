use strum::IntoEnumIterator;

use crate::merino::{
    game::mapbin::{MapDataNode, NodeData, Vec2f, Vec3f},
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
            NodeChildType::MapPolySet => {
                Self::add_new_polyset_node(parent_node, pointer_pos, canvas_context);
            }
            NodeChildType::MapObjSet
            | NodeChildType::MapItemSet
            | NodeChildType::MapEnemySet
            | NodeChildType::MapLocator
            | NodeChildType::MapTerrain => {
                Self::add_new_point_node(
                    version,
                    parent_node,
                    child_type,
                    pointer_pos,
                    canvas_context,
                );
            }

            NodeChildType::MapPath => {
                Self::add_new_path_node(parent_node, pointer_pos, canvas_context);
            }

            NodeChildType::MapRect => {
                Self::add_new_rect_node(parent_node, pointer_pos, canvas_context);
            }

            NodeChildType::MapCircle => {
                Self::add_new_circle_node(parent_node, pointer_pos, canvas_context);
            }
        }
    }

    /* generic types */

    fn add_new_point_node(
        version: f32,
        parent_node: &mut MapDataNode,
        child_type: NodeChildType,
        pointer_pos: egui::Pos2,
        canvas_context: &CanvasContext,
    ) {
        let pos = canvas_context
            .camera
            .convert_from_camera(pointer_pos.to_vec2());

        let mut node_data = match child_type {
            NodeChildType::MapObjSet => NodeData::default_mapobjset(version),
            NodeChildType::MapItemSet => NodeData::default_mapitemset(version),
            NodeChildType::MapEnemySet => NodeData::default_mapenemyset(version),
            NodeChildType::MapLocator => NodeData::default_maplocator(),
            NodeChildType::MapTerrain => NodeData::default_mapterrain(version),
            _ => return,
        };

        let position = match &mut node_data {
            NodeData::MapObjSet { position, .. } => position,
            NodeData::MapItemSet { position, .. } => position,
            NodeData::MapEnemySet { position, .. } => position,
            NodeData::MapLocator { position, .. } => position,
            NodeData::MapTerrain { position, .. } => position,
            _ => return,
        };

        *position = pos.into();

        Self::push_child_node(parent_node, child_type, node_data);
    }

    /* specific types */

    fn add_new_polyset_node(
        parent_node: &mut MapDataNode,
        pointer_pos: egui::Pos2,
        canvas_context: &CanvasContext,
    ) {
        let pos = canvas_context
            .camera
            .convert_from_camera(pointer_pos.to_vec2());

        let mut node_data = NodeData::default_mappolyset();

        let len = egui::Vec2::new(4.0, 0.0);

        if let NodeData::MapPolySet { start, end, .. } = &mut node_data {
            *start = pos.into();
            *end = (pos + len).into();
        }

        Self::push_child_node(parent_node, NodeChildType::MapPolySet, node_data);
    }

    fn add_new_path_node(
        parent_node: &mut MapDataNode,
        pointer_pos: egui::Pos2,
        canvas_context: &CanvasContext,
    ) {
        let pos = canvas_context
            .camera
            .convert_from_camera(pointer_pos.to_vec2());

        let mut node_data = NodeData::default_mappath();

        let len = egui::Vec2::new(4.0, 0.0);

        if let NodeData::MapPath { points, .. } = &mut node_data {
            points.push(pos.into());
            points.push((pos + len).into())
        }

        Self::push_child_node(parent_node, NodeChildType::MapPath, node_data);
    }

    fn add_new_rect_node(
        parent_node: &mut MapDataNode,
        pointer_pos: egui::Pos2,
        canvas_context: &CanvasContext,
    ) {
        let pos = canvas_context
            .camera
            .convert_from_camera(pointer_pos.to_vec2());

        let mut node_data = NodeData::default_maprect();

        let size = egui::Vec2::splat(4.0);

        match &mut node_data {
            NodeData::MapRect {
                bounds_start,
                bounds_end,
                ..
            } => {
                *bounds_start = pos.into();
                *bounds_end = (pos + size).into();
            }
            _ => return,
        }

        Self::push_child_node(parent_node, NodeChildType::MapRect, node_data);
    }

    fn add_new_circle_node(
        parent_node: &mut MapDataNode,
        pointer_pos: egui::Pos2,
        canvas_context: &CanvasContext,
    ) {
        let pos = canvas_context
            .camera
            .convert_from_camera(pointer_pos.to_vec2());

        let mut node_data = NodeData::default_mapcircle();

        let default_radius = 4.0;

        if let NodeData::MapCircle {
            position, radius, ..
        } = &mut node_data
        {
            *position = pos.into();
            *radius = default_radius;
        }

        Self::push_child_node(parent_node, NodeChildType::MapCircle, node_data);
    }

    /* helpers */

    fn push_child_node(
        parent_node: &mut MapDataNode,
        child_type: NodeChildType,
        node_data: NodeData,
    ) {
        let node = MapDataNode {
            node_type: child_type.into(),
            node_data,
            ..Default::default()
        };

        parent_node
            .children_of_type_vec_option_mut(child_type)
            .get_or_insert_with(Vec::new)
            .push(node);
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

impl From<egui::Vec2> for Vec2f {
    fn from(value: egui::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
