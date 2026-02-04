mod home;
mod header;
mod sidepanel;
mod uistate;

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
        // draw panels
        header::draw(ctx, self);
        sidepanel::draw(ctx, self);
       
        // call last to cover full remaining area
        home::draw(ctx, self);
    }
}