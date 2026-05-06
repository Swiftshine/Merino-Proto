use crate::merino::{
    game::mapbin::{MapNodeType, NodeData},
    level_editor::{LevelEditor, NodePath, le_traits::Editable},
};

impl LevelEditor {
    pub fn edit_node_properties(&mut self, ui: &mut egui::Ui, node_path: NodePath) {
        let node = match self.get_node_at_path(node_path) {
            Some(n) => n,
            None => return,
        };

        egui::Area::new(egui::Id::from("le_node_property_editor"))
            .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-10.0, 10.0))
            .show(ui.ctx(), |ui| {
                egui::Frame::popup(ui.style())
                    .inner_margin(egui::Vec2::splat(8.0))
                    .show(ui, |ui| {
                        let base_label = "Edit node properties ".to_string();

                        let node_type_string = node.node_type.to_string();

                        ui.label(format!("{} ({})", base_label, node_type_string));
                        match node.node_type {
                            MapNodeType::MapObjSet => {
                                Self::edit_mapobjset_properties(ui, &mut node.node_data);
                            }
                            _ => {}
                        }
                    });
            });
    }

    fn edit_mapobjset_properties(ui: &mut egui::Ui, node_data: &mut NodeData) {
        // blah blah
        if let NodeData::MapObjSet { name, position, .. } = node_data {
            ui.label("Name");
            ui.add(egui::TextEdit::singleline(name).char_limit(32));
            ui.label("Position");
            position.edit_properties(ui);
        }
    }
}
