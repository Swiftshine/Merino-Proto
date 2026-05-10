use std::fs;

use crate::merino::{
    common::util::{get_merino_folder_path, make_merino_folder, merino_folder_exists},
    level_editor::LevelEditor,
    reader::read_level,
    writer::write_level,
};
use anyhow::{Context, Result};
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
}
