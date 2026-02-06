use eframe::egui;

pub fn draw(ctx: &egui::Context, state: &mut super::MemoApp) {
    egui::CentralPanel::default()
        .show(ctx, |ui| {
            ui.heading("Memo Editor");
            ui.horizontal(|ui| {
            // input field and Add button
            ui.text_edit_multiline(&mut state.input);
                if ui.button("Add").clicked() {
                    if !state.input.is_empty() {
                        state.memos.push(state.input.clone());
                        state.input.clear();
                    }
                }
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
            // view memos
            for i in 0..state.memos.len() {
                ui.horizontal(|ui| {
                    ui.label(format!("• {}", state.memos[i]));

                    // Edit ボタン
                    if ui.button("Edit").clicked() {
                        // 編集処理 (簡易版)
                        state.input = state.memos[i].clone();
                        state.memos.remove(i);
                    }

                    // Delete ボタン
                    if ui.button("Delete").clicked() {
                        state.memos.remove(i);
                    }
                });
            }
            });


        });
}