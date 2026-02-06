use eframe::egui;
use egui::{Frame, Margin, Color32};


pub fn draw(ctx: &egui::Context, app: &mut super::MemoApp) {
    // setting to aniamate sidebar width
    let anim = ctx.animate_bool_with_time_and_easing(
        egui::Id::new("sidebar_width_anim"),
        app.state.show_sidebar,
        0.35,
        egui::emath::easing::quadratic_out,
    );
    if anim < 0.1 {return;}
    // Max width of the side panel,and current width based on animation value
    let max_width = 300.0;
    let width = anim * max_width;
    
    egui::SidePanel::left("side_panel")
        .resizable(false)
        .frame(
            Frame::default()
                .inner_margin(Margin::same(10))
                .fill(Color32::from_gray(50)) 
        )
        .show(ctx, |ui| {
            ui.set_width(width);
            // Processing to prevent the contents from being displayed immediately
            if anim < 0.6 {return;} 

            ui.set_opacity(anim);
            
            // draw side panel contents
            ui.label("This is inner margin");
            
            // until here
        }
    );
}