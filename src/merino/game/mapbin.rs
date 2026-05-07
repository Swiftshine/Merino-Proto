use crate::merino::level_editor::{NodeBranch, NodePath};
use std::fmt::Display;
use strum::{AsRefStr, Display, EnumString, FromRepr};

#[derive(FromRepr, Debug, Default, Display, AsRefStr, Copy, Clone, EnumString, PartialEq)]
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

#[derive(Debug, Clone, Copy)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
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
        unk1: Vec2f,
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
        unk2: Vec2f,
        unk3: Vec2f,
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
        unk2: f32,
        unk3: f32,
        unk4: f32,
        unk5: Option<i32>,      // version >= 4.43
        unk6: Option<String32>, // version >= 4.44
        unk7: f32,
        unk8: f32,
        unk9: f32,
        unk10: f32,
        unk11: Option<i32>, // version >= 4.71
        unk12: Option<i32>, // version >= 4.71
        unk13: Option<i32>, // version >= 4.71
        unk14: Option<i32>, // version >= 4.71
        unk15: Vec<[Vec2f; 3]>,
        params: Params<3>,
        unk16: Option<[[String32; 2]; 3]>, // version >= 4.6
    },
}

#[derive(Default)]
pub struct MapDataNode {
    pub node_type: MapNodeType,
    pub node_data: NodeData,
    pub sub1: Option<Vec<MapDataNode>>,
    pub sub2: Option<Vec<MapDataNode>>,
    pub sub4: Option<Vec<MapDataNode>>,
    pub sub8: Option<Vec<MapDataNode>>,
    pub sub10: Option<Vec<MapDataNode>>,
    pub sub20: Option<Vec<MapDataNode>>,
    pub sub40: Option<Vec<MapDataNode>>,
    pub sub80: Option<Vec<MapDataNode>>,
    pub sub100: Option<Vec<MapDataNode>>,
}

impl MapDataNode {
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
        let subs = [
            &self.sub1,
            &self.sub2,
            &self.sub4,
            &self.sub8,
            &self.sub10,
            &self.sub20,
            &self.sub40,
            &self.sub80,
            &self.sub100,
        ];

        subs.into_iter().flatten().flatten()
    }

    // pub fn available_children_mut(&mut self) -> impl Iterator<Item = &mut MapDataNode> {
    //     let subs = [
    //         &mut self.sub1,
    //         &mut self.sub2,
    //         &mut self.sub4,
    //         &mut self.sub8,
    //         &mut self.sub10,
    //         &mut self.sub20,
    //         &mut self.sub40,
    //         &mut self.sub80,
    //         &mut self.sub100,
    //     ];

    //     subs.into_iter().flatten().flatten()
    // }

    /// Returns an iterator over (folder, index, child)
    pub fn all_children_mut(
        &mut self,
    ) -> impl Iterator<Item = (NodeBranch, usize, &mut MapDataNode)> {
        let mut items = Vec::new();

        // helper macro to reduce boilerplate
        macro_rules! collect_sub {
            ($branch:ident, $field:ident) => {
                if let Some(vec) = &mut self.$field {
                    for (i, node) in vec.iter_mut().enumerate() {
                        items.push((NodeBranch::$branch, i, node));
                    }
                }
            };
        }

        collect_sub!(Sub1, sub1);
        collect_sub!(Sub2, sub2);
        collect_sub!(Sub4, sub4);
        collect_sub!(Sub8, sub8);
        collect_sub!(Sub10, sub10);
        collect_sub!(Sub20, sub20);
        collect_sub!(Sub40, sub40);
        collect_sub!(Sub80, sub80);
        collect_sub!(Sub100, sub100);

        items.into_iter()
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

    pub fn get_node_at_path(&mut self, path: NodePath) -> Option<&mut MapDataNode> {
        let mut current = &mut self.root;

        for (branch, index) in path {
            let vec = match branch {
                NodeBranch::Sub1 => &mut current.sub1,
                NodeBranch::Sub2 => &mut current.sub2,
                NodeBranch::Sub4 => &mut current.sub4,
                NodeBranch::Sub8 => &mut current.sub8,
                NodeBranch::Sub10 => &mut current.sub10,
                NodeBranch::Sub20 => &mut current.sub20,
                NodeBranch::Sub40 => &mut current.sub40,
                NodeBranch::Sub80 => &mut current.sub80,
                NodeBranch::Sub100 => &mut current.sub100,
            };

            current = vec.as_mut()?.get_mut(index)?;
        }

        Some(current)
    }
}
