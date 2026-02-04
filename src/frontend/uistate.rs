pub struct UIState {
    pub show_sidebar: bool,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            show_sidebar: false,
        }
    }
}