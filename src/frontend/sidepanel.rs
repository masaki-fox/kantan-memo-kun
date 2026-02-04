use eframe::egui;


pub fn draw(ctx: &egui::Context, app: &mut super::MemoApp) {
    // setting to aniamate sidebar width
    let anim = ctx.animate_bool_with_time_and_easing(
        egui::Id::new("sidebar_width_anim"),
        app.state.show_sidebar,
        0.35,
        egui::emath::easing::quadratic_out,
    );

    // Max width of the side panel,and current width based on animation value
    let max_width = 300.0;
    let width = anim * max_width;
    
    match app.state.show_sidebar {
        // do animate, but skip drawing
        false => return,
        true => {
            egui::SidePanel::left("side_panel")
                .default_width(width)
                .min_width(0.0)
                .max_width(max_width)
                .show(ctx, |ui| {
                    // 中身のフェード（opacity を anim に乗せる）
                    ui.set_min_width(width);
                    ui.set_max_width(width);

                    ui.group(|ui| {
                        ui.set_opacity(anim);  // ここでフェード
                        ui.vertical(|ui| {
                            ui.label("メニュー");
                            if ui.button("All Notes").clicked() {}
                            if ui.button("Tags").clicked() {}
                        });
                    });
                }
            );
        }
    };
}