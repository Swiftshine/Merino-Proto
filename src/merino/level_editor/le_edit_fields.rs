use strum::IntoEnumIterator;

use crate::merino::{
    common::emoji::*,
    game::mapbin::{MapDataNode, MapNodeType, NodeData},
    level_editor::{
        CanvasContext, CanvasTarget, EditorCommand, EditorContext, LevelEditor, NodeChildType,
        NodePath, ObjectPropertyEditorContext,
        le_traits::{EditInfo, Editable},
    },
};

impl LevelEditor {
    pub fn edit_node_properties(&mut self, ui: &mut egui::Ui, node_path: NodePath) {
        let LevelEditor {
            file_context,
            object_property_editor_context,
            canvas_context,
            editor_context,
            ..
        } = self;

        let mapdata = &mut file_context.mapdata;
        let node = match mapdata.get_node_at_path(&node_path) {
            Some(n) => n,
            None => return,
        };

        // don't allow user to attempt to delete the root node
        if node.node_type != MapNodeType::MapSet {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Properties").strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(EmojiMessage::discard())
                        .on_hover_text("Delete node")
                        .clicked()
                    {
                        editor_context
                            .commands
                            .push(EditorCommand::remove_node(node_path.clone()));
                    }
                });
            });
        }

        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| match node.node_type {
                MapNodeType::MapSet => {
                    Self::edit_mapset_properties(ui, &mut node.node_data);
                }

                MapNodeType::MapPolySet => {
                    Self::edit_mappolyset_properties(ui, &mut node.node_data);
                }

                MapNodeType::MapObjSet => {
                    Self::edit_mapobjset_properties(
                        ui,
                        object_property_editor_context,
                        &mut node.node_data,
                    );
                }

                MapNodeType::MapItemSet => {
                    Self::edit_mapitemset_properties(
                        ui,
                        object_property_editor_context,
                        &mut node.node_data,
                    );
                }

                MapNodeType::MapEnemySet => {
                    Self::edit_mapenemyset_properties(
                        ui,
                        object_property_editor_context,
                        &mut node.node_data,
                    );
                }

                MapNodeType::MapLocator => {
                    Self::edit_maplocator_properties(
                        ui,
                        object_property_editor_context,
                        &mut node.node_data,
                    );
                }

                MapNodeType::MapPath => {
                    Self::edit_mappath_properties(
                        ui,
                        object_property_editor_context,
                        &mut node.node_data,
                    );
                }

                MapNodeType::MapRect => {
                    Self::edit_maprect_properties(
                        ui,
                        object_property_editor_context,
                        &mut node.node_data,
                    );
                }

                MapNodeType::MapCircle => {
                    Self::edit_mapcircle_properties(
                        ui,
                        object_property_editor_context,
                        &mut node.node_data,
                    );
                }

                MapNodeType::MapTerrain => {
                    Self::edit_mapterrain_properties(
                        ui,
                        object_property_editor_context,
                        &mut node.node_data,
                    );
                }
            });

        // view children

        Self::edit_child_ui(ui, canvas_context, editor_context, node, &node_path);
    }

    fn edit_child_ui(
        ui: &mut egui::Ui,
        canvas_context: &mut CanvasContext,
        editor_context: &mut EditorContext,
        node: &mut MapDataNode,
        node_path: &NodePath,
    ) {
        ui.label(egui::RichText::new("Children").strong());
        let mut child_to_select = None;

        for child_type in NodeChildType::iter() {
            egui::Frame::new()
                .fill(ui.visuals().faint_bg_color)
                .corner_radius(4.0)
                .inner_margin(4.0)
                .show(ui, |ui| {
                    ui.label(child_type.to_string());

                    // dont make indentations if no children present
                    let has_children = node.has_child_of_type(child_type);
                    if has_children {
                        ui.indent(ui.id().with(child_type), |ui| {
                            for (index, _) in node.children_of_type_mut(child_type).enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("Index {}", index));

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if ui
                                                .button(EmojiMessage::discard())
                                                .on_hover_text("Delete child")
                                                .clicked()
                                            {
                                                let mut del_path = node_path.clone();
                                                del_path.push((child_type, index));
                                                editor_context
                                                    .commands
                                                    .push(EditorCommand::remove_node(del_path));
                                            }

                                            if ui
                                                .button(EmojiMessage::cross())
                                                .on_hover_text("Detach child")
                                                .clicked()
                                            {
                                                let mut child_path = node_path.clone();
                                                child_path.push((child_type, index));

                                                // set the parent of the child to root
                                                editor_context.commands.push(
                                                    EditorCommand::move_node(
                                                        child_path,
                                                        Vec::new(), // root
                                                    ),
                                                );
                                            }

                                            if ui
                                                .button(EmojiMessage::target())
                                                .on_hover_text("Go to child")
                                                .clicked()
                                            {
                                                let mut new_path = node_path.clone();
                                                new_path.push((child_type, index));
                                                child_to_select = Some(new_path);

                                                // todo! snap camera to that position
                                            }
                                        },
                                    )
                                });
                            }
                        });
                    }

                    ui.horizontal(|ui| {
                        if ui
                            .button(EmojiMessage::add_msg("New child"))
                            .on_hover_text("Create a new node of this type.")
                            .clicked()
                        {
                            canvas_context.target =
                                Some(CanvasTarget::new_to_node(child_type, node_path.clone()));
                        }

                        if ui
                            .button(EmojiMessage::target_msg("Set Child"))
                            .on_hover_text("Select an existing node of this type.")
                            .clicked()
                        {
                            canvas_context.target = Some(CanvasTarget::search(node_path.clone()));
                        }
                    });
                });
        }

        if let Some(path) = child_to_select {
            canvas_context.selected_node_paths.clear();
            canvas_context.selected_node_paths.push(path);
        }
    }

    fn edit_mapobjset_properties(
        ui: &mut egui::Ui,
        context: &mut ObjectPropertyEditorContext,
        node_data: &mut NodeData,
    ) {
        if let NodeData::MapObjSet {
            name,
            position,
            unk3,
            unk4,
            unk5,
            unk6,
            unk7,
            unk8,
            unk9,
            unk10,
            unk11,
            unk12,
            unk13,
            params,
            unk14,
        } = node_data
        {
            // todo!() search for display name and do that
            // also todo!() search for note and add that
            name.edit_properties(ui, EditInfo::label("Name"));
            position.edit_properties(ui, EditInfo::label("Position"));
            unk3.edit_properties(ui, EditInfo::label("Unk 3"));
            unk4.edit_properties(ui, EditInfo::label("Unk 4"));
            unk5.edit_properties(ui, EditInfo::label("Unk 5"));
            unk6.edit_properties(ui, EditInfo::label("Unk 6"));
            unk7.edit_properties(ui, EditInfo::label("Unk 7"));
            unk8.edit_properties(ui, EditInfo::label("Unk 8"));
            unk9.edit_properties(ui, EditInfo::label("Unk 9"));
            unk10.edit_properties(ui, EditInfo::label("Unk 10"));
            unk11.edit_properties(ui, EditInfo::label("Unk 11"));
            unk12.edit_properties(ui, EditInfo::label("Unk 12"));
            unk13.edit_properties(ui, EditInfo::label("Unk 13"));
            unk14.edit_properties(ui, EditInfo::label("Unk 14"));

            params.edit_properties(
                ui,
                EditInfo::search_param(
                    &context.parameter_objects,
                    MapNodeType::MapObjSet,
                    name.as_str(),
                ),
            );
        }
    }

    fn edit_mapset_properties(ui: &mut egui::Ui, node_data: &mut NodeData) {
        if let NodeData::MapSet {
            unk1,
            bounds_start,
            bounds_end,
        } = node_data
        {
            unk1.edit_properties(ui, EditInfo::label("Unk 1"));
            bounds_start.edit_properties(ui, EditInfo::label("Bounds Start"));
            bounds_end.edit_properties(ui, EditInfo::label("Bounds End"));
        }
    }

    fn edit_mappolyset_properties(ui: &mut egui::Ui, node_data: &mut NodeData) {
        if let NodeData::MapPolySet {
            start,
            end,
            collision_type,
            unk3,
            ..
        } = node_data
        {
            // not allowing the user to edit the collision normal because that is to be automatically calculated
            start.edit_properties(ui, EditInfo::label("Start"));
            end.edit_properties(ui, EditInfo::label("End"));
            collision_type.edit_properties(ui, EditInfo::label("Collision Type"));
            unk3.edit_properties(ui, EditInfo::label("Unk 3"));
        }
    }

    fn edit_mapitemset_properties(
        ui: &mut egui::Ui,
        context: &mut ObjectPropertyEditorContext,
        node_data: &mut NodeData,
    ) {
        if let NodeData::MapItemSet {
            name,
            position,
            unk3,
            unk4,
            unk5,
            unk6,
            unk7,
            unk8,
            unk9,
            unk10,
            unk11,
            unk12,
            unk13,
            params,
        } = node_data
        {
            name.edit_properties(ui, EditInfo::label("Name"));
            position.edit_properties(ui, EditInfo::label("Position"));
            unk3.edit_properties(ui, EditInfo::label("Unk 3"));
            unk4.edit_properties(ui, EditInfo::label("Unk 4"));
            unk5.edit_properties(ui, EditInfo::label("Unk 5"));
            unk6.edit_properties(ui, EditInfo::label("Unk 6"));
            unk7.edit_properties(ui, EditInfo::label("Unk 7"));
            unk8.edit_properties(ui, EditInfo::label("Unk 8"));
            unk9.edit_properties(ui, EditInfo::label("Unk 9"));
            unk10.edit_properties(ui, EditInfo::label("Unk 10"));
            unk11.edit_properties(ui, EditInfo::label("Unk 11"));
            unk12.edit_properties(ui, EditInfo::label("Unk 12"));
            unk13.edit_properties(ui, EditInfo::label("Unk 13"));

            params.edit_properties(
                ui,
                EditInfo::search_param(
                    &context.parameter_objects,
                    MapNodeType::MapItemSet,
                    name.as_str(),
                ),
            );
        }
    }

    fn edit_mapenemyset_properties(
        ui: &mut egui::Ui,
        context: &mut ObjectPropertyEditorContext,
        node_data: &mut NodeData,
    ) {
        if let NodeData::MapEnemySet {
            name,
            direction,
            orientation,
            position,
            unk7,
            unk8,
            unk9,
            unk10,
            unk11,
            unk12,
            unk13,
            unk14,
            unk15,
            unk16,
            unk17,
            unk18,
            unk19,
            unk20,
            unk21,
            unk22,
            unk23,
            unk24,
            params,
        } = node_data
        {
            name.edit_properties(ui, EditInfo::label("Name"));
            direction.edit_properties(ui, EditInfo::label("Direction"));
            orientation.edit_properties(ui, EditInfo::label("Orientation"));
            position.edit_properties(ui, EditInfo::label("Position"));

            unk7.edit_properties(ui, EditInfo::label("Unk 7"));
            unk8.edit_properties(ui, EditInfo::label("Unk 8"));
            unk9.edit_properties(ui, EditInfo::label("Unk 9"));
            unk10.edit_properties(ui, EditInfo::label("Unk 10"));
            unk11.edit_properties(ui, EditInfo::label("Unk 11"));
            unk12.edit_properties(ui, EditInfo::label("Unk 12"));
            unk13.edit_properties(ui, EditInfo::label("Unk 13"));
            unk14.edit_properties(ui, EditInfo::label("Unk 14"));
            unk15.edit_properties(ui, EditInfo::label("Unk 15"));
            unk16.edit_properties(ui, EditInfo::label("Unk 16"));
            unk17.edit_properties(ui, EditInfo::label("Unk 17"));
            unk18.edit_properties(ui, EditInfo::label("Unk 18"));
            unk19.edit_properties(ui, EditInfo::label("Unk 19"));
            unk20.edit_properties(ui, EditInfo::label("Unk 20"));
            unk21.edit_properties(ui, EditInfo::label("Unk 21"));
            unk22.edit_properties(ui, EditInfo::label("Unk 22"));
            unk23.edit_properties(ui, EditInfo::label("Unk 23"));
            unk24.edit_properties(ui, EditInfo::label("Unk 24"));

            params.edit_properties(
                ui,
                EditInfo::search_param(
                    &context.parameter_objects,
                    MapNodeType::MapEnemySet,
                    name.as_str(),
                ),
            );
        }
    }

    fn edit_maplocator_properties(
        ui: &mut egui::Ui,
        context: &mut ObjectPropertyEditorContext,
        node_data: &mut NodeData,
    ) {
        if let NodeData::MapLocator {
            name,
            position,
            params,
        } = node_data
        {
            name.edit_properties(ui, EditInfo::label("Name"));
            position.edit_properties(ui, EditInfo::label("Position"));

            params.edit_properties(
                ui,
                EditInfo::search_param(
                    &context.parameter_objects,
                    MapNodeType::MapLocator,
                    name.as_str(),
                ),
            );
        }
    }

    fn edit_mappath_properties(
        ui: &mut egui::Ui,
        context: &mut ObjectPropertyEditorContext,
        node_data: &mut NodeData,
    ) {
        if let NodeData::MapPath {
            name,
            points,
            params,
        } = node_data
        {
            name.edit_properties(ui, EditInfo::label("Name"));
            points.edit_properties(ui, EditInfo::label("Points"));

            params.edit_properties(
                ui,
                EditInfo::search_param(
                    &context.parameter_objects,
                    MapNodeType::MapPath,
                    name.as_str(),
                ),
            );
        }
    }

    fn edit_maprect_properties(
        ui: &mut egui::Ui,
        context: &mut ObjectPropertyEditorContext,
        node_data: &mut NodeData,
    ) {
        if let NodeData::MapRect {
            name,
            bounds_start,
            bounds_end,
            params,
        } = node_data
        {
            name.edit_properties(ui, EditInfo::label("Name"));
            bounds_start.edit_properties(ui, EditInfo::label("Bounds Start"));
            bounds_end.edit_properties(ui, EditInfo::label("Bounds End"));

            params.edit_properties(
                ui,
                EditInfo::search_param(
                    &context.parameter_objects,
                    MapNodeType::MapRect,
                    name.as_str(),
                ),
            );
        }
    }

    fn edit_mapcircle_properties(
        ui: &mut egui::Ui,
        context: &mut ObjectPropertyEditorContext,
        node_data: &mut NodeData,
    ) {
        if let NodeData::MapCircle {
            name,
            position,
            radius,
            params,
        } = node_data
        {
            name.edit_properties(ui, EditInfo::label("Name"));
            position.edit_properties(ui, EditInfo::label("Position"));
            radius.edit_properties(ui, EditInfo::label("Radius"));

            params.edit_properties(
                ui,
                EditInfo::search_param(
                    &context.parameter_objects,
                    MapNodeType::MapCircle,
                    name.as_str(),
                ),
            );
        }
    }

    fn edit_mapterrain_properties(
        ui: &mut egui::Ui,
        context: &mut ObjectPropertyEditorContext,
        node_data: &mut NodeData,
    ) {
        if let NodeData::MapTerrain {
            collision_type,
            position,
            unk3,
            unk4,
            unk5,
            unk6,
            unk7,
            unk8,
            unk9,
            unk10,
            unk11,
            unk12,
            unk13,
            params,
            unk15,
        } = node_data
        {
            collision_type.edit_properties(ui, EditInfo::label("Terrain Type"));
            position.edit_properties(ui, EditInfo::label("Position"));

            unk3.edit_properties(ui, EditInfo::label("Unk 3"));
            unk4.edit_properties(ui, EditInfo::label("Unk 4"));
            unk5.edit_properties(ui, EditInfo::label("Unk 5"));
            unk6.edit_properties(ui, EditInfo::label("Unk 6"));
            unk7.edit_properties(ui, EditInfo::label("Unk 7"));
            unk8.edit_properties(ui, EditInfo::label("Unk 8"));
            unk9.edit_properties(ui, EditInfo::label("Unk 9"));
            unk10.edit_properties(ui, EditInfo::label("Unk 10"));
            unk11.edit_properties(ui, EditInfo::label("Unk 11"));
            unk12.edit_properties(ui, EditInfo::label("Unk 12"));
            unk13.edit_properties(ui, EditInfo::label("Unk 13"));
            unk15.edit_properties(ui, EditInfo::label("Unk 15"));

            params.edit_properties(
                ui,
                EditInfo::search_param(
                    &context.parameter_objects,
                    MapNodeType::MapTerrain,
                    collision_type.as_str(),
                ),
            );
        }
    }
}
