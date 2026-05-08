use strum::IntoEnumIterator;

use crate::merino::{
    common::emoji::*,
    game::mapbin::{MapDataNode, MapNodeType, NodeData},
    level_editor::{
        AddTarget, CanvasContext, FileContext, LevelEditor, NodeChildType, NodePath,
        ObjectPropertyEditorContext,
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

        ui.label(egui::RichText::new("Properties").strong());
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                match node.node_type {
                    MapNodeType::MapObjSet => {
                        Self::edit_mapobjset_properties(
                            ui,
                            object_property_editor_context,
                            &mut node.node_data,
                        );
                    }

                    _ => {
                        // todo!
                    }
                }
            });

        // view children

        Self::edit_child_ui(
            ui,
            object_property_editor_context,
            canvas_context,
            node,
            &node_path,
        );

        Self::confirm_child_deletion(
            ui,
            file_context,
            object_property_editor_context,
            canvas_context,
        );
    }

    fn edit_child_ui(
        ui: &mut egui::Ui,
        prop_context: &mut ObjectPropertyEditorContext,
        canvas_context: &mut CanvasContext,
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
                                                .on_hover_text("Delete node")
                                                .clicked()
                                            {
                                                let mut del_path = node_path.clone();
                                                del_path.push((child_type, index));
                                                prop_context.node_path_to_remove = Some(del_path);
                                            }

                                            if ui
                                                .button(EmojiMessage::target())
                                                .on_hover_text("Go to node")
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
                            canvas_context.current_add_object =
                                Some(AddTarget::node(child_type, node_path.clone()));
                        }

                        // if ui
                        //     .button(EmojiMessage::target_msg("Set Child"))
                        //     .on_hover_text("Select an existing node of this type.")
                        //     .clicked()
                        // {
                        //     println!("do something with this later too!");
                        //     // todo!
                        // }
                    });
                });
        }

        if let Some(path) = child_to_select {
            canvas_context.selected_node_paths.clear();
            canvas_context.selected_node_paths.push(path);
        }
    }

    fn confirm_child_deletion(
        ui: &mut egui::Ui,
        file_context: &mut FileContext,
        prop_context: &mut ObjectPropertyEditorContext,
        canvas_context: &mut CanvasContext,
    ) {
        if let Some(path) = &prop_context.node_path_to_remove {
            let mut should_close = false;
            let mut confirmed = false;
            let mapdata = &mut file_context.mapdata;

            // check if the node being deleted has its own children
            let has_children = mapdata
                .get_node_at_path(path)
                .map(|n| n.has_children())
                .unwrap_or(false);

            egui::Window::new("Confirm Deletion")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ui.ctx(), |ui| {
                    if has_children {
                        // todo! make warning togglable
                        ui.colored_label(
                            egui::Color32::LIGHT_RED,
                            EmojiMessage::warning_msg("Warning: this child has children."),
                        );
                        ui.label("Deleting it will remove all nested nodes.");
                    }

                    ui.label("Are you sure you want to delete this node?");

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button(EmojiMessage::cross_msg("Cancel")).clicked() {
                            should_close = true;
                        }

                        if ui.button(EmojiMessage::check_msg("Confirm")).clicked() {
                            confirmed = true;
                            should_close = true;
                        }
                    });
                });

            if confirmed {
                mapdata.remove_node_at_path(path);
                canvas_context.selected_node_paths.retain(|p| p != path);
            }

            if should_close {
                prop_context.node_path_to_remove = None;
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
