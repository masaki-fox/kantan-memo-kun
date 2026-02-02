use eframe::egui::{self, text};
use egui::FontFamily;
use std::sync::Arc;



fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions{
        viewport: eframe::egui::ViewportBuilder {
            maximized: Some(true),
            ..Default::default()
        },
        ..Default::default()
    };
    
    // execute eframe application
    eframe::run_native(
        "kantan-memo-kun",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    memos: Vec<String>,
    input: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            memos: Vec::new(),
            input: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // top panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("Memo App");
        });

        // main panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Memo Editor");
            ui.horizontal(|ui| {
            // input field and Add button
            ui.text_edit_multiline(&mut self.input);
                if ui.button("Add").clicked() {
                    if !self.input.is_empty() {
                        self.memos.push(self.input.clone());
                        self.input.clear();
                    }
                }
            });
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for memo in &self.memos {
                    ui.label(memo);
                }
            });
            ui.separator();

            // メモ一覧
            for i in 0..self.memos.len() {
                ui.horizontal(|ui| {
                    ui.label(format!("• {}", self.memos[i]));

                    // Edit ボタン
                    if ui.button("Edit").clicked() {
                        // 編集処理 (簡易版)
                        self.input = self.memos[i].clone();
                        self.memos.remove(i);
                    }

                    // Delete ボタン
                    if ui.button("Delete").clicked() {
                        self.memos.remove(i);
                    }
                });
            }
        });

        // left side panel
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.label("Welcome to Kantan Memo-kun!");
        });

    }
}

