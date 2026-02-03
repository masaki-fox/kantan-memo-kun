use eframe::egui;

pub fn draw(ctx: &egui::Context, state: &mut super::UiRoot) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.label("Welcome to Kantan Memo-kun!");
        });
}