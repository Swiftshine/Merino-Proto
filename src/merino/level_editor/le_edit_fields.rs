use strum::IntoEnumIterator;

use crate::merino::{
    common::emoji::*,
    game::mapbin::{MapNodeType, NodeData},
    level_editor::{
        LevelEditor, NodeChildType, NodePath, ObjectPropertyEditorContext,
        le_traits::{EditInfo, Editable},
    },
};

impl LevelEditor {
    pub fn edit_node_properties(&mut self, ui: &mut egui::Ui, node_path: NodePath) {
        let LevelEditor {
            file_context,
            object_property_editor_context,
            canvas_context,
            ..
        } = self;

        let mapdata = &mut file_context.mapdata;
        let node = match mapdata.get_node_at_path(&node_path) {
            Some(n) => n,
            None => return,
        };

        let mut child_to_select = None;

        ui.label(egui::RichText::new("Properties").strong());
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| match node.node_type {
                MapNodeType::MapObjSet => {
                    Self::edit_mapobjset_properties(
                        ui,
                        object_property_editor_context,
                        &mut node.node_data,
                    );
                }
                _ => {}
            });

        // view children

        let has_children = node.has_children();

        ui.vertical(|ui| {
            ui.label(egui::RichText::new("Children").strong())
                .on_hover_text("Click on a child to go to it.");
            ui.add_space(4.0);

            if has_children {
                egui::Frame::new()
                    .fill(ui.visuals().faint_bg_color)
                    .corner_radius(4.0)
                    .inner_margin(4.0)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("node_children_scroll")
                            .max_height(300.0)
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                for child_type in NodeChildType::iter() {
                                    // check if we have children of this category
                                    let children: Vec<_> = node
                                        .all_children_mut()
                                        .filter(|(branch, _, _)| *branch == child_type)
                                        .collect();

                                    if children.is_empty() {
                                        continue;
                                    }

                                    // create subheader
                                    ui.label(child_type.to_string());

                                    ui.indent(ui.id().with(child_type), |ui| {
                                        for (branch, index, _) in children {
                                            ui.horizontal(|ui| {
                                                let label = format!("Index {}", index);

                                                if ui.button(label).clicked() {
                                                    let mut new_path = node_path.clone();
                                                    new_path.push((branch, index));
                                                    child_to_select = Some(new_path);

                                                    // todo! snap camera to that position
                                                }

                                                ui.with_layout(
                                                    egui::Layout::right_to_left(
                                                        egui::Align::Center,
                                                    ),
                                                    |ui| {
                                                        if ui
                                                            .button(EmojiMessage::discard())
                                                            .on_hover_text("Delete Node")
                                                            .clicked()
                                                        {
                                                            let mut del_path = node_path.clone();
                                                            del_path.push((branch, index));
                                                            object_property_editor_context
                                                                .node_path_to_remove =
                                                                Some(del_path);
                                                        }
                                                    },
                                                );
                                            });
                                        }
                                    });
                                    ui.add_space(4.0);
                                }

                                if let Some(path) = child_to_select {
                                    canvas_context.selected_node_paths.clear();
                                    canvas_context.selected_node_paths.push(path);
                                }
                            });
                    });
            } else {
                // add children
                ui.label("No children. Add some?");
                ui.label("[TODO] set up addition/\"make child\" options");
            }
        });

        if let Some(ref path) = object_property_editor_context.node_path_to_remove {
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
                            EmojiMessage::warning_msg("Warning: This node contains children."),
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
                canvas_context.selected_node_paths.retain(|p| p != path); // Clean up selection
            }

            if should_close {
                object_property_editor_context.node_path_to_remove = None;
            }
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
}
