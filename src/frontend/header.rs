use eframe::egui;

pub fn draw(ctx: &egui::Context, state: &mut super::UiRoot) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.heading("Memo App");
    });
}