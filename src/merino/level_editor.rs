use std::path::PathBuf;

use crate::merino::{
    common::camera::CanvasCamera, game::mapbin::Mapbin, level_editor::le_params::ParameterObject,
};

mod le_canvas;
mod le_edit_fields;
mod le_edit_object;
mod le_io;
mod le_params;
mod le_traits;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NodeBranch {
    Sub1,
    Sub2,
    Sub4,
    Sub8,
    Sub10,
    Sub20,
    Sub40,
    Sub80,
    Sub100,
}

// in order to keep track of which nodes are selected.
// this is indicated in sequential traversal order
// e.g. [[Sub1, 0], [Sub2, 0], [Sub4, 1]] would be:
// Sub1[0].Sub2[0].Sub4[1]
pub type NodePath = Vec<(NodeBranch, usize)>;

#[derive(Default)]
pub struct LevelEditorState {
    pub camera: CanvasCamera,
    pub selected_node_paths: Vec<NodePath>,
    pub parameter_objects: Vec<ParameterObject>,
}

// impl LevelEditorState {
//     pub fn get_param_object(&self, node_type: MapNodeType, name: &str) -> Option<&ParameterObject> {
//         self.parameter_objects
//             .iter()
//             .find(|obj| obj.set_type == node_type && obj.name == name)
//     }
// }

pub struct LevelEditor {
    // i/o
    file_open: bool,
    file_path: Option<PathBuf>,

    // files
    mapdata: Mapbin,

    // state
    pub state: LevelEditorState,
}

impl LevelEditor {
    pub fn new() -> Self {
        Self {
            file_open: false,
            file_path: None,
            mapdata: Mapbin::default(),
            state: LevelEditorState::default(),
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
                        self.file_open && self.file_path.is_some(),
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
            if self.file_open {
                self.show_canvas(ui);
            }
        });
    }

    pub fn handle_inputs(&mut self, ui: &mut egui::Ui, response: &egui::Response) {
        // camera pan
        if response.dragged_by(egui::PointerButton::Secondary) {
            let delta = response.drag_delta();
            self.state.camera.pan(delta / self.state.camera.zoom)
        }

        // clear selections
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.state.selected_node_paths.clear();
        }
    }
}
