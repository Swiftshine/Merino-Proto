use crate::merino::{
    game::mapbin::{LimitedString, MapNodeType, Params, Vec2f, Vec3f},
    level_editor::le_params::{ParameterDataType, ParameterObject},
};

/// A trait to simplify property parsing.
pub trait Editable {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>);
}

pub enum EditInfo<'a> {
    Label(&'a str),
    Params(&'a ParameterObject),
}

impl<'a> EditInfo<'a> {
    pub fn label(label: &'a str) -> Option<Self> {
        Some(Self::Label(label))
    }

    pub fn search_param(
        list: &'a [ParameterObject],
        node_type: MapNodeType,
        name: &'a str,
    ) -> Option<Self> {
        list.iter()
            .find(|obj| obj.set_type == node_type && obj.name == name)
            .map(Self::Params)
    }
}

// actual trait implementations

impl<T> Editable for Option<T>
where
    T: Editable,
{
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) {
        if let Some(val) = self {
            val.edit_properties(ui, info);
        }
    }
}

macro_rules! impl_editable_numeric {
    ($($t:ty),*) => {
        $(
            impl Editable for $t {
                fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) {
                    let mut render = |ui: &mut egui::Ui| {
                        ui.add(egui::DragValue::new(self)
                            .speed(1.0)
                            .range(<$t>::MIN..=<$t>::MAX));
                    };

                    if let Some(EditInfo::Label(label)) = info {
                        // this is its own attribute
                        ui.collapsing(label, |ui| ui.horizontal(render));
                    } else {
                        // this is part of an existing attribute
                        render(ui);
                    }
                }
            }
        )*
    };
}

impl_editable_numeric!(u32, i32, f32);

macro_rules! impl_editable_vec {
    ($t:ty, [$($field:ident),*]) => {
        impl Editable for $t {
            fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) {
                if let Some(EditInfo::Label(label)) = info {
                    ui.collapsing(label, |ui| {
                        ui.horizontal(|ui| {
                            $(
                                ui.label(stringify!($field).to_uppercase());
                                ui.add(egui::DragValue::new(&mut self.$field)
                                    .speed(0.5)
                                    .range(f32::MIN..=f32::MAX));
                            )*
                        });
                    });
                }
            }
        }
    };
}

impl_editable_vec!(Vec2f, [x, y]);
impl_editable_vec!(Vec3f, [x, y, z]);

impl<const N: usize> Editable for LimitedString<N> {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) {
        if let Some(EditInfo::Label(label)) = info {
            ui.collapsing(label, |ui| {
                ui.add(egui::TextEdit::singleline(&mut self.0).char_limit(N));
            });
        } else {
            ui.add(egui::TextEdit::singleline(&mut self.0).char_limit(N));
        }
    }
}

impl<const N: usize> Editable for Params<N> {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) {
        if let Some(EditInfo::Params(param_object)) = info {
            ui.collapsing("Parameters", |ui| {
                for param in param_object.parameters.iter() {
                    ui.collapsing(&param.name, |ui| {
                        if let Some(desc) = &param.description
                            && !desc.is_empty()
                        {
                            ui.label(desc);
                        }

                        if let Some(note) = &param.note
                            && !note.is_empty()
                        {
                            ui.label(format!("Note: {note}"));
                        }

                        match &param.data_type {
                            ParameterDataType::Int => {
                                if let Some(val) = self.int_values.get_mut(param.slot) {
                                    val.edit_properties(ui, None);
                                }
                            }

                            ParameterDataType::Float => {
                                if let Some(val) = self.float_values.get_mut(param.slot) {
                                    val.edit_properties(ui, None);
                                }
                            }

                            ParameterDataType::String => {
                                if let Some(val) = self.string_values.get_mut(param.slot) {
                                    val.edit_properties(ui, None);
                                }
                            }

                            ParameterDataType::Bool => {
                                if let Some(val) = self.int_values.get_mut(param.slot) {
                                    let mut bool_value = *val != 0;

                                    if ui.checkbox(&mut bool_value, "Value").changed() {
                                        *val = if bool_value { 1 } else { 0 };
                                    }
                                }
                            }

                            ParameterDataType::DropdownInt => {
                                if let Some(options) = &param.dropdown_options {
                                    if let Some(val) = self.int_values.get_mut(param.slot) {
                                        egui::ComboBox::from_label("Value")
                                            .selected_text(&options[*val as usize].key)
                                            .show_ui(ui, |ui| {
                                                for option in options.iter() {
                                                    ui.selectable_value(
                                                        val,
                                                        option.value,
                                                        &option.key,
                                                    );
                                                }
                                            });
                                    }
                                }
                            }
                            _ => {}
                        }
                    });
                }
            });
        }

        ui.collapsing("Raw Parameters", |ui| {
            ui.label("Int Params");
            ui.horizontal(|ui| {
                for val in self.int_values.iter_mut() {
                    val.edit_properties(ui, None);
                }
            });

            ui.label("Float Params");
            ui.horizontal(|ui| {
                for val in self.float_values.iter_mut() {
                    val.edit_properties(ui, None);
                }
            });

            ui.label("String Params");
            for val in self.string_values.iter_mut() {
                val.edit_properties(ui, None);
            }
        });
    }
}

// todo! make this not suck
impl<T, const N: usize> Editable for [T; N]
where
    T: Editable,
{
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) {
        if let Some(EditInfo::Label(label)) = info {
            ui.collapsing(label, |ui| {
                for item in self.iter_mut() {
                    item.edit_properties(ui, None);
                }
            });
        } else {
            for item in self.iter_mut() {
                item.edit_properties(ui, None);
            }
        }
    }
}
