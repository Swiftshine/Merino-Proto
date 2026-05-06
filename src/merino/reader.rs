use anyhow::{Result, anyhow};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

use crate::merino::game::mapbin::{MapDataNode, Mapbin, String32};

pub trait Readable {
    fn read(reader: &mut Reader) -> Result<Self>
    where
        Self: Sized;
}

// reader

#[derive(Default)]
pub struct Reader<'a> {
    cursor: Cursor<&'a [u8]>,
    pub version: f32,
    pub object_types: Vec<String32>,
    pub collectible_types: Vec<String32>,
    pub collision_types: Vec<String32>,
    pub rect_types: Vec<String32>,
    pub enemy_types: Vec<String32>,
    pub unk_types_1: Vec<String32>,
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(bytes),
            version: 0.0,
            ..Default::default()
        }
    }

    // pub fn position(&self) -> u64 {
    //     self.cursor.position()
    // }

    // pub fn align(&mut self, alignment: u64) {
    //     self.cursor
    //         .set_position(self.position().next_multiple_of(alignment));
    // }

    /* raw data */

    fn read_bytes(&mut self, num_bytes: usize) -> Result<Vec<u8>> {
        let mut bytes = vec![0u8; num_bytes];
        self.cursor.read_exact(&mut bytes)?;
        Ok(bytes)
    }

    // pub fn read_u8(&mut self) -> Result<u8> {
    //     Ok(self.cursor.read_u8()?)
    // }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(self.cursor.read_u32::<BigEndian>()?)
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(self.cursor.read_i32::<BigEndian>()?)
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        Ok(self.cursor.read_f32::<BigEndian>()?)
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        Ok(self.cursor.read_u64::<BigEndian>()?)
    }

    pub fn read_string(&mut self, string_length: usize) -> Result<String> {
        let bytes = self.read_bytes(string_length)?;

        // find pos of null byte
        let string_len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());

        Ok(String::from_utf8_lossy(&bytes[..string_len]).to_string())
    }

    pub fn read_object<T>(&mut self) -> Result<T>
    where
        T: Readable,
    {
        T::read(self)
    }

    pub fn read_at_version<T, F>(&mut self, min_version: f32, f: F) -> Result<Option<T>>
    where
        F: FnOnce(&mut Self) -> Result<T>,
    {
        if self.version >= min_version {
            f(self).map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn read_below_version<T, F>(&mut self, max_version: f32, f: F) -> Result<Option<T>>
    where
        F: FnOnce(&mut Self) -> Result<T>,
    {
        if self.version < max_version {
            f(self).map(Some)
        } else {
            Ok(None)
        }
    }
    // custom

    /// Reads a u32 count and performs F that many times, returning a Result<Vec<T>>
    pub fn read_array<T, F>(&mut self, mut f: F) -> Result<Vec<T>>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        let count = self.read_u32()?;

        let mut results = Vec::with_capacity(count as usize);

        for _ in 0..count {
            results.push(f(self)?);
        }

        Ok(results)
    }

    pub fn read_level(mut self) -> Result<Mapbin> {
        // read header

        let _filesize = self.read_u64()? as usize; // excluding the header
        let version = self.read_f32()?;
        self.version = version;

        // read strings

        self.object_types = self.read_array(|reader| reader.read_object::<String32>())?;
        self.collectible_types = self.read_array(|reader| reader.read_object::<String32>())?;
        self.collision_types = self.read_array(|reader| reader.read_object::<String32>())?;
        self.rect_types = self.read_array(|reader| reader.read_object::<String32>())?;
        self.enemy_types = self.read_array(|reader| reader.read_object::<String32>())?;
        self.unk_types_1 = self.read_array(|reader| reader.read_object::<String32>())?;
        let root = MapDataNode::read(&mut self)?;

        Ok(Mapbin {
            version,
            object_types: self.object_types,
            item_types: self.collectible_types,
            collision_types: self.collision_types,
            rect_types: self.rect_types,
            enemy_types: self.enemy_types,
            unk_types_1: self.unk_types_1,
            root,
        })
    }

    // accessors
    fn get_string_by_index(
        &self,
        list: &[String32],
        index: usize,
        label: &str,
    ) -> Result<String32> {
        list.get(index).cloned().ok_or_else(|| {
            anyhow!(
                "{} index {} out of bounds (len: {})",
                label,
                index,
                list.len()
            )
        })
    }

    pub fn read_object_type(&mut self) -> Result<String32> {
        let index = self.read_u32()? as usize;
        self.get_string_by_index(&self.object_types, index, "Object")
    }

    pub fn read_item_type(&mut self) -> Result<String32> {
        let index = self.read_u32()? as usize;
        self.get_string_by_index(&self.collectible_types, index, "Collectible")
    }

    pub fn read_collision_type(&mut self) -> Result<String32> {
        let index = self.read_u32()? as usize;
        self.get_string_by_index(&self.collision_types, index, "Collision")
    }

    // pub fn read_unk_type_1(&mut self) -> Result<String> {
    //     let index = self.read_u32()? as usize;
    //     self.get_string_by_index(&self.unk_types_1, index, "Unk1")
    // }
}

pub fn read_level(bytes: &[u8]) -> Result<Mapbin> {
    Reader::new(bytes).read_level()
}
