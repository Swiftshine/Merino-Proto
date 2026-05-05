mod merino;
use anyhow::{Result, bail};
use std::{env, fs};

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        bail!("spcify filename (something.mapbin)");
    }

    let bytes = fs::read(&args[1])?;

    let _mapbin = merino::reader::read_level(&bytes)?;

    Ok(())
}
