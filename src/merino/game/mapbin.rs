use crate::merino::reader::{Readable, Reader};
use anyhow::{Result, anyhow};
use strum::FromRepr;

#[derive(FromRepr, Debug)]
#[repr(u32)]
pub enum MapNodeType {
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

#[derive(Debug)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Readable for Vec2f {
    fn read(reader: &mut Reader) -> Result<Self>
    where
        Self: Sized,
    {
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;

        Ok(Self { x, y })
    }
}

#[derive(Debug)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Readable for Vec3f {
    fn read(reader: &mut Reader) -> Result<Self>
    where
        Self: Sized,
    {
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;

        Ok(Self { x, y, z })
    }
}

#[derive(Debug)]
pub struct Params<const N: usize> {
    pub int_values: [i32; N],
    pub float_values: [f32; N],
    pub string_values: [String; N],
}

impl<const N: usize> Readable for Params<N> {
    fn read(reader: &mut Reader) -> Result<Self> {
        let mut int_values = [0i32; N];
        for i in 0..N {
            int_values[i] = reader.read_i32()?;
        }

        let mut float_values = [0.0f32; N];
        for i in 0..N {
            float_values[i] = reader.read_f32()?;
        }

        let mut string_values = std::array::from_fn(|_| String::new());
        for i in 0..N {
            string_values[i] = reader.read_string(64)?;
        }

        Ok(Self {
            int_values,
            float_values,
            string_values,
        })
    }
}

#[derive(Debug)]
pub enum NodeData {
    MapSet {
        unk1: Option<i32>, // >= 4.70
        bounds_start: Vec2f,
        bounds_end: Vec2f,
    },

    MapPolySet {
        start: Vec2f,
        end: Vec2f,
        unk1: Vec2f,
        unk2: u32,
        unk3: u32,
    },

    MapObjSet {
        unk1: i32,
        unk2: Vec2f,
        unk3: Vec2f,
        unk4: Vec2f,
        unk5: String,
        unk6: Option<i32>,    // >= 4.43
        unk7: Option<String>, // >= 4.44
        unk8: Vec2f,
        unk9: Vec2f,
        unk10: Option<i32>, // >= 4.71
        unk11: Option<i32>, // >= 4.71
        unk12: Option<i32>, // >= 4.71
        unk13: Option<i32>, // >= 4.71
        params: Params<5>,
        unk14: Option<[[String; 2]; 5]>, // >= 4.50
    },

    MapItemSet {
        unk1: i32,
        unk2: Vec2f,
        unk3: Vec2f,
        unk4: Vec2f,
        unk5: String,
        unk6: Option<i32>,    // version >= 4.43
        unk7: Option<String>, // version >= 4.44
        unk8: Vec2f,
        unk9: Vec2f,
        unk10: Option<i32>, // version >= 4.71
        unk11: Option<i32>, // version >= 4.71
        unk12: Option<i32>, // version >= 4.71
        unk13: Option<i32>, // version >= 4.71
        params: Params<5>,
    },

    MapEnemySet {
        unk1: String,
        unk2: String,
        unk3: String,
        unk4: f32,
        unk5: f32,
        unk6: f32,
        unk7: Option<String>,  // version >= 4.45
        unk8: Option<String>,  // version < 4.43
        unk9: Option<String>,  // version < 4.43
        unk10: Option<String>, // version < 4.43
        unk11: Option<i32>,    // version < 4.43
        unk12: Option<i32>,    // version < 4.43
        unk13: i32,
        unk14: Option<i32>,    // version >= 4.42
        unk15: Option<String>, // version >= 4.44
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
        unk1: String,
        position: Vec3f,
        params: Params<3>,
    },

    MapPath {
        unk1: String,
        points: Vec<Vec2f>,
        params: Params<3>,
    },

    MapRect {
        unk1: String,
        bounds_start: Vec2f,
        bounds_end: Vec2f,
        params: Params<3>,
    },

    MapCircle {
        unk1: String,
        position: Vec2f,
        radius: f32,
        params: Params<3>,
    },

    MapTerrain {
        unk1: i32,
        unk2: f32,
        unk3: f32,
        unk4: f32,
        unk5: Option<i32>,    // version >= 4.43
        unk6: Option<String>, // version >= 4.44
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
        unk16: Option<[[String; 2]; 3]>, // version >= 4.6
    },
}

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

pub struct Mapbin {
    pub gimmick_types: Vec<String>,
    pub collectible_types: Vec<String>,
    pub collision_types: Vec<String>,
    pub rect_types: Vec<String>,
    pub enemy_types: Vec<String>,
    pub unk_types_1: Vec<String>,
    pub root: MapDataNode,
}

impl MapDataNode {
    pub fn read(reader: &mut Reader) -> Result<Self> {
        let node_type_raw = reader.read_u32()?;
        let node_type = MapNodeType::from_repr(node_type_raw)
            .ok_or_else(|| anyhow!("invalid node type, found {node_type_raw}"))?;

        let node_data = match node_type {
            MapNodeType::MapSet => NodeData::MapSet {
                unk1: reader.read_at_version(4.70, |r| r.read_i32())?,
                bounds_start: reader.read_object::<Vec2f>()?,
                bounds_end: reader.read_object::<Vec2f>()?,
            },

            MapNodeType::MapPolySet => NodeData::MapPolySet {
                start: reader.read_object::<Vec2f>()?,
                end: reader.read_object::<Vec2f>()?,
                unk1: reader.read_object::<Vec2f>()?,
                unk2: reader.read_u32()?,
                unk3: reader.read_u32()?,
            },

            MapNodeType::MapObjSet => NodeData::MapObjSet {
                unk1: reader.read_i32()?,
                unk2: reader.read_object::<Vec2f>()?,
                unk3: reader.read_object::<Vec2f>()?,
                unk4: reader.read_object::<Vec2f>()?,
                unk5: reader.read_string(32)?,
                unk6: reader.read_at_version(4.43, |r| r.read_i32())?,
                unk7: reader.read_at_version(4.44, |r| r.read_string(32))?,
                unk8: reader.read_object::<Vec2f>()?,
                unk9: reader.read_object::<Vec2f>()?,
                unk10: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk11: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk12: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk13: reader.read_at_version(4.71, |r| r.read_i32())?,
                params: reader.read_object::<Params<5>>()?,
                unk14: reader.read_at_version(4.50, |r| {
                    let mut outer = std::array::from_fn(|_| [String::new(), String::new()]);
                    for i in 0..5 {
                        outer[i] = [r.read_string(32)?, r.read_string(32)?];
                    }
                    Ok(outer)
                })?,
            },

            MapNodeType::MapItemSet => NodeData::MapItemSet {
                unk1: reader.read_i32()?,
                unk2: reader.read_object::<Vec2f>()?,
                unk3: reader.read_object::<Vec2f>()?,
                unk4: reader.read_object::<Vec2f>()?,
                unk5: reader.read_string(32)?,
                unk6: reader.read_at_version(4.43, |r| r.read_i32())?,
                unk7: reader.read_at_version(4.44, |r| r.read_string(32))?,
                unk8: reader.read_object::<Vec2f>()?,
                unk9: reader.read_object::<Vec2f>()?,
                unk10: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk11: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk12: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk13: reader.read_at_version(4.71, |r| r.read_i32())?,
                params: reader.read_object::<Params<5>>()?,
            },

            MapNodeType::MapEnemySet => NodeData::MapEnemySet {
                unk1: reader.read_string(32)?,
                unk2: reader.read_string(16)?,
                unk3: reader.read_string(16)?,
                unk4: reader.read_f32()?,
                unk5: reader.read_f32()?,
                unk6: reader.read_f32()?,
                unk7: reader.read_at_version(4.45, |r| r.read_string(32))?,
                unk8: reader.read_below_version(4.43, |r| r.read_string(16))?,
                unk9: reader.read_below_version(4.43, |r| r.read_string(16))?,
                unk10: reader.read_below_version(4.43, |r| r.read_string(32))?,
                unk11: reader.read_below_version(4.43, |r| r.read_i32())?,
                unk12: reader.read_below_version(4.43, |r| r.read_i32())?,
                unk13: reader.read_i32()?,
                unk14: reader.read_at_version(4.42, |r| r.read_i32())?,
                unk15: reader.read_at_version(4.44, |r| r.read_string(32))?,
                unk16: reader.read_f32()?,
                unk17: reader.read_f32()?,
                unk18: reader.read_f32()?,
                unk19: reader.read_f32()?,
                unk20: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk21: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk22: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk23: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk24: reader.read_at_version(4.72, |r| r.read_i32())?,
                params: reader.read_object::<Params<5>>()?,
            },

            MapNodeType::MapLocator => NodeData::MapLocator {
                unk1: reader.read_string(64)?,
                position: reader.read_object::<Vec3f>()?,
                params: reader.read_object::<Params<3>>()?,
            },

            MapNodeType::MapPath => NodeData::MapPath {
                unk1: reader.read_string(64)?,
                points: reader.read_array(|r| r.read_object::<Vec2f>())?,
                params: reader.read_object::<Params<3>>()?,
            },

            MapNodeType::MapRect => NodeData::MapRect {
                unk1: reader.read_string(64)?,
                bounds_start: reader.read_object::<Vec2f>()?,
                bounds_end: reader.read_object::<Vec2f>()?,
                params: reader.read_object::<Params<3>>()?,
            },

            MapNodeType::MapCircle => NodeData::MapCircle {
                unk1: reader.read_string(64)?,
                position: reader.read_object::<Vec2f>()?,
                radius: reader.read_f32()?,
                params: reader.read_object::<Params<3>>()?,
            },

            MapNodeType::MapTerrain => NodeData::MapTerrain {
                unk1: reader.read_i32()?,
                unk2: reader.read_f32()?,
                unk3: reader.read_f32()?,
                unk4: reader.read_f32()?,
                unk5: reader.read_at_version(4.43, |r| r.read_i32())?,
                unk6: reader.read_at_version(4.44, |r| r.read_string(32))?,
                unk7: reader.read_f32()?,
                unk8: reader.read_f32()?,
                unk9: reader.read_f32()?,
                unk10: reader.read_f32()?,
                unk11: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk12: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk13: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk14: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk15: reader.read_array(|r| {
                    Ok([
                        r.read_object::<Vec2f>()?,
                        r.read_object::<Vec2f>()?,
                        r.read_object::<Vec2f>()?,
                    ])
                })?,
                params: reader.read_object::<Params<3>>()?,
                unk16: reader.read_at_version(4.6, |r| {
                    let mut outer = std::array::from_fn(|_| [String::new(), String::new()]);
                    for i in 0..3 {
                        outer[i] = [r.read_string(32)?, r.read_string(32)?];
                    }
                    Ok(outer)
                })?,
            },
            _ => todo!(),
        };

        let flags = reader.read_u32()?;

        // helper to read a list of sub-nodes if flag present
        let mut read_sub_node = |flag: u32| -> Result<Option<Vec<MapDataNode>>> {
            if (flags & flag) != 0 {
                let nodes = reader.read_array(|r| Self::read(r))?;
                Ok(Some(nodes))
            } else {
                Ok(None)
            }
        };

        Ok(MapDataNode {
            node_type,
            node_data,
            sub1: read_sub_node(0x1)?,
            sub2: read_sub_node(0x2)?,
            sub4: read_sub_node(0x4)?,
            sub8: read_sub_node(0x8)?,
            sub10: read_sub_node(0x10)?,
            sub20: read_sub_node(0x20)?,
            sub40: read_sub_node(0x40)?,
            sub80: read_sub_node(0x80)?,
            sub100: read_sub_node(0x100)?,
        })
    }
}
