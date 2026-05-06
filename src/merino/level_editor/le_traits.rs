use crate::merino::game::mapbin::{Params, Vec2f, Vec3f};

/// A trait to simplify property parsing.
pub trait Editable {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>);
}

pub enum EditInfo<'a> {
    Value { label: &'a str },
    String { label: &'a str, limit: usize },
    StringLimit(usize),
}

impl<'a> EditInfo<'a> {
    pub fn string_limit(limit: usize) -> Option<Self> {
        Some(Self::StringLimit(limit))
    }

    pub fn value(label: &'a str) -> Option<Self> {
        Some(Self::Value { label })
    }

    pub fn string(label: &'a str, limit: usize) -> Option<Self> {
        Some(Self::String { label, limit })
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

impl Editable for u32 {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) {
        if let Some(EditInfo::Value { label }) = info {
            ui.collapsing(label, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(self)
                            .speed(1)
                            .range(Self::MIN..=Self::MAX),
                    );
                });
            });
        } else {
            // just as-is
            ui.add(
                egui::DragValue::new(self)
                    .speed(1)
                    .range(Self::MIN..=Self::MAX),
            );
        }
    }
}

impl Editable for i32 {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) {
        if let Some(EditInfo::Value { label }) = info {
            ui.collapsing(label, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(self)
                            .speed(1)
                            .range(Self::MIN..=Self::MAX),
                    );
                });
            });
        } else {
            ui.add(
                egui::DragValue::new(self)
                    .speed(1)
                    .range(Self::MIN..=Self::MAX),
            );
        }
    }
}

impl Editable for f32 {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) {
        if let Some(EditInfo::Value { label }) = info {
            ui.collapsing(label, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(self)
                            .speed(1)
                            .range(Self::MIN..=Self::MAX),
                    );
                });
            });
        } else {
            ui.add(
                egui::DragValue::new(self)
                    .speed(1)
                    .range(Self::MIN..=Self::MAX),
            );
        }
    }
}

impl Editable for Vec2f {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) {
        if let Some(EditInfo::Value { label }) = info {
            ui.collapsing(label, |ui| {
                ui.horizontal(|ui| {
                    let items = [("X", &mut self.x), ("Y", &mut self.y)];
                    for (item_label, value) in items {
                        ui.label(item_label);
                        ui.add(
                            egui::DragValue::new(value)
                                .speed(0.5)
                                .range(f32::MIN..=f32::MAX),
                        );
                    }
                });
            });
        }
    }
}

impl Editable for Vec3f {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) {
        if let Some(EditInfo::Value { label }) = info {
            ui.collapsing(label, |ui| {
                ui.horizontal(|ui| {
                    let items = [("X", &mut self.x), ("Y", &mut self.y), ("Z", &mut self.z)];
                    for (label, value) in items {
                        ui.label(label);
                        ui.add(
                            egui::DragValue::new(value)
                                .speed(0.5)
                                .range(f32::MIN..=f32::MAX),
                        );
                    }
                });
            });
        }
    }
}

impl Editable for String {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) {
        if let Some(EditInfo::String { label, limit }) = info {
            ui.collapsing(label, |ui| {
                ui.add(egui::TextEdit::singleline(self).char_limit(limit));
            });
        } else if let Some(EditInfo::StringLimit(limit)) = info {
            ui.add(egui::TextEdit::singleline(self).char_limit(limit));
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
                    val.edit_properties(ui, EditInfo::string_limit(64));
                }
            });
        });
    }
}
