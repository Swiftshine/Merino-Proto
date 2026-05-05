mod merino;
use anyhow::{Result, bail};
use std::{env, fs};

use crate::merino::game::mapbin::{MapNodeType, NodeData};

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        bail!("spcify filename (something.mapbin)");
    }

    let bytes = fs::read(&args[1])?;

    let mapbin = merino::reader::read_level(&bytes)?;

    let nodes: Vec<&NodeData> = mapbin.root.children().map(|n| &n.node_data).collect();

    for node in nodes {
        if let NodeData::MapPolySet {
            collision_type_index,
            unk3,
            ..
        } = node
        {
            let col_type = &mapbin.collision_types[*collision_type_index as usize];

            println!("{col_type}, {unk3}");
        }
    }
    // let gimmicks: Vec<&NodeData> = mapbin
    //     .root
    //     .children()
    //     .filter(|n| matches!(n.node_type, MapNodeType::MapObjSet))
    //     .map(|n| &n.node_data)
    //     .collect();

    // for (index, gimmick) in gimmicks.iter().enumerate() {
    //     if let NodeData::MapObjSet {
    //         name_index, unk8, ..
    //     } = gimmick
    //     {
    //         let name = &mapbin.gimmick_types[*name_index as usize];
    //         println!("{name} ({name_index}) (index {index}): {:#?}", unk8);
    //     }
    // }
    Ok(())
}
