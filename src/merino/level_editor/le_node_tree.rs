// use crate::merino::{
//     common::emoji::EmojiMessage,
//     game::mapbin::MapDataNode,
//     level_editor::{LevelEditor, NodeChildType, NodePath},
// };
// use strum::IntoEnumIterator;

// impl MapDataNode {
//     // todo! show selected object (the one the object properties menu is open for) in node tree.
//     // todo! also use dragging and dropping or some other manual node moving method
//     /// Helper method for showing a tree.
//     fn ui_show_tree(&mut self, ui: &mut egui::Ui, name: &str, path: &mut NodePath) {
//         egui::CollapsingHeader::new(name)
//             .id_salt(&path)
//             .default_open(false)
//             .show(ui, |ui| {
//                 for child_type in NodeChildType::iter() {
//                     self.ui_show_child_folder(ui, &child_type.to_string(), child_type, path);
//                 }
//             });
//     }

//     fn ui_show_child_folder(
//         &mut self,
//         ui: &mut egui::Ui,
//         label: &str,
//         child_type: NodeChildType,
//         path: &mut NodePath,
//     ) {
//         // only show the folder if there are actually children inside it
//         if !self.has_child_of_type(child_type) {
//             return;
//         }

//         egui::CollapsingHeader::new(label).show(ui, |ui| {
//             for (i, child) in self.children_of_type_mut(child_type).enumerate() {
//                 path.push((child_type, i));
//                 let node_label = format!("Index {i}");
//                 ui.horizontal(|ui| {
//                     if ui
//                         .button(EmojiMessage::add())
//                         .on_hover_text("Add child")
//                         .clicked()
//                     {
//                         todo!();
//                     }

//                     if ui
//                         .button(EmojiMessage::discard())
//                         .on_hover_text("Delete node")
//                         .clicked()
//                     {
//                         todo!();
//                     }

//                     if ui
//                         .button(EmojiMessage::target())
//                         .on_hover_text("Go to node")
//                         .clicked()
//                     {
//                         todo!();
//                     }
//                 });

//                 if child.has_children() {
//                     child.ui_show_tree(ui, &node_label, path);
//                 }
//                 path.pop();
//             }
//         });
//     }
// }

// impl LevelEditor {
//     pub fn show_node_tree(&mut self, ui: &mut egui::Ui) {
//         egui::ScrollArea::vertical().show(ui, |ui| {
//             let mut path = NodePath::default();
//             let root = &mut self.file_context.mapdata.root;

//             root.ui_show_tree(ui, "MapSet", &mut path);
//         });
//     }
// }
