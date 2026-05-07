use std::path::PathBuf;

use crate::merino::{
    common::camera::CanvasCamera, game::mapbin::Mapbin, level_editor::le_params::ParameterObject,
};

use strum::{Display, EnumIter};

mod le_canvas;
mod le_edit_fields;
mod le_edit_object;
mod le_io;
mod le_params;
mod le_traits;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, Display)]
pub enum NodeChildType {
    MapPolySet,
    MapObjSet,
    MapItemSet,
    MapEnemySet,
    MapLocator,
    MapPath,
    MapRect,
    MapCircle,
    MapTerrain,
}

// impl NodeChildType {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             Self::MapPolySet => "[MapPolySet]",
//             Self::MapObjSet => "[MapObjSet]",
//             Self::MapItemSet => "[MapItemSet]",
//             Self::MapEnemySet => "[MapEnemySet]",
//             Self::MapLocator => "[MapLocator]",
//             Self::MapPath => "[MapPath]",
//             Self::MapRect => "[MapRect]",
//             Self::MapCircle => "[MapCircle]",
//             Self::MapTerrain => "[MapTerrain]",
//         }
//     }
// }

// in order to keep track of which nodes are selected.
// this is indicated in sequential traversal order
// e.g. [[Sub1, 0], [Sub2, 0], [Sub4, 1]] would be:
// Sub1[0].Sub2[0].Sub4[1]
pub type NodePath = Vec<(NodeChildType, usize)>;

#[derive(Default)]
pub struct CanvasContext {
    pub camera: CanvasCamera,
    pub selected_node_paths: Vec<NodePath>,
    // todo! make this toggleable
    pub display_dummy_terrain: bool,
}

#[derive(Default)]
pub struct IOContext {
    pub file_open: bool,
    pub file_path: Option<PathBuf>,
}

#[derive(Default)]
pub struct FileContext {
    pub mapdata: Mapbin,
}

#[derive(Default)]
pub struct ObjectPropertyEditorContext {
    pub node_path_to_remove: Option<NodePath>,
    pub parameter_objects: Vec<ParameterObject>,
}

// impl ObjectPropertyEditorContext {
//     pub fn get_param_object(&self, node_type: MapNodeType, name: &str) -> Option<&ParameterObject> {
//         self.parameter_objects
//             .iter()
//             .find(|obj| obj.set_type == node_type && obj.name == name)
//     }
// }

#[derive(Default)]
pub struct LevelEditor {
    // contexts
    pub io_context: IOContext,
    pub file_context: FileContext,
    pub canvas_context: CanvasContext,
    pub object_property_editor_context: ObjectPropertyEditorContext,
}

impl LevelEditor {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        // top panel
        egui::TopBottomPanel::top("le_top_panel").show(ui.ctx(), |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // file submenu
                if ui.button("Open").clicked() {
                    let _ = self.open_file();
                }

                if ui
                    .add_enabled(
                        self.io_context.file_open && self.io_context.file_path.is_some(),
                        egui::Button::new("Save As"),
                    )
                    .clicked()
                {
                    let _ = self.save_file();
                }

                if ui.button("Load Parameter Data").clicked() {
                    let _ = self.load_param_data();
                }
            });
        });

        // canvas
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            if self.io_context.file_open {
                self.show_canvas(ui);
            }
        });
    }

    pub fn handle_inputs(&mut self, ui: &mut egui::Ui) {
        let secondary_down = ui.input(|i| i.pointer.button_down(egui::PointerButton::Secondary));

        // camera pan
        if secondary_down {
            let delta = ui.input(|i| i.pointer.delta());
            if delta != egui::Vec2::ZERO {
                self.canvas_context
                    .camera
                    .pan(delta / self.canvas_context.camera.zoom);
            }
        }

        // pan reset handling
        if secondary_down && ui.input(|i| i.key_pressed(egui::Key::R)) {
            self.canvas_context.camera.reset();
        }

        // clear selections
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.canvas_context.selected_node_paths.clear();
        }
    }
}
