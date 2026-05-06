use crate::merino::game::mapbin::MapDataNode;
use crate::merino::game::mapbin::Mapbin;
use crate::merino::game::mapbin::NodeData;
use crate::merino::game::mapbin::Params;
use crate::merino::game::mapbin::Vec2f;
use crate::merino::game::mapbin::Vec3f;
use anyhow::Result;
use anyhow::anyhow;
use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

const PLACEHOLDER_VALUE: u32 = 0xDEADCAFE;

pub trait Writable {
    fn write(&self, writer: &mut Writer, version: f32) -> Result<()>;
}

pub struct Writer {
    pub buffer: Vec<u8>,
}

impl Writer {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    // primitives

    fn write_i32(&mut self, val: i32) -> Result<()> {
        self.buffer.write_i32::<BigEndian>(val)?;
        Ok(())
    }

    fn write_u32(&mut self, val: u32) -> Result<()> {
        self.buffer.write_u32::<BigEndian>(val)?;
        Ok(())
    }

    fn write_f32(&mut self, val: f32) -> Result<()> {
        self.buffer.write_f32::<BigEndian>(val)?;
        Ok(())
    }

    fn write_string(&mut self, string: &String, size: usize) -> Result<()> {
        let bytes = string.as_bytes();

        let len = bytes.len().min(size);

        self.buffer.write_all(&bytes[..len])?;

        for _ in 0..(size - len) {
            self.buffer.write_u8(0)?;
        }

        Ok(())
    }

    // util
    fn get_index_of(&self, list: &[String], value: &str, label: &str) -> Result<u32> {
        list.iter()
            .position(|s| s == value)
            .map(|i| i as u32)
            .ok_or_else(|| anyhow!("{} '{}' not found in string table", label, value))
    }

    fn write_at_version<T, F>(
        &mut self,
        version: f32,
        min: f32,
        val: &Option<T>,
        f: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self, &T) -> Result<()>,
    {
        if version >= min {
            if let Some(v) = val {
                f(self, v)?;
            } else {
                return Err(anyhow!("Missing required versioned field (>= {})", min));
            }
        }
        Ok(())
    }

    fn write_below_version<T, F>(
        &mut self,
        version: f32,
        max: f32,
        val: &Option<T>,
        f: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self, &T) -> Result<()>,
    {
        if version < max {
            if let Some(v) = val {
                f(self, v)?;
            } else {
                return Err(anyhow!("Missing required versioned field (< {})", max));
            }
        }
        Ok(())
    }
    // custom

    fn write_node(&mut self, node: &MapDataNode, mapbin: &Mapbin, version: f32) -> Result<()> {
        // type
        self.write_u32(node.node_type as u32)?;

        // write data
        match &node.node_data {
            NodeData::MapSet {
                unk1,
                bounds_start,
                bounds_end,
            } => {
                self.write_at_version(version, 4.70, unk1, |w, v| w.write_i32(*v))?;
                bounds_start.write(self, version)?;
                bounds_end.write(self, version)?;
            }

            NodeData::MapPolySet {
                start,
                end,
                unk1,
                collision_type,
                unk3,
            } => {
                start.write(self, version)?;
                end.write(self, version)?;
                unk1.write(self, version)?;
                let index =
                    self.get_index_of(&mapbin.collision_types, collision_type, "Collision")?;
                self.write_u32(index)?;
                self.write_u32(*unk3)?;
            }

            NodeData::MapObjSet {
                name,
                position,
                unk3,
                unk4,
                unk5,
                unk6,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                unk13,
                params,
                unk14,
            } => {
                let index = self.get_index_of(&mapbin.object_types, name, "Object")?;
                self.write_u32(index)?;
                position.write(self, version)?;
                self.write_f32(*unk3)?;
                unk4.write(self, version)?;
                self.write_string(unk5, 32)?;
                self.write_at_version(version, 4.43, unk6, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.44, unk7, |w, v| w.write_string(v, 32))?;
                unk8.write(self, version)?;
                unk9.write(self, version)?;
                self.write_at_version(version, 4.71, unk10, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.71, unk11, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.71, unk12, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.71, unk13, |w, v| w.write_i32(*v))?;
                params.write(self, version)?;
                self.write_at_version(version, 4.50, unk14, |w, v| {
                    for pair in v {
                        w.write_string(&pair[0], 32)?;
                        w.write_string(&pair[1], 32)?;
                    }
                    Ok(())
                })?;
            }

            NodeData::MapItemSet {
                name,
                unk2,
                unk3,
                unk4,
                unk5,
                unk6,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                unk13,
                params,
            } => {
                let index = self.get_index_of(&mapbin.item_types, name, "Item")?;
                self.write_u32(index)?;
                unk2.write(self, version)?;
                unk3.write(self, version)?;
                unk4.write(self, version)?;
                self.write_string(unk5, 32)?;
                self.write_at_version(version, 4.43, unk6, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.44, unk7, |w, v| w.write_string(v, 32))?;
                unk8.write(self, version)?;
                unk9.write(self, version)?;
                self.write_at_version(version, 4.71, unk10, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.71, unk11, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.71, unk12, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.71, unk13, |w, v| w.write_i32(*v))?;
                params.write(self, version)?;
            }

            NodeData::MapEnemySet {
                name,
                direction,
                orientation,
                position,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                unk13,
                unk14,
                unk15,
                unk16,
                unk17,
                unk18,
                unk19,
                unk20,
                unk21,
                unk22,
                unk23,
                unk24,
                params,
            } => {
                self.write_string(name, 32)?;
                self.write_string(direction, 16)?;
                self.write_string(orientation, 16)?;
                position.write(self, version)?;
                self.write_at_version(version, 4.45, unk7, |w, v| w.write_string(v, 32))?;
                self.write_below_version(version, 4.43, unk8, |w, v| w.write_string(v, 16))?;
                self.write_below_version(version, 4.43, unk9, |w, v| w.write_string(v, 16))?;
                self.write_below_version(version, 4.43, unk10, |w, v| w.write_string(v, 32))?;
                self.write_below_version(version, 4.43, unk11, |w, v| w.write_i32(*v))?;
                self.write_below_version(version, 4.43, unk12, |w, v| w.write_i32(*v))?;
                self.write_i32(*unk13)?;
                self.write_at_version(version, 4.42, unk14, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.44, unk15, |w, v| w.write_string(v, 32))?;
                self.write_f32(*unk16)?;
                self.write_f32(*unk17)?;
                self.write_f32(*unk18)?;
                self.write_f32(*unk19)?;
                self.write_at_version(version, 4.71, unk20, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.71, unk21, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.71, unk22, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.71, unk23, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.72, unk24, |w, v| w.write_i32(*v))?;
                params.write(self, version)?;
            }
            NodeData::MapLocator {
                name,
                position,
                params,
            } => {
                self.write_string(name, 64)?;
                position.write(self, version)?;
                params.write(self, version)?;
            }

            NodeData::MapPath {
                name,
                points,
                params,
            } => {
                self.write_string(name, 64)?;
                self.write_u32(points.len() as u32)?;
                for p in points {
                    p.write(self, version)?;
                }
                params.write(self, version)?;
            }

            NodeData::MapRect {
                name,
                bounds_start,
                bounds_end,
                params,
            } => {
                self.write_string(name, 64)?;
                bounds_start.write(self, version)?;
                bounds_end.write(self, version)?;
                params.write(self, version)?;
            }

            NodeData::MapCircle {
                name,
                position,
                radius,
                params,
            } => {
                self.write_string(name, 64)?;
                position.write(self, version)?;
                self.write_f32(*radius)?;
                params.write(self, version)?;
            }

            NodeData::MapTerrain {
                collision_type,
                unk2,
                unk3,
                unk4,
                unk5,
                unk6,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                unk13,
                unk14,
                unk15,
                params,
                unk16,
            } => {
                let index =
                    self.get_index_of(&mapbin.collision_types, collision_type, "Collision")?;
                self.write_u32(index)?;
                self.write_f32(*unk2)?;
                self.write_f32(*unk3)?;
                self.write_f32(*unk4)?;
                self.write_at_version(version, 4.43, unk5, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.44, unk6, |w, v| w.write_string(v, 32))?;
                self.write_f32(*unk7)?;
                self.write_f32(*unk8)?;
                self.write_f32(*unk9)?;
                self.write_f32(*unk10)?;
                self.write_at_version(version, 4.71, unk11, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.71, unk12, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.71, unk13, |w, v| w.write_i32(*v))?;
                self.write_at_version(version, 4.71, unk14, |w, v| w.write_i32(*v))?;
                self.write_u32(unk15.len() as u32)?;
                for triangle in unk15 {
                    triangle[0].write(self, version)?;
                    triangle[1].write(self, version)?;
                    triangle[2].write(self, version)?;
                }
                params.write(self, version)?;
                self.write_at_version(version, 4.6, unk16, |w, v| {
                    for pair in v {
                        w.write_string(&pair[0], 32)?;
                        w.write_string(&pair[1], 32)?;
                    }
                    Ok(())
                })?;
            }

            NodeData::None => unreachable!(),
        }

        let mut flags = 0u32;
        let sub_refs = [
            (&node.sub1, 0x1),
            (&node.sub2, 0x2),
            (&node.sub4, 0x4),
            (&node.sub8, 0x8),
            (&node.sub10, 0x10),
            (&node.sub20, 0x20),
            (&node.sub40, 0x40),
            (&node.sub80, 0x80),
            (&node.sub100, 0x100),
        ];

        for (sub, flag) in sub_refs.iter() {
            if sub.is_some() {
                flags |= flag;
            }
        }

        self.write_u32(flags)?;

        for (sub, _) in sub_refs.iter() {
            if let Some(nodes) = sub {
                self.write_u32(nodes.len() as u32)?;
                for n in nodes {
                    self.write_node(n, mapbin, version)?;
                }
            }
        }
        Ok(())
    }

    fn write_level(mut self, mapbin: &Mapbin) -> Result<Vec<u8>> {
        // filesize
        self.write_u32(PLACEHOLDER_VALUE)?;
        self.write_u32(PLACEHOLDER_VALUE)?;

        // version
        self.write_f32(mapbin.version)?;

        // strings

        let mut write_string32_array = |array: &Vec<String>| -> Result<()> {
            let count = array.len();
            self.write_u32(count as u32)?;

            for string in array.iter() {
                self.write_string(string, 32)?;
            }

            Ok(())
        };

        let string_tables = [
            &mapbin.object_types,
            &mapbin.item_types,
            &mapbin.collision_types,
            &mapbin.rect_types,
            &mapbin.enemy_types,
            &mapbin.unk_types_1,
        ];

        for table in string_tables {
            write_string32_array(table)?;
        }

        // nodes
        self.write_node(&mapbin.root, mapbin, mapbin.version)?;

        // pad to 0x20 bytes
        let len = self.buffer.len();
        for _ in 0..(len.next_multiple_of(0x20) - len) {
            self.buffer.push(0);
        }

        // write size
        let total_size = self.buffer.len() as u64;
        let mut real_size_slice = &mut self.buffer[0..8];
        real_size_slice.write_u64::<BigEndian>(total_size - 0xC)?; // exclude header

        Ok(self.buffer)
    }
}

pub fn write_level(mapbin: &Mapbin) -> Result<Vec<u8>> {
    Writer::new().write_level(mapbin)
}

// impls
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
