use crate::merino::level_editor::{NodeChildType, NodePath};
use enum_map::Enum;
use std::fmt::Display;
use strum::{AsRefStr, Display, EnumString, FromRepr};

#[derive(
    FromRepr, Debug, Default, Display, AsRefStr, Copy, Clone, EnumString, PartialEq, Enum, Hash, Eq,
)]
#[repr(u32)]
pub enum MapNodeType {
    #[default]
    MapSet = 0,
    MapPolySet = 1,
    MapObjSet = 2,
    MapItemSet = 3,
    MapEnemySet = 4,
    MapLocator = 5,
    MapPath = 6,
    MapRect = 7,
    MapCircle = 8,
    MapTerrain = 9,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum MapNodeFlag {
    MapPolySet = 0x1,
    MapObjSet = 0x2,
    MapItemSet = 0x4,
    MapEnemySet = 0x8,
    MapLocator = 0x10,
    MapPath = 0x20,
    MapRect = 0x40,
    MapCircle = 0x80,
    MapTerrain = 0x100,
}

// a string with a char limit
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LimitedString<const N: usize>(pub String);

impl<const N: usize> LimitedString<N> {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<const N: usize> From<String> for LimitedString<N> {
    fn from(string: String) -> Self {
        Self(string)
    }
}

impl<const N: usize> Display for LimitedString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub type String16 = LimitedString<16>;
pub type String32 = LimitedString<32>;
pub type String64 = LimitedString<64>;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Params<const N: usize> {
    pub int_values: [i32; N],
    pub float_values: [f32; N],
    pub string_values: [String64; N],
}

impl<const N: usize> Default for Params<N> {
    fn default() -> Self {
        Self {
            int_values: [0; N],
            float_values: [0.0; N],
            string_values: std::array::from_fn(|_| Default::default()),
        }
    }
}

#[derive(Debug, Default)]
pub enum NodeData {
    #[default]
    None,
    MapSet {
        unk1: Option<i32>, // >= 4.70
        bounds_start: Vec2f,
        bounds_end: Vec2f,
    },

    MapPolySet {
        start: Vec2f,
        end: Vec2f,
        collision_normal: Vec2f,
        collision_type: String32,
        unk3: u32,
    },

    MapObjSet {
        name: String32,
        position: Vec3f,
        unk3: f32,
        unk4: Vec2f,
        unk5: String32,
        unk6: Option<i32>,      // >= 4.43
        unk7: Option<String32>, // >= 4.44
        unk8: Vec2f,
        unk9: Vec2f,
        unk10: Option<i32>, // >= 4.71
        unk11: Option<i32>, // >= 4.71
        unk12: Option<i32>, // >= 4.71
        unk13: Option<i32>, // >= 4.71
        params: Params<5>,
        unk14: Option<[[String32; 2]; 5]>, // >= 4.50
    },

    MapItemSet {
        name: String32,
        position: Vec3f,
        unk3: f32,
        unk4: Vec2f,
        unk5: String32,
        unk6: Option<i32>,      // version >= 4.43
        unk7: Option<String32>, // version >= 4.44
        unk8: Vec2f,
        unk9: Vec2f,
        unk10: Option<i32>, // version >= 4.71
        unk11: Option<i32>, // version >= 4.71
        unk12: Option<i32>, // version >= 4.71
        unk13: Option<i32>, // version >= 4.71
        params: Params<5>,
    },

    MapEnemySet {
        name: String32,
        direction: String16,
        orientation: String16,
        position: Vec3f,
        unk7: Option<String32>,  // version >= 4.45
        unk8: Option<String16>,  // version < 4.43
        unk9: Option<String16>,  // version < 4.43
        unk10: Option<String32>, // version < 4.43
        unk11: Option<i32>,      // version < 4.43
        unk12: Option<i32>,      // version < 4.43
        unk13: i32,
        unk14: Option<i32>,      // version >= 4.42
        unk15: Option<String32>, // version >= 4.44
        unk16: f32,
        unk17: f32,
        unk18: f32,
        unk19: f32,
        unk20: Option<i32>, // version >= 4.71
        unk21: Option<i32>, // version >= 4.71
        unk22: Option<i32>, // version >= 4.71
        unk23: Option<i32>, // version >= 4.71
        unk24: Option<i32>, // version >= 4.72
        params: Params<5>,
    },

