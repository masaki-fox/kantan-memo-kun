use eframe::{egui::{self, InnerResponse, Response}, egui_glow::painter};
use crate::background::db_operator::{save_memo_to_db};


pub fn draw(ctx: &egui::Context, state: &mut super::MemoApp) {
    egui::CentralPanel::default()
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.text_edit_multiline(&mut state.input);
                // save
                if ui.button("save").clicked() {
                    // 編集処理 (簡易版)
                    //state.input = state.memos[i].clone();
                    match save_memo_to_db(&state.input) {
                        Ok(_) => println!("Success save"),
                        Err(e) => eprintln!("failed save: {}", e),
                    }
                    //state.memos.remove(i);
                    
                }
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
            // view memos
            for i in 0..state.memos.len() {
                ui.horizontal(|ui| {
                    ui.label(format!("• {}", state.memos[i]));



                    // Delete ボタン
                    if ui.button("Delete").clicked() {
                        state.memos.remove(i);
                    }
                });
            }
            });


        });
}
/*
            // Enter 押下 or ボタンクリック検知
            if text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                // ここで登録イベントとして扱う
                println!("Enter pressed, text: {}", self.input_text);
                // → DB に登録する関数を呼ぶ
                save_to_db(&self.input_text);
                self.input_text.clear(); // 登録後クリア
            }
            */

