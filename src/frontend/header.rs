use eframe::egui;

pub fn draw(ctx: &egui::Context, memo_app: &mut super::MemoApp) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        if ui.button("≡ Menu").clicked() {
            memo_app.state.show_sidebar = !memo_app.state.show_sidebar;
        }
        ui.heading("Memo App");
    });
}