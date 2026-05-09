use crate::merino::{game::mapbin::MapNodeType, level_editor::LevelEditor};
use anyhow::Result;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct ImageDefinition {
    pub display_image: Option<String>,
}

impl LevelEditor {
    pub fn parse_image_data(&mut self, json: String) -> Result<()> {
        let json: serde_json::Value = serde_json::from_str(&json).expect("failed to parse json");

        let mut image_objects = HashMap::new();

        let set_names = ["MapObjSet", "MapItemSet", "MapEnemySet", "Maplocator"];

        for set_name in set_names {
            let set_object = match json.get(set_name).and_then(|v| v.as_object()) {
                Some(obj) => obj,
                None => continue,
            };

            let set_type = MapNodeType::from_str(set_name)?;

            for (obj_name, props) in set_object {
                let display_image = props
                    .get("display_image")
                    .and_then(|v| v.as_str())
                    .map(String::from);

                let key = (set_type, obj_name.clone());

                image_objects.insert(key, ImageDefinition { display_image });
            }
        }

        self.canvas_context.image_bank.image_objects = image_objects;

        Ok(())
    }
}

// use crate::merino::game::mapbin::MapDataNode;

// pub struct ImageHandle;

// pub enum ConditionValue {
//     Int(i32),
//     Float(f32),
//     Bool(bool),
//     String(String)
// }

// impl ObjectImageDefinition {
//     pub fn resolve_image<'a>(&'a self, node: &MapDataNode) -> &'a ImageHandle {
//         for variant in &self.variants {
//             if variant.matches(node) {
//                 return &variant.images;
//             }
//         }

//         &self.default_image
//     }
// }

// pub struct ImageVariant {
//     pub conditions: Vec<ImageCondition>,
//     pub image: ImageHandle,
// }

// impl ImageVariant {
//     pub fn matches(&self, node: &MapDataNode) -> bool {
//         self.conditions.iter().all(|cond| {
//             cond.matches(node)
//         })
//     }
// }

// pub struct ImageCondition {
//     pub parameter: String,
//     pub expected: ConditionValue,
// }

// pub struct ImageCondition {
//     pub parameter: String,
//     pub expected: ConditionValue
// }
