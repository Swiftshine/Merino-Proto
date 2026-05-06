use crate::merino::{
    game::mapbin::{MapNodeType, NodeData},
    level_editor::{
        LevelEditor, NodePath,
        le_traits::{EditInfo, Editable},
    },
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
                        let node_type_string = node.node_type.to_string();
                        ui.label(format!("Edit node properties ({node_type_string})"));

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
            name.edit_properties(ui, EditInfo::string("Name", 32));
            position.edit_properties(ui, EditInfo::value("Position"));
            unk3.edit_properties(ui, EditInfo::value("Unk 3"));
            unk4.edit_properties(ui, EditInfo::value("Unk 4"));
            unk5.edit_properties(ui, EditInfo::string("Unk 5", 32));
            unk6.edit_properties(ui, EditInfo::value("Unk 6"));
            unk7.edit_properties(ui, EditInfo::string("Unk 7", 32));
            unk8.edit_properties(ui, EditInfo::value("Unk 8"));
            unk9.edit_properties(ui, EditInfo::value("Unk 9"));
            unk10.edit_properties(ui, EditInfo::value("Unk 10"));
            unk11.edit_properties(ui, EditInfo::value("Unk 11"));
            unk12.edit_properties(ui, EditInfo::value("Unk 12"));
            unk13.edit_properties(ui, EditInfo::value("Unk 13"));
            params.edit_properties(ui, None);
        }
    }
}
