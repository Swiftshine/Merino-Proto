use crate::merino::game::mapbin::Params;
use crate::merino::level_editor::le_params::ParameterDataType;
use crate::merino::{game::mapbin::MapNodeType, level_editor::LevelEditor};
use anyhow::{Result, anyhow};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct VariantCondition {
    pub data_type: ParameterDataType,
    pub slot: usize,
    pub expected_value: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct CopySource {
    pub data_type: ParameterDataType,
    pub slot: usize,
}

#[derive(Debug, Clone)]
pub enum VariantAction {
    SwapImage { display_image: String },

    RotateFromParam { source: CopySource },
}

#[derive(Debug, Clone)]
pub struct ImageVariant {
    pub when: Option<VariantCondition>,
    pub action: VariantAction,
}

#[derive(Debug, Clone, Default)]
pub struct ImageDefinition {
    pub display_image: Option<String>,
    pub variants: Vec<ImageVariant>,
}

pub struct ResolvedImage {
    pub image_path: String,
    pub rotation_degrees: f32,
}

impl ImageDefinition {
    pub fn resolve<const N: usize>(&self, params: &Params<N>) -> Option<ResolvedImage> {
        let mut resolved = ResolvedImage {
            image_path: self.display_image.clone()?,
            rotation_degrees: 0.0,
        };

        for variant in &self.variants {
            let matched = match &variant.when {
                Some(condition) => evaluate_condition(condition, params),

                None => true,
            };

            if !matched {
                continue;
            }

            apply_variant_action(&variant.action, params, &mut resolved);
        }

        Some(resolved)
    }
}

impl ParameterDataType {
    pub fn matches_json_value<const N: usize>(
        &self,
        expected: &serde_json::Value,
        params: &Params<N>,
        slot: usize,
    ) -> bool {
        match self {
            Self::Int | Self::DropdownInt => {
                expected.as_i64() == Some(params.int_values[slot] as i64)
            }

            Self::Float => expected.as_f64() == Some(params.float_values[slot] as f64),

            Self::String => expected.as_str() == Some(params.string_values[slot].as_str()),

            Self::Bool => expected.as_bool() == Some(params.int_values[slot] != 0),

            Self::None => false,
        }
    }

    pub fn extract_as_f32<const N: usize>(&self, params: &Params<N>, slot: usize) -> Option<f32> {
        match self {
            Self::Int | Self::DropdownInt => Some(params.int_values[slot] as f32),

            Self::Float => Some(params.float_values[slot]),

            _ => None,
        }
    }
}

fn apply_variant_action<const N: usize>(
    action: &VariantAction,
    params: &Params<N>,
    resolved: &mut ResolvedImage,
) {
    match action {
        VariantAction::SwapImage { display_image } => {
            resolved.image_path = display_image.clone();
        }

        VariantAction::RotateFromParam { source } => {
            if let Some(rotation) = source.data_type.extract_as_f32(params, source.slot) {
                resolved.rotation_degrees = rotation;
            }
        }
    }
}

fn evaluate_condition<const N: usize>(condition: &VariantCondition, params: &Params<N>) -> bool {
    condition
        .data_type
        .matches_json_value(&condition.expected_value, params, condition.slot)
}

impl LevelEditor {
    pub fn parse_image_data(&mut self, json: String) -> Result<()> {
        let json: serde_json::Value = serde_json::from_str(&json)?;

        let set_names = [
            "MapObjSet",
            "MapItemSet",
            "MapEnemySet",
            "MapLocator",
            "MapTerrain",
        ];

        self.canvas_context.image_bank.image_objects.clear();

        for set_name in set_names {
            let set_object = match json.get(set_name).and_then(|v| v.as_object()) {
                Some(v) => v,
                None => continue,
            };

            let set_type = MapNodeType::from_str(set_name)?;

            for (obj_name, obj_data) in set_object {
                let mut image_def = ImageDefinition::default();

                image_def.display_image = obj_data
                    .get("display_image")
                    .and_then(|v| v.as_str())
                    .map(String::from);

                if let Some(variants) = obj_data.get("variants").and_then(|v| v.as_array()) {
                    for variant in variants {
                        let when = variant
                            .get("when")
                            .map(|when_obj| {
                                Ok::<VariantCondition, anyhow::Error>(VariantCondition {
                                    data_type: ParameterDataType::from_string(
                                        when_obj["data_type"]
                                            .as_str()
                                            .ok_or_else(|| anyhow!("missing when.data_type"))?,
                                    )?,

                                    slot: when_obj["slot"]
                                        .as_u64()
                                        .ok_or_else(|| anyhow!("missing when.slot"))?
                                        as usize,

                                    expected_value: when_obj["expected_value"].clone(),
                                })
                            })
                            .transpose()?;

                        let action = if let Some(then_obj) = variant.get("then") {
                            VariantAction::SwapImage {
                                display_image: then_obj["display_image"]
                                    .as_str()
                                    .ok_or_else(|| anyhow!("missing then.display_image"))?
                                    .to_string(),
                            }
                        } else if let Some(copy_obj) = variant.get("copy") {
                            let source = CopySource {
                                data_type: ParameterDataType::from_string(
                                    copy_obj["data_type"]
                                        .as_str()
                                        .ok_or_else(|| anyhow!("missing copy.data_type"))?,
                                )?,

                                slot: copy_obj["slot"]
                                    .as_u64()
                                    .ok_or_else(|| anyhow!("missing copy.slot"))?
                                    as usize,
                            };

                            let to = variant["to"]
                                .as_str()
                                .ok_or_else(|| anyhow!("missing to"))?;

                            match to {
                                "rotation" => VariantAction::RotateFromParam { source },

                                _ => {
                                    return Err(anyhow!("unsupported manipulation type"));
                                }
                            }
                        } else {
                            return Err(anyhow!(
                                "variant must contain either \
                                     `then` or `copy`"
                            ));
                        };

                        image_def.variants.push(ImageVariant { when, action });
                    }
                }

                self.canvas_context
                    .image_bank
                    .image_objects
                    .insert((set_type, obj_name.clone()), image_def);
            }
        }

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
