use std::fs;

use crate::merino::{level_editor::LevelEditor, reader::read_level};
use anyhow::Result;
use rfd::FileDialog;

impl LevelEditor {
    pub fn open_file(&mut self, ctx: &egui::Context) -> Result<()> {
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

            println!("opened file!");
        }

        Ok(())
    }
}
