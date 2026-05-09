// use crate::merino::game::mapbin::MapDataNode;

// pub struct ImageHandle;

// pub enum ConditionValue {
//     Int(i32),
//     Float(f32),
//     Bool(bool),
//     String(String)
// }

// pub struct ObjectImageDefinition {
//     pub default_image: ImageHandle,
//     pub variants: Vec<ImageVariant>,
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
