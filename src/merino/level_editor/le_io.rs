use std::fs;

use crate::merino::{level_editor::LevelEditor, reader::read_level, writer::write_level};
use anyhow::Result;
use rfd::FileDialog;

impl LevelEditor {
    pub fn open_file(&mut self) -> Result<()> {
        // ask user to open file
        // for now we're opening the mapbin directly
        if let Some(path) = FileDialog::new()
            .add_filter("Level File", &["mapbin"])
            .pick_file()
        {
            self.file_path = Some(path);
            let path = self.file_path.as_ref().unwrap();
            let data = fs::read(path)?;

            self.mapdata = read_level(&data)?;

            self.file_open = true;
        }

        Ok(())
    }

    pub fn save_file(&mut self) -> Result<()> {
        if let Some(path) = FileDialog::new()
            .add_filter("Level File", &["mapbin"])
            .save_file()
        {
            // collect any brand new strings
            self.collect_new_strings();
            // then write the data
            let data = write_level(&self.mapdata)?;
            fs::write(path, data)?;
        }
        Ok(())
    }

    fn collect_new_strings(&mut self) {
        // go through every item, check if the string already exists,
        // and add it to the appropriate vec if not
        self.mapdata.collect_new_strings();
    }
}
