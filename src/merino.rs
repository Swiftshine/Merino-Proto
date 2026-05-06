mod common;
mod game;
mod level_editor;
mod reader;
mod views;

use anyhow::Result;
use eframe::{NativeOptions, egui};

use crate::merino::{level_editor::LevelEditor, views::MerinoView};

pub struct MerinoApp {
    view: MerinoView,
    level_editor: LevelEditor,
}

impl MerinoApp {
    fn new() -> Self {
        Self {
            view: MerinoView::Home,
            level_editor: LevelEditor::new(),
        }
    }

    pub fn run() -> Result<(), eframe::Error> {
        let options = NativeOptions::default();

        eframe::run_native(
            "Merino (Prototype)",
            options,
            Box::new(|_cc| Ok(Box::<MerinoApp>::from(MerinoApp::new()))),
        )
    }
}

impl eframe::App for MerinoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("m_top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.view, MerinoView::Home, "Merino");
                    ui.selectable_value(&mut self.view, MerinoView::LevelEditor, "Level Editor");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.view {
            MerinoView::Home => {
                ui.centered_and_justified(|ui| {
                    ui.label("Welcome to Merino.");
                });
            }

            MerinoView::LevelEditor => self.level_editor.show_ui(ui),
        });
    }
}