    MapLocator {
        name: String64,
        position: Vec3f,
        params: Params<3>,
    },

    MapPath {
        name: String64,
        points: Vec<Vec2f>,
        params: Params<3>,
    },

    MapRect {
        name: String64,
        bounds_start: Vec2f,
        bounds_end: Vec2f,
        params: Params<3>,
    },

    MapCircle {
        name: String64,
        position: Vec2f,
        radius: f32,
        params: Params<3>,
    },

    MapTerrain {
        collision_type: String32,
        position: Vec3f,
        unk3: Option<i32>,      // version >= 4.43
        unk4: Option<String32>, // version >= 4.44
        unk5: f32,
        unk6: f32,
        unk7: f32,
        unk8: f32,
        unk9: Option<i32>,  // version >= 4.71
        unk10: Option<i32>, // version >= 4.71
        unk11: Option<i32>, // version >= 4.71
        unk12: Option<i32>, // version >= 4.71
        unk13: Vec<[Vec2f; 3]>,
        params: Params<3>,
        unk15: Option<[[String32; 2]; 3]>, // version >= 4.6
    },
}

impl NodeData {
    // pub fn default_mapset(version: f32) -> Self {
    //     Self::MapSet {
    //         unk1: (version >= 4.70).then_some(0),
    //         bounds_start: Default::default(),
    //         bounds_end: Default::default(),
    //     }
    // }

    pub fn default_mappolyset() -> Self {
        Self::MapPolySet {
            start: Default::default(),
            end: Default::default(),
            collision_normal: Default::default(),
            collision_type: Default::default(),
            unk3: 0,
        }
    }

    pub fn default_mapobjset(version: f32) -> Self {
        Self::MapObjSet {
            name: Default::default(),
            position: Default::default(),
            unk3: 0.0,
            unk4: Default::default(),
            unk5: Default::default(),
            unk6: (version >= 4.43).then_some(0),
            unk7: (version >= 4.44).then(String32::default),
            unk8: Default::default(),
            unk9: Default::default(),
            unk10: (version >= 4.71).then_some(0),
            unk11: (version >= 4.71).then_some(0),
            unk12: (version >= 4.71).then_some(0),
            unk13: (version >= 4.71).then_some(0),
            params: Default::default(),
            unk14: (version >= 4.50).then(Default::default),
        }
    }

    pub fn default_mapitemset(version: f32) -> Self {
        Self::MapItemSet {
            name: Default::default(),
            position: Default::default(),
            unk3: 0.0,
            unk4: Default::default(),
            unk5: Default::default(),
            unk6: (version >= 4.43).then_some(0),
            unk7: (version >= 4.44).then(String32::default),
            unk8: Default::default(),
            unk9: Default::default(),
            unk10: (version >= 4.71).then_some(0),
            unk11: (version >= 4.71).then_some(0),
            unk12: (version >= 4.71).then_some(0),
            unk13: (version >= 4.71).then_some(0),
            params: Default::default(),
        }
    }

    pub fn default_mapenemyset(version: f32) -> Self {
        Self::MapEnemySet {
            name: Default::default(),
            direction: Default::default(),
            orientation: Default::default(),
            position: Default::default(),
            unk7: (version >= 4.45).then(String32::default),
            unk8: (version < 4.43).then(String16::default),
            unk9: (version < 4.43).then(String16::default),
            unk10: (version < 4.43).then(String32::default),
            unk11: (version < 4.43).then_some(0),
            unk12: (version < 4.43).then_some(0),
            unk13: 0,
            unk14: (version >= 4.42).then_some(0),
            unk15: (version >= 4.44).then(String32::default),
            unk16: 0.0,
            unk17: 0.0,
            unk18: 0.0,
            unk19: 0.0,
            unk20: (version >= 4.71).then_some(0),
            unk21: (version >= 4.71).then_some(0),
            unk22: (version >= 4.71).then_some(0),
            unk23: (version >= 4.71).then_some(0),
            unk24: (version >= 4.72).then_some(0),
            params: Default::default(),
        }
    }

