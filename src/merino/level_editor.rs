use crate::merino::common::util::get_merino_folder_path;
use crate::merino::game::mapbin::Params;
use crate::merino::level_editor::le_image::ImageDefinition;
use crate::merino::{
    common::camera::CanvasCamera, game::mapbin::Mapbin, level_editor::le_params::ParameterObject,
};
use crate::merino::{common::emoji::*, game::mapbin::MapNodeType};
use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use enum_map::EnumMap;
use strum::{Display, EnumIter};

mod le_add_object;
mod le_canvas;
mod le_edit_fields;
mod le_edit_object;
mod le_image;
mod le_inputs;
mod le_io;
mod le_node_tree;
mod le_params;
mod le_set_object;
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

impl From<NodeChildType> for MapNodeType {
    fn from(value: NodeChildType) -> Self {
        match value {
            NodeChildType::MapPolySet => Self::MapPolySet,
            NodeChildType::MapObjSet => Self::MapObjSet,
            NodeChildType::MapItemSet => Self::MapItemSet,
            NodeChildType::MapEnemySet => Self::MapEnemySet,
            NodeChildType::MapLocator => Self::MapLocator,
            NodeChildType::MapPath => Self::MapPath,
            NodeChildType::MapRect => Self::MapRect,
            NodeChildType::MapCircle => Self::MapCircle,
            NodeChildType::MapTerrain => Self::MapTerrain,
        }
    }
}

pub enum EditorCommand {
    MoveNode {
        child: NodePath,
        new_parent: NodePath,
    },
    RemoveNode {
        path: NodePath,
    },
    SelectNode {
        path: NodePath,
    },
    AddToSelection {
        path: NodePath,
    },
    // SelectParent {
    //     child: NodePath,
    // },
}

impl EditorCommand {
    pub fn move_node(child: NodePath, new_parent: NodePath) -> Self {
        Self::MoveNode { child, new_parent }
    }

    pub fn remove_node(path: NodePath) -> Self {
        Self::RemoveNode { path }
    }

    /// Adds to node selection
    pub fn add_to_selection(path: NodePath) -> Self {
        Self::AddToSelection { path }
    }

    /// Clears the node selection and sets this one
    pub fn select_node(path: NodePath) -> Self {
        Self::SelectNode { path }
    }

    // pub fn select_parent_of(child: NodePath) -> Self {
    //     Self::SelectParent { child }
    // }
}

// todo! maybe something like EditorRequest that queries something?
// although the result would only be obtained on the next frame

pub struct NodeEditSettings {
    pub visible: bool,
    pub editable: bool,
}

impl Default for NodeEditSettings {
    fn default() -> Self {
        Self {
            visible: true,
            editable: true,
        }
    }
}

// in order to keep track of which nodes are selected.
// this is indicated in sequential traversal order
// e.g. [[MapPolySet, 0], [MapObjSet, 0], [MapItemSet, 1]] would be:
// MapPolySet[0].MapObjSet[0].MapItemSet[1]
pub type NodePath = Vec<(NodeChildType, usize)>;

pub enum CanvasTarget {
    /// Create a new child to attach to the root node.
    NewToRoot(NodeChildType),
    /// Create a new child to attach to an existing node.
    NewToNode(NodeChildType, NodePath),
    // /// Search for an existing node to attach to the root node.
    // SearchRoot,
    /// Search for an existing node to attach to the given parent.
    Search(NodePath),
}

impl CanvasTarget {
    pub fn new_to_root(child_type: NodeChildType) -> Self {
        Self::NewToRoot(child_type)
    }

    pub fn new_to_node(child_type: NodeChildType, new_owner: NodePath) -> Self {
        Self::NewToNode(child_type, new_owner)
    }

    // pub fn search_root() -> Self {
    //     Self::SearchRoot
    // }

    pub fn search(parent: NodePath) -> Self {
        Self::Search(parent)
    }

    fn get_type(&self) -> Option<NodeChildType> {
        match self {
            Self::NewToRoot(child_type) => Some(*child_type),
            Self::NewToNode(child_type, ..) => Some(*child_type),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        if let Some(child_type) = self.get_type() {
            child_type.to_string()
        } else {
            String::from("Existing node")
        }
    }

    pub fn is_add(&self) -> bool {
        match self {
            Self::NewToRoot(..) => true,
            Self::NewToNode(..) => true,
            _ => false,
        }
    }

    pub fn is_search(&self) -> bool {
        match self {
            Self::Search(..) => true,
            _ => false,
        }
    }
}

pub struct DeleteConfirmation {
    path: NodePath,
}

#[derive(Default)]
pub struct EditorContext {
    pub commands: Vec<EditorCommand>,
    pub pending_delete: Option<DeleteConfirmation>,
}

#[derive(Default)]
pub struct ImageBank {
    pub image_objects: HashMap<(MapNodeType, String), ImageDefinition>,
    textures: HashMap<String, egui::TextureHandle>,
}

impl ImageBank {
    pub fn load_texture(
        &mut self,
        ctx: &egui::Context,
        asset_id: &str,
        file_path: &str,
    ) -> Result<()> {
        if self.textures.contains_key(asset_id) {
            return Ok(());
        }

        let image = image::open(file_path)?.to_rgba8();

        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.into_raw();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

        let texture = ctx.load_texture(asset_id, color_image, egui::TextureOptions::LINEAR);

        self.textures.insert(asset_id.to_string(), texture);

        Ok(())
    }

