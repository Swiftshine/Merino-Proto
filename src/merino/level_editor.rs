use crate::merino::common::emoji::*;
use std::path::PathBuf;

use crate::merino::{
    common::camera::CanvasCamera, game::mapbin::Mapbin, level_editor::le_params::ParameterObject,
};

use strum::{Display, EnumIter};

mod le_canvas;
mod le_edit_fields;
mod le_edit_object;
mod le_inputs;
mod le_io;
mod le_node_tree;
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

#[derive(PartialEq)]
pub enum Tab {
    Canvas,
    ObjectProperties,
    AddObject,
    // NodeTree,
}

struct TabViewer<'a> {
    editor: &'a mut LevelEditor,
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Canvas => EmojiMessage::palette_msg("Canvas"),
            Tab::ObjectProperties => EmojiMessage::memo_msg("Object Properties"),
            Tab::AddObject => EmojiMessage::add_msg("Add Object"),
            // Tab::NodeTree => EmojiMessage::folder_msg("Node Tree"),
        }
        .into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let file_open = self.editor.io_context.file_open;

        match tab {
            Tab::Canvas => {
                if file_open {
                    self.editor.show_canvas(ui);
                } else {
                    ui.centered_and_justified(|ui| ui.label("No file open."));
                }
            }

            Tab::ObjectProperties => {
                if self.editor.canvas_context.selected_node_paths.len() == 1 {
                    let node_path = self.editor.canvas_context.selected_node_paths[0].clone();
                    self.editor.edit_node_properties(ui, node_path);
                } else {
                    ui.centered_and_justified(|ui| ui.label("Select exactly one object to edit."));
                }
            }

            Tab::AddObject => {
                ui.label("blah blah");
            } // Tab::NodeTree => {
              //     if file_open {
              //         self.editor.show_node_tree(ui);
              //     } else {
              //         ui.centered_and_justified(|ui| ui.label("No file open."));
              //     }
              // }
        }
    }
}

pub struct LevelEditor {
    // contexts
    pub io_context: IOContext,
    pub file_context: FileContext,
    pub canvas_context: CanvasContext,
    pub object_property_editor_context: ObjectPropertyEditorContext,

    pub dock_state: egui_dock::DockState<Tab>,
}

impl LevelEditor {
    pub fn new() -> Self {
        let mut dock_state = egui_dock::DockState::new(vec![Tab::Canvas]);

        // split window, properties on the right by default
        let [_canvas_node, _properties_node] = dock_state.main_surface_mut().split_right(
            egui_dock::NodeIndex::root(),
            0.8, // 80% canvas, 20% properties,
            vec![Tab::ObjectProperties],
        );

        Self {
            io_context: Default::default(),
            file_context: Default::default(),
            canvas_context: Default::default(),
            object_property_editor_context: Default::default(),
            dock_state,
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

                if ui.button(EmojiMessage::palette_msg("Canvas")).clicked() {
                    self.open_tab(Tab::Canvas);
                }

                if ui
                    .button(EmojiMessage::memo_msg("Object Properties"))
                    .clicked()
                {
                    self.open_tab(Tab::ObjectProperties)
                }

                if ui.button(EmojiMessage::add_msg("Add Object")).clicked() {
                    self.open_tab(Tab::AddObject)
                }

                // if ui.button(EmojiMessage::folder_msg("Node Tree")).clicked() {
                //     self.open_tab(Tab::NodeTree)
                // }
            });
        });

        // temporarily move dock_state to avoid borrowing &mut self twice
        let mut dock_state =
            std::mem::replace(&mut self.dock_state, egui_dock::DockState::new(vec![]));

        egui_dock::DockArea::new(&mut dock_state)
            .style(egui_dock::Style::from_egui(ui.style()))
            .show(ui.ctx(), &mut TabViewer { editor: self });

        // then put it back
        self.dock_state = dock_state;
    }

    pub fn handle_mouse_inputs(&mut self, ui: &mut egui::Ui) {
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
    }

    /// Checks if the tab is open, and if not, opens it.
    fn open_tab(&mut self, tab: Tab) {
        if !self
            .dock_state
            .main_surface()
            .iter()
            .any(|node| node.tabs().map_or(false, |tabs| tabs.contains(&tab)))
        {
            self.dock_state.main_surface_mut().push_to_focused_leaf(tab);
        }
    }
}