    pub fn default_maplocator() -> Self {
        Self::MapLocator {
            name: Default::default(),
            position: Default::default(),
            params: Default::default(),
        }
    }

    pub fn default_mappath() -> Self {
        Self::MapPath {
            name: Default::default(),
            points: Default::default(),
            params: Default::default(),
        }
    }

    pub fn default_maprect() -> Self {
        Self::MapRect {
            name: Default::default(),
            bounds_start: Default::default(),
            bounds_end: Default::default(),
            params: Default::default(),
        }
    }

    pub fn default_mapcircle() -> Self {
        Self::MapCircle {
            name: Default::default(),
            position: Default::default(),
            radius: 0.0,
            params: Default::default(),
        }
    }

    pub fn default_mapterrain(version: f32) -> Self {
        Self::MapTerrain {
            collision_type: Default::default(),
            position: Default::default(),
            unk3: (version >= 4.43).then_some(0),
            unk4: (version >= 4.44).then(String32::default),
            unk5: 0.0,
            unk6: 0.0,
            unk7: 0.0,
            unk8: 0.0,
            unk9: (version >= 4.71).then_some(0),
            unk10: (version >= 4.71).then_some(0),
            unk11: (version >= 4.71).then_some(0),
            unk12: (version >= 4.71).then_some(0),
            unk13: Default::default(),
            params: Default::default(),
            unk15: (version >= 4.6).then(Default::default),
        }
    }
}

#[derive(Debug, Default)]
pub struct MapDataNode {
    pub node_type: MapNodeType,
    pub node_data: NodeData,
    pub children_mappolyset: Option<Vec<MapDataNode>>, // MapPolySet
    pub children_mapobjset: Option<Vec<MapDataNode>>,  // MapObjSet
    pub children_mapitemset: Option<Vec<MapDataNode>>, // MapItemSet
    pub children_mapenemyset: Option<Vec<MapDataNode>>, // MapEnemySet
    pub children_maplocator: Option<Vec<MapDataNode>>, // MapLocator
    pub children_mappath: Option<Vec<MapDataNode>>,    // MapPath
    pub children_maprect: Option<Vec<MapDataNode>>,    // MapRect
    pub children_mapcircle: Option<Vec<MapDataNode>>,  // MapCircle
    pub children_mapterrain: Option<Vec<MapDataNode>>, // MapTerrain
}

impl MapDataNode {
    // pub fn remove_child(&mut self, child_type: NodeChildType, index: usize) -> Option<MapDataNode> {
    //     let vec = self.children_of_type_vec_mut(child_type)?;
    //     if index < vec.len() {
    //         Some(vec.remove(index))
    //     } else {
    //         None
    //     }
    // }

    pub fn find_node_mut(&mut self, path: &NodePath) -> Option<&mut MapDataNode> {
        let mut node = self;

        for (child_type, index) in path {
            node = node
                .children_of_type_vec_mut(*child_type)?
                .get_mut(*index)?;
        }

        Some(node)
    }

    fn collect_strings(
        &self,
        object_types: &mut Vec<String32>,
        item_types: &mut Vec<String32>,
        collision_types: &mut Vec<String32>,
        enemy_types: &mut Vec<String32>,
    ) {
        match &self.node_data {
            NodeData::MapPolySet { collision_type, .. } => {
                if !collision_types.contains(collision_type) {
                    collision_types.push(collision_type.clone());
                }
            }
            NodeData::MapObjSet { name, .. } => {
                if !object_types.contains(name) {
                    object_types.push(name.clone());
                }
            }
            NodeData::MapItemSet { name, .. } => {
                if !item_types.contains(name) {
                    item_types.push(name.clone());
                }
            }
            NodeData::MapEnemySet { name, .. } => {
                if !enemy_types.contains(name) {
                    enemy_types.push(name.clone());
                }
            }
            NodeData::MapTerrain { collision_type, .. } => {
                if !collision_types.contains(collision_type) {
                    collision_types.push(collision_type.clone());
                }
            }
            _ => {}
        }

        // do the same for children
        for child in self.available_children() {
            child.collect_strings(object_types, item_types, collision_types, enemy_types);
        }
    }

