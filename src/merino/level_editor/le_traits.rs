use crate::merino::game::mapbin::Vec2f;

/// A trait to simplify property parsing.
pub trait Editable {
    fn edit_properties(&mut self, ui: &mut egui::Ui);
}

// actual trait implementations
impl Editable for Vec2f {
    fn edit_properties(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("X");
            ui.add(
                egui::DragValue::new(&mut self.x)
                    .speed(0.5)
                    .range(f32::MIN..=f32::MAX),
            );
            ui.label("Y");
            ui.add(
                egui::DragValue::new(&mut self.y)
                    .speed(0.5)
                    .range(f32::MIN..=f32::MAX),
            );
        });
    }
}
