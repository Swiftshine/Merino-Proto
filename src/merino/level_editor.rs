use std::path::PathBuf;

use crate::merino::{common::camera::CanvasCamera, game::mapbin::Mapbin};

mod le_canvas;
mod le_io;
mod le_object;

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
}

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
                    let _ = self.open_file(ui.ctx());
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

    // pub fn get_node_at_math(&mut self, path: &NodePath) -> Option<&mut MapDataNode> {
    //     let mut current = &mut self.mapdata.root;

    //     for (branch, index) in path {
    //         let vec = match branch {
    //             NodeBranch::Sub1 => &mut current.sub1,
    //             NodeBranch::Sub2 => &mut current.sub2,
    //             NodeBranch::Sub4 => &mut current.sub4,
    //             NodeBranch::Sub8 => &mut current.sub8,
    //             NodeBranch::Sub10 => &mut current.sub10,
    //             NodeBranch::Sub20 => &mut current.sub20,
    //             NodeBranch::Sub40 => &mut current.sub40,
    //             NodeBranch::Sub80 => &mut current.sub80,
    //             NodeBranch::Sub100 => &mut current.sub100,
    //         };

    //         current = vec.as_mut()?.get_mut(*index)?;
    //     }

    //     Some(current)
    // }
}
