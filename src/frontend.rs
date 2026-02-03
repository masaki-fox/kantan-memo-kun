mod home;
mod header;
mod leftsidepanel;

pub struct UiRoot {
    memos: Vec<String>,
    input: String,
}

impl Default for UiRoot {
    fn default() -> Self {
        Self {
            memos: Vec::new(),
            input: String::new(),
        }
    }
}

impl eframe::App for UiRoot {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // draw panels
        header::draw(ctx, self);
        home::draw(ctx, self);
        leftsidepanel::draw(ctx, self);
    }
}