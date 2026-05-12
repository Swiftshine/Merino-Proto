use std::fs;

use crate::merino::{
    common::util::{get_merino_folder_path, make_merino_folder, merino_folder_exists},
    level_editor::LevelEditor,
    reader::read_level,
    writer::write_level,
};
use ::gfarch::gfarch::{CompressionType, GFCPOffset, Version};
use anyhow::{Context, Result};
use gfarch::gfarch;
use rfd::FileDialog;

const OBJECTDATA_FILE: &str = "objectdata.json";
const IMAGEDATA_FILE: &str = "imagedata.json";

impl LevelEditor {
    // returns if the file was actually opened
    pub fn open_file(&mut self) -> Result<bool> {
        // ask user to open file
        // for now we're opening the mapbin directly
        if let Some(path) = FileDialog::new()
            .add_filter("Level File", &["mapbin"])
            .pick_file()
        {
            self.io_context.file_path = Some(path);
            let path = self.io_context.file_path.as_ref().unwrap();
            let data = fs::read(path)?;

            match read_level(&data) {
                Ok(data) => {
                    self.file_context.mapdata = data;
                    self.io_context.file_open = true;
                }

                Err(e) => {
                    dbg!(e);
                }
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn save_file(&mut self) -> Result<()> {
        if let Some(path) = FileDialog::new()
            .add_filter("Level File", &["mapbin"])
            .save_file()
        {
            // collect any brand new strings
            self.collect_new_strings();
            // then write the data
            let data = write_level(&self.file_context.mapdata)?;
            fs::write(path, data)?;
        }
        Ok(())
    }

    fn collect_new_strings(&mut self) {
        // go through every item, check if the string already exists,
        // and add it to the appropriate vec if not
        self.file_context.mapdata.collect_new_strings();
    }

    /// Loads `objectdata.json`
    pub fn load_param_data(&mut self) -> Result<()> {
        if !merino_folder_exists()? {
            make_merino_folder()?;
        }

        let path = get_merino_folder_path()?.join(OBJECTDATA_FILE);
        let file = fs::read_to_string(path).context("Could not find objectdata.json")?;

        self.parse_params(file)
    }

    pub fn load_image_data(&mut self) -> Result<()> {
        if !merino_folder_exists()? {
            make_merino_folder()?;
        }

        let path = get_merino_folder_path()?.join(IMAGEDATA_FILE);

        let file = fs::read_to_string(path).context("Could not find imagedata.json")?;

        self.parse_image_data(file)?;

        Ok(())
    }

    pub fn open_archive(&mut self) -> Result<()> {
        if let Some(path) = FileDialog::new()
            .add_filter("Good-Feel Archive", &["gfa"])
            .pick_file()
        {
            self.io_context.file_path = Some(path);
            let path = self.io_context.file_path.as_ref().unwrap();

            let data = fs::read(path)?;

            self.file_context.archive_contents = gfarch::extract(&data)?.into_iter().collect();

            // todo! since both BSON and level files are often stored in the same archive,
            // just have a level editor and a BSON viewer for the same file
            // that way it can all be done in-editor
            // there'd be a docked tab for it and clicking on it would open that file's editor

            self.io_context.archive_open = true;
            // file is not open just yet because we don't know if there even is a level file in here at all
        }
        Ok(())
    }

    pub fn save_archive(&mut self) -> Result<()> {
        let Some(path) = FileDialog::new()
            .add_filter("Good-Feel Archive", &["gfa"])
            .save_file()
        else {
            return Ok(());
        };

        let Some(name) = self.file_context.current_archive_file.as_ref() else {
            anyhow::bail!("No current archive file selected");
        };

        let level_data = write_level(&self.file_context.mapdata)?;

        let file = self
            .file_context
            .archive_contents
            .get_mut(name)
            .ok_or_else(|| anyhow::anyhow!("Archive entry not found: {name}"))?;

        *file = level_data;

        let files: Vec<(String, Vec<u8>)> = self
            .file_context
            .archive_contents
            .iter()
            .map(|(name, data)| (name.clone(), data.clone()))
            .collect();

        let data = gfarch::pack_from_files(
            &files,
            Version::V3_1,
            CompressionType::BPE,
            GFCPOffset::Default,
        );

        fs::write(path, data)?;

        Ok(())
    }
}
