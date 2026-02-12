use eframe::egui;
use crate::background::db_operator::{*};

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
                if let Some(conn) = &mut state.conn{
                    match get_all_memos(conn) {
                        Ok(memos) => {
                            for (id, name) in memos {
                                ui.label(format!("{}: {}", id, name));
                            }
                        }
                        Err(err) => {
                            ui.label(format!("DB read error: {:?}", err));
                        }
                    }
                }

            });
            // view memos



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

