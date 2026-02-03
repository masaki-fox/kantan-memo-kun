mod frontend;

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
        Box::new(|_cc| Ok(Box::new(frontend::UiRoot::default()))),
    )
}


