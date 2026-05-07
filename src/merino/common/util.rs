use std::{env, fs, path::PathBuf};

use anyhow::{Context, Result};

const MERINO_FOLDER: &'static str = "merino_res";

pub fn get_merino_folder_path() -> Result<PathBuf> {
    let base_path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        // dev
        PathBuf::from(manifest_dir)
    } else {
        // release
        env::current_exe()?
            .parent()
            .context("Could not find executable parent directory")?
            .to_path_buf()
    };

    Ok(base_path.join(MERINO_FOLDER))
}

pub fn merino_folder_exists() -> Result<bool> {
    Ok(get_merino_folder_path()?.exists())
}

pub fn make_merino_folder() -> Result<()> {
    fs::create_dir(get_merino_folder_path()?)?;
    Ok(())
}
