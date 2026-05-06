use crate::merino::{
    game::mapbin::*,
    reader::{Readable, Reader},
    writer::{Writable, Writer},
};
use anyhow::{Result, anyhow};

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
                collision_type: reader.read_collision_type()?,
                unk3: reader.read_u32()?,
            },

            MapNodeType::MapObjSet => NodeData::MapObjSet {
                name: reader.read_object_type()?,
                position: reader.read_object::<Vec3f>()?,
                unk3: reader.read_f32()?,
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
                name: reader.read_item_type()?,
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
                name: reader.read_string(32)?,
                direction: reader.read_string(16)?,
                orientation: reader.read_string(16)?,
                position: reader.read_object::<Vec3f>()?,
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
                name: reader.read_string(64)?,
                position: reader.read_object::<Vec3f>()?,
                params: reader.read_object::<Params<3>>()?,
            },

            MapNodeType::MapPath => NodeData::MapPath {
                name: reader.read_string(64)?,
                points: reader.read_array(|r| r.read_object::<Vec2f>())?,
                params: reader.read_object::<Params<3>>()?,
            },

            MapNodeType::MapRect => NodeData::MapRect {
                name: reader.read_string(64)?,
                bounds_start: reader.read_object::<Vec2f>()?,
                bounds_end: reader.read_object::<Vec2f>()?,
                params: reader.read_object::<Params<3>>()?,
            },

            MapNodeType::MapCircle => NodeData::MapCircle {
                name: reader.read_string(64)?,
                position: reader.read_object::<Vec2f>()?,
                radius: reader.read_f32()?,
                params: reader.read_object::<Params<3>>()?,
            },

            MapNodeType::MapTerrain => NodeData::MapTerrain {
                collision_type: reader.read_collision_type()?,
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

impl Writable for Vec2f {
    fn write(&self, writer: &mut Writer, _: f32) -> Result<()> {
        writer.write_f32(self.x)?;
        writer.write_f32(self.y)?;
        Ok(())
    }
}

impl Writable for Vec3f {
    fn write(&self, writer: &mut Writer, _: f32) -> Result<()> {
        writer.write_f32(self.x)?;
        writer.write_f32(self.y)?;
        writer.write_f32(self.z)?;
        Ok(())
    }
}

impl<const N: usize> Writable for Params<N> {
    fn write(&self, writer: &mut Writer, _: f32) -> Result<()> {
        for v in self.int_values {
            writer.write_i32(v)?;
        }
        for v in self.float_values {
            writer.write_f32(v)?;
        }
        for v in &self.string_values {
            writer.write_string(v, 64)?;
        }
        Ok(())
    }
}
