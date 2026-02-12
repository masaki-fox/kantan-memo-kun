use eframe::egui;
use crate::background::db_operator::{*};

pub fn draw(ctx: &egui::Context, state: &mut super::MemoApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            // text line
            let response =
                ui.text_edit_multiline(&mut state.input);
            // save memo
            if response.lost_focus(){
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