    pub fn resolve_image_for_node<const N: usize>(
        &mut self,
        ctx: &egui::Context,
        node_type: MapNodeType,
        name: &str,
        params: &Params<N>,
    ) -> Option<(egui::TextureHandle, f32)> {
        let def = self.image_objects.get(&(node_type, name.to_string()))?;

        let resolved = def.resolve(params)?;

        let asset_id = format!("{}/{}/{}", node_type.to_string(), name, resolved.image_path);

        let base_path = get_merino_folder_path().ok()?;

        let file_path = base_path
            .join("image")
            .join(node_type.to_string())
            .join(&resolved.image_path);

        let file_path = file_path.to_string_lossy();

        let _ = self.load_texture(ctx, &asset_id, &file_path);

        let texture = self.textures.get(&asset_id)?;

        Some((texture.clone(), resolved.rotation_degrees))
    }
}

#[derive(Default)]
pub struct CanvasContext {
    pub camera: CanvasCamera,
    pub selected_node_paths: Vec<NodePath>,
    pub target: Option<CanvasTarget>,
    pub node_edit_settings: EnumMap<MapNodeType, NodeEditSettings>,
    pub image_bank: ImageBank,
    // settings
    pub display_dummy_terrain: bool,
    pub display_squares_for_images: bool,
}

impl CanvasContext {
    pub fn can_view(&self, node_type: MapNodeType) -> bool {
        self.node_edit_settings[node_type].visible
    }

    pub fn can_edit(&self, node_type: MapNodeType) -> bool {
        self.node_edit_settings[node_type].editable
    }

    pub fn prune_invalid_selections(&mut self) {
        self.selected_node_paths.retain(|path| {
            let root_settings = &self.node_edit_settings[MapNodeType::MapSet];

            if !root_settings.visible || !root_settings.editable {
                return false;
            }

            // every node in the path must be visible + editable
            path.iter().all(|(child_type, _)| {
                let node_type = MapNodeType::from(*child_type);
                let settings = &self.node_edit_settings[node_type];

                settings.visible && settings.editable
            })
        });
    }
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

impl FileContext {
    pub fn move_node(&mut self, child: &NodePath, new_parent: &NodePath) {
        if child.is_empty() {
            // can't move root node
            return;
        }

        // prevent a node from moving into itself or its descendants
        if new_parent.starts_with(child) {
            return;
        }

        // final segment tells us what child list this node belongs to
        let (child_type, _) = *child.last().unwrap();

        // remove node
        let Some(node) = self.mapdata.remove_node_at_path(child) else {
            return;
        };

        // find new parent
        let Some(parent) = self.mapdata.get_node_at_path(new_parent) else {
            return;
        };

        // push node
        parent
            .children_of_type_vec_option_mut(child_type)
            .get_or_insert_with(Vec::new)
            .push(node);
    }
}

#[derive(Default)]
pub struct ObjectPropertyEditorContext {
    pub parameter_objects: Vec<ParameterObject>,
}

// impl ObjectPropertyEditorContext {
//     pub fn get_param_object(&self, node_type: MapNodeType, name: &str) -> Option<&ParameterObject> {
//         self.parameter_objects
//             .iter()
//             .find(|obj| obj.set_type == node_type && obj.name == name)
//     }
// }

// todo! save/load docking settings
#[derive(PartialEq)]
pub enum Tab {
    AddObject,
    Canvas,
    CanvasSettings,
    // NodeTree,
    ObjectProperties,
    SetObject,
}

struct TabViewer<'a> {
    editor: &'a mut LevelEditor,
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::AddObject => EmojiMessage::add_msg("Add Object"),
            Tab::Canvas => EmojiMessage::palette_msg("Canvas"),
            Tab::CanvasSettings => EmojiMessage::burger_msg("Canvas Settings"),
            Tab::ObjectProperties => EmojiMessage::memo_msg("Object Properties"),
            // Tab::NodeTree => EmojiMessage::folder_msg("Node Tree"),
            Tab::SetObject => EmojiMessage::target_msg("Set Object"),
        }
        .into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let file_open = self.editor.io_context.file_open;

        macro_rules! do_if_file_open {
            ($body:expr) => {
                if file_open {
                    $body
                } else {
                    ui.centered_and_justified(|ui| ui.label("No file open."));
                }
            };
        }

        match tab {
            Tab::AddObject => {
                do_if_file_open!({
                    self.editor.show_add_object(ui);
                });
            }

            Tab::Canvas => {
                do_if_file_open!({
                    self.editor.show_canvas(ui);
                });
            }

            Tab::CanvasSettings => {
                do_if_file_open!({
                    self.editor.show_canvas_settings(ui);
                });
            }
            // Tab::NodeTree => {
            //     if file_open {
            //         self.editor.show_node_tree(ui);
            //     } else {
            //         ui.centered_and_justified(|ui| ui.label("No file open."));
            //     }
            // }
            Tab::ObjectProperties => {
                if self.editor.canvas_context.selected_node_paths.len() == 1 {
                    let node_path = self.editor.canvas_context.selected_node_paths[0].clone();
                    self.editor.edit_node_properties(ui, node_path);
                } else {
                    ui.centered_and_justified(|ui| ui.label("Select exactly one object to edit."));
                }
            }

            Tab::SetObject => {
                do_if_file_open!({
                    self.editor.show_set_object(ui);
                })
            }
        }
    }
}

pub struct LevelEditor {
    // contexts
    pub io_context: IOContext,
    pub file_context: FileContext,
    pub canvas_context: CanvasContext,
    pub object_property_editor_context: ObjectPropertyEditorContext,
    pub editor_context: EditorContext,

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
            editor_context: Default::default(),
            dock_state,
        }
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        // top panel
        egui::TopBottomPanel::top("le_top_panel").show(ui.ctx(), |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // file submenu
                if ui.button("Open").clicked() {
                    if let Ok(opened) = self.open_file()
                        && opened
                    {
                        // clear any selections
                        self.canvas_context.selected_node_paths.clear();
                        self.canvas_context.target = None;
                        self.canvas_context.camera = CanvasCamera::default();
                        self.editor_context.commands.clear();
                    }
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

                if ui.button("Load Image Data").clicked() {
                    let _ = self.load_image_data();
                }

                ui.menu_button("Open Window", |ui| {
                    let items = [
                        (
                            EmojiMessage::add_msg("Add Object"),
                            Tab::AddObject,
                            "Add an object to the canvas.",
                        ),
                        (
                            EmojiMessage::palette_msg("Canvas"),
                            Tab::Canvas,
                            "View the canvas.",
                        ),
                        (
                            EmojiMessage::burger_msg("Canvas Settings"),
                            Tab::CanvasSettings,
                            "Edit canvas settings.",
                        ),
                        // (EmojiMessage::folder_msg("Node Tree"), Tab::NodeTree, "View a tree of every node in the file.")
                        (
                            EmojiMessage::memo_msg("Object Properties"),
                            Tab::ObjectProperties,
                            "Edit object properties.",
                        ),
                        (
                            EmojiMessage::target_msg("Set Object"),
                            Tab::SetObject,
                            "Make an object the child of an existing object.",
                        ),
                    ];

                    for (label, tab, hover_text) in items {
                        if ui.button(label).on_hover_text(hover_text).clicked() {
                            self.open_tab(tab);
                        }
                    }
                });
            });
        });

        self.process_commands();
        self.show_delete_confirmation(ui.ctx());

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
            .any(|node| node.tabs().is_some_and(|tabs| tabs.contains(&tab)))
        {
            self.dock_state.main_surface_mut().push_to_focused_leaf(tab);
        }
    }

    fn process_commands(&mut self) {
        if !self.editor_context.commands.is_empty() {
            // take all commands, leaving empty vec
            let commands = std::mem::take(&mut self.editor_context.commands);

            for command in commands {
                match command {
                    EditorCommand::MoveNode { child, new_parent } => {
                        self.file_context.move_node(&child, &new_parent);
                    }

                    EditorCommand::RemoveNode { path } => {
                        self.editor_context.pending_delete =
                            Some(DeleteConfirmation { path: path });
                    }

                    EditorCommand::SelectNode { path } => {
                        self.canvas_context.selected_node_paths.clear();
                        self.canvas_context.selected_node_paths.push(path);
                    }

                    EditorCommand::AddToSelection { path } => {
                        let paths = &mut self.canvas_context.selected_node_paths;

                        if !paths.contains(&path) {
                            paths.push(path);
                        }
                    }
                }
            }
        }
    }

    fn show_delete_confirmation(&mut self, ctx: &egui::Context) {
        let Some(delete) = &self.editor_context.pending_delete else {
            return;
        };

        let path = delete.path.clone();

        let has_children = self
            .file_context
            .mapdata
            .get_node_at_path(&path)
            .map_or(false, |node| node.has_children());

        let mut should_close = false;
        let mut confirmed = false;

        egui::Window::new("Confirm Deletion")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                if has_children {
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
            self.file_context.mapdata.remove_node_at_path(&path);

            self.canvas_context
                .selected_node_paths
                .retain(|p| p != &path);
        }

        if should_close {
            self.editor_context.pending_delete = None;
        }
    }
}
