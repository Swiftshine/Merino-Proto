use crate::merino::game::mapbin::{LimitedString, Params, Vec2f, Vec3f};

/// A trait to simplify property parsing.
pub trait Editable {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>);
}

pub enum EditInfo<'a> {
    Label(&'a str),
}

impl<'a> EditInfo<'a> {
    pub fn label(label: &'a str) -> Option<Self> {
        Some(Self::Label(label))
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
    fn edit_properties(&mut self, ui: &mut egui::Ui, _info: Option<EditInfo>) {
        // todo: implement some way to enable custom param names
        // this could easily be implemented into the EditInfo enum
        // to make it contain a struct that *actually* contains the param info
        // e.g.
        // EditInfo::Parameter { fields: ParamFields }
        // or something like that
        // for now, though, just display the values as they are

        // this is where custom-defined parameters would go
        // ui.collapsing("Parameters", ...);

        ui.collapsing("Raw Parameters", |ui| {
            ui.collapsing("Int Params", |ui| {
                ui.horizontal(|ui| {
                    for val in self.int_values.iter_mut() {
                        val.edit_properties(ui, None);
                    }
                });
            });

            ui.collapsing("Float Params", |ui| {
                ui.horizontal(|ui| {
                    for val in self.float_values.iter_mut() {
                        val.edit_properties(ui, None);
                    }
                });
            });

            ui.collapsing("String Params", |ui| {
                for val in self.string_values.iter_mut() {
                    val.edit_properties(ui, None);
                }
            });
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
