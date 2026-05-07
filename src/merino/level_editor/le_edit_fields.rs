use crate::merino::{
    common::emoji::*,
    game::mapbin::{MapNodeType, NodeData},
    level_editor::{
        LevelEditor, LevelEditorState, NodePath,
        le_traits::{EditInfo, Editable},
    },
};

impl LevelEditor {
    pub fn edit_node_properties(&mut self, ui: &mut egui::Ui, node_path: NodePath) {
        let LevelEditor { mapdata, state, .. } = self;

        let node = match mapdata.get_node_at_path(&node_path) {
            Some(n) => n,
            None => return,
        };

        let mut child_to_select = None;

        egui::Area::new(egui::Id::from("le_node_property_editor"))
            .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-10.0, 10.0))
            .show(ui.ctx(), |ui| {
                egui::Frame::popup(ui.style())
                    .inner_margin(egui::Vec2::splat(8.0))
                    .show(ui, |ui| {
                        let node_type_string = node.node_type.to_string();
                        ui.label(format!("Edit node properties ({node_type_string})"));

                        ui.separator();

                        egui::ScrollArea::vertical()
                            .max_height(400.0)
                            .show(ui, |ui| match node.node_type {
                                MapNodeType::MapObjSet => {
                                    Self::edit_mapobjset_properties(ui, state, &mut node.node_data);
                                }
                                _ => {}
                            });

                        // view children

                        let has_children = node.has_children();

                        if has_children {
                            ui.separator();
                            let resp = ui.collapsing("Children", |ui| {
                                egui::ScrollArea::vertical()
                                    .max_height(400.0)
                                    .show(ui, |ui| {
                                        for (branch, index, child) in node.all_children_mut() {
                                            ui.horizontal(|ui| {
                                                let label = format!("Child Index {}", index);

                                                let hover_label = format!(
                                                    "[{}] Index {}: {}",
                                                    branch.as_str(),
                                                    index,
                                                    child.node_type
                                                );

                                                if ui
                                                    .button(label)
                                                    .on_hover_text(hover_label)
                                                    .clicked()
                                                {
                                                    let mut new_path = node_path.clone();
                                                    new_path.push((branch, index));
                                                    child_to_select = Some(new_path);
                                                }

                                                // the delete button goes on the far right
                                                ui.with_layout(
                                                    egui::Layout::right_to_left(
                                                        egui::Align::Center,
                                                    ),
                                                    |ui| {
                                                        if ui
                                                            .button(ICON_DISCARD)
                                                            .on_hover_text("Delete Node")
                                                            .clicked()
                                                        {
                                                            let mut del_path = node_path.clone();
                                                            del_path.push((branch, index));
                                                            state.node_path_to_remove =
                                                                Some(del_path);
                                                        }
                                                    },
                                                );
                                            });
                                        }

                                        if let Some(path) = child_to_select {
                                            state.selected_node_paths.clear();
                                            state.selected_node_paths.push(path);
                                        }
                                    });
                            });

                            resp.header_response.on_hover_text("View child nodes.");
                        } else {
                            // todo! give option to add child
                        }
                    });
            });

        if let Some(ref path) = state.node_path_to_remove {
            let mut should_close = false;
            let mut confirmed = false;

            // check if the node being deleted has its own children
            let has_sub_children = mapdata
                .get_node_at_path(&path)
                .map(|n| n.has_children())
                .unwrap_or(false);

            egui::Window::new("Confirm Deletion")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ui.ctx(), |ui| {
                    if has_sub_children {
                        // todo - make the warning togglable
                        ui.colored_label(
                            egui::Color32::LIGHT_RED,
                            &format!("{ICON_WARNING} Warning: this node contains children."),
                        );
                        ui.label("Deleting it will remove all nested nodes.");
                    } else {
                        ui.label("Are you sure you want to delete this node?");
                    }

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            should_close = true;
                        }
                        if ui.button("Delete").clicked() {
                            confirmed = true;
                            should_close = true;
                        }
                    });
                });

            if confirmed {
                mapdata.remove_node_at_path(path);
                state.selected_node_paths.retain(|p| p != path); // Clean up selection
            }

            if should_close {
                state.node_path_to_remove = None;
            }
        }
    }

    fn edit_mapobjset_properties(
        ui: &mut egui::Ui,
        state: &mut LevelEditorState,
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
                    &state.parameter_objects,
                    MapNodeType::MapObjSet,
                    name.as_str(),
                ),
            );
        }
    }
}
