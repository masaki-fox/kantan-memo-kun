mod frontend;
mod background;

use crate::frontend::{home, header, sidepanel, uistate};


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
        Box::new(|_cc| Ok(Box::new(MemoApp::default()))),
    )
}

pub struct MemoApp {
    memos: Vec<String>,
    input: String,
    state: uistate::UIState,
}

impl Default for MemoApp {
    fn default() -> Self {
        Self {
            memos: Vec::new(),
            input: String::new(),
            state: uistate::UIState::default(),
        }
    }
}

impl eframe::App for MemoApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5); // contents scall setting

        // draw panels
        header::draw(ctx, self);
        sidepanel::draw(ctx, self);
       
        // call last to cover full remaining area
        home::draw(ctx, self);
    }
}