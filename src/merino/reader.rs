use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

use crate::merino::game::mapbin::{MapDataNode, Mapbin};

pub trait Readable {
    fn read(reader: &mut Reader) -> Result<Self>
    where
        Self: Sized;
}

// reader

pub struct Reader<'a> {
    cursor: Cursor<&'a [u8]>,
    pub version: f32,
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(bytes),
            version: 0.0,
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

    pub fn read_level(&mut self) -> Result<Mapbin> {
        // read header

        let _filesize = self.read_u64()? as usize; // excluding the header
        let version = self.read_f32()?;
        self.version = version;

        // read strings

        let gimmick_types = self.read_array(|reader| reader.read_string(0x20))?;
        let collectible_types = self.read_array(|reader| reader.read_string(0x20))?;
        let collision_types = self.read_array(|reader| reader.read_string(0x20))?;
        let rect_types = self.read_array(|reader| reader.read_string(0x20))?;
        let enemy_types = self.read_array(|reader| reader.read_string(0x20))?;
        let unk_types_1 = self.read_array(|reader| reader.read_string(0x20))?;
        let root = MapDataNode::read(self)?;

        Ok(Mapbin {
            gimmick_types,
            collectible_types,
            collision_types,
            rect_types,
            enemy_types,
            unk_types_1,
            root,
        })
    }
}

pub fn read_level(bytes: &[u8]) -> Result<Mapbin> {
    Reader::new(bytes).read_level()
}