    pub fn available_children(&self) -> impl Iterator<Item = &MapDataNode> {
        let children = [
            &self.children_mappolyset,
            &self.children_mapobjset,
            &self.children_mapitemset,
            &self.children_mapenemyset,
            &self.children_maplocator,
            &self.children_mappath,
            &self.children_maprect,
            &self.children_mapcircle,
            &self.children_mapterrain,
        ];

        children.into_iter().flatten().flatten()
    }

    // pub fn available_children_mut(&mut self) -> impl Iterator<Item = &mut MapDataNode> {
    //     let children = [
    //         &mut self.children_mappolyset,
    //         &mut self.children_mapobjset,
    //         &mut self.children_mapitemset,
    //         &mut self.children_mapenemyset,
    //         &mut self.children_maplocator,
    //         &mut self.children_mappath,
    //         &mut self.children_maprect,
    //         &mut self.children_mapcircle,
    //         &mut self.children_mapterrain,
    //     ];

    //     children.into_iter().flatten().flatten()
    // }

    /// Returns an iterator over (folder, index, child)
    pub fn all_children_mut(
        &mut self,
    ) -> impl Iterator<Item = (NodeChildType, usize, &mut MapDataNode)> {
        let mut items = Vec::new();

        // helper macro to reduce boilerplate
        macro_rules! collect_sub {
            ($child_type:ident, $field:ident) => {
                if let Some(vec) = &mut self.$field {
                    for (i, node) in vec.iter_mut().enumerate() {
                        items.push((NodeChildType::$child_type, i, node));
                    }
                }
            };
        }

        collect_sub!(MapPolySet, children_mappolyset);
        collect_sub!(MapObjSet, children_mapobjset);
        collect_sub!(MapItemSet, children_mapitemset);
        collect_sub!(MapEnemySet, children_mapenemyset);
        collect_sub!(MapLocator, children_maplocator);
        collect_sub!(MapPath, children_mappath);
        collect_sub!(MapRect, children_maprect);
        collect_sub!(MapCircle, children_mapcircle);
        collect_sub!(MapTerrain, children_mapterrain);

        items.into_iter()
    }

    pub fn has_children(&self) -> bool {
        self.available_children().next().is_some()
    }

    pub fn has_child_of_type(&self, child_type: NodeChildType) -> bool {
        let list = match child_type {
            NodeChildType::MapPolySet => &self.children_mappolyset,
            NodeChildType::MapObjSet => &self.children_mapobjset,
            NodeChildType::MapItemSet => &self.children_mapitemset,
            NodeChildType::MapEnemySet => &self.children_mapenemyset,
            NodeChildType::MapLocator => &self.children_maplocator,
            NodeChildType::MapPath => &self.children_mappath,
            NodeChildType::MapRect => &self.children_maprect,
            NodeChildType::MapCircle => &self.children_mapcircle,
            NodeChildType::MapTerrain => &self.children_mapterrain,
        };

        list.as_ref().is_some_and(|children| !children.is_empty())
    }

    pub fn children_of_type_mut(
        &mut self,
        child_type: NodeChildType,
    ) -> impl Iterator<Item = &mut MapDataNode> {
        let list = match child_type {
            NodeChildType::MapPolySet => &mut self.children_mappolyset,
            NodeChildType::MapObjSet => &mut self.children_mapobjset,
            NodeChildType::MapItemSet => &mut self.children_mapitemset,
            NodeChildType::MapEnemySet => &mut self.children_mapenemyset,
            NodeChildType::MapLocator => &mut self.children_maplocator,
            NodeChildType::MapPath => &mut self.children_mappath,
            NodeChildType::MapRect => &mut self.children_maprect,
            NodeChildType::MapCircle => &mut self.children_mapcircle,
            NodeChildType::MapTerrain => &mut self.children_mapterrain,
        };

        list.as_mut().into_iter().flatten()
    }

    pub fn children_of_type_vec_option_mut(
        &mut self,
        child_type: NodeChildType,
    ) -> &mut Option<Vec<MapDataNode>> {
        match child_type {
            NodeChildType::MapPolySet => &mut self.children_mappolyset,
            NodeChildType::MapObjSet => &mut self.children_mapobjset,
            NodeChildType::MapItemSet => &mut self.children_mapitemset,
            NodeChildType::MapEnemySet => &mut self.children_mapenemyset,
            NodeChildType::MapLocator => &mut self.children_maplocator,
            NodeChildType::MapPath => &mut self.children_mappath,
            NodeChildType::MapRect => &mut self.children_maprect,
            NodeChildType::MapCircle => &mut self.children_mapcircle,
            NodeChildType::MapTerrain => &mut self.children_mapterrain,
        }
    }

    pub fn children_of_type_vec_mut(
        &mut self,
        child_type: NodeChildType,
    ) -> Option<&mut Vec<MapDataNode>> {
        match child_type {
            NodeChildType::MapPolySet => self.children_mappolyset.as_mut(),
            NodeChildType::MapObjSet => self.children_mapobjset.as_mut(),
            NodeChildType::MapItemSet => self.children_mapitemset.as_mut(),
            NodeChildType::MapEnemySet => self.children_mapenemyset.as_mut(),
            NodeChildType::MapLocator => self.children_maplocator.as_mut(),
            NodeChildType::MapPath => self.children_mappath.as_mut(),
            NodeChildType::MapRect => self.children_maprect.as_mut(),
            NodeChildType::MapCircle => self.children_mapcircle.as_mut(),
            NodeChildType::MapTerrain => self.children_mapterrain.as_mut(),
        }
    }
}

#[derive(Default)]
pub struct Mapbin {
    pub version: f32,
    pub object_types: Vec<String32>,
    pub item_types: Vec<String32>,
    pub collision_types: Vec<String32>,
    pub rect_types: Vec<String32>,
    pub enemy_types: Vec<String32>,
    pub unk_types_1: Vec<String32>,
    pub root: MapDataNode,
}

impl Mapbin {
    pub fn collect_new_strings(&mut self) {
        self.root.collect_strings(
            &mut self.object_types,
            &mut self.item_types,
            &mut self.collision_types,
            &mut self.enemy_types,
        );
    }

    pub fn get_node_at_path(&mut self, path: &NodePath) -> Option<&mut MapDataNode> {
        let mut current = &mut self.root;

        for (branch, index) in path {
            let vec = match branch {
                NodeChildType::MapPolySet => &mut current.children_mappolyset,
                NodeChildType::MapObjSet => &mut current.children_mapobjset,
                NodeChildType::MapItemSet => &mut current.children_mapitemset,
                NodeChildType::MapEnemySet => &mut current.children_mapenemyset,
                NodeChildType::MapLocator => &mut current.children_maplocator,
                NodeChildType::MapPath => &mut current.children_mappath,
                NodeChildType::MapRect => &mut current.children_maprect,
                NodeChildType::MapCircle => &mut current.children_mapcircle,
                NodeChildType::MapTerrain => &mut current.children_mapterrain,
            };

            current = vec.as_mut()?.get_mut(*index)?;
        }

        Some(current)
    }

    pub fn remove_node_at_path(&mut self, path: &NodePath) -> Option<MapDataNode> {
        if path.is_empty() {
            return None;
        }

        let mut parent_path = path.clone();
        let (branch, index) = parent_path.pop()?;

        // get the parent of the node we want to kill
        let parent = self.get_node_at_path(&parent_path)?;

        let vec = match branch {
            NodeChildType::MapPolySet => &mut parent.children_mappolyset,
            NodeChildType::MapObjSet => &mut parent.children_mapobjset,
            NodeChildType::MapItemSet => &mut parent.children_mapitemset,
            NodeChildType::MapEnemySet => &mut parent.children_mapenemyset,
            NodeChildType::MapLocator => &mut parent.children_maplocator,
            NodeChildType::MapPath => &mut parent.children_mappath,
            NodeChildType::MapRect => &mut parent.children_maprect,
            NodeChildType::MapCircle => &mut parent.children_mapcircle,
            NodeChildType::MapTerrain => &mut parent.children_mapterrain,
        };

        if let Some(v) = vec
            && index < v.len()
        {
            return Some(v.remove(index));
        }
        None
    }
}
