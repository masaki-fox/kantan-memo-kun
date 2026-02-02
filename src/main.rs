use eframe::egui;
use egui::FontFamily;
use std::sync::Arc;


fn main() -> eframe::Result<()> {
    // custom window settings
    let options = eframe::NativeOptions {
        viewport: egui::viewport::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]),
            ..Default::default()
    };
    
    // execute eframe application
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // font setting
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

// フォント設定用の関数
fn setup_custom_fonts(ctx: &egui::Context) {
    // フォント設定を取得
    let mut fonts = egui::FontDefinitions::default();
    
    // 日本語フォント（可変ウェイト）を追加
    fonts.font_data.insert(
        "noto_sans_jp".to_owned(),
        egui::FontData::from_static(
            include_bytes!("assets/NotoSansJP-VariableFont_wght.ttf")
        ).into(),
    );
    
    // フォントファミリーに追加
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "noto_sans_jp".to_owned()); // 一番優先度高く追加
    
    // モノスペースフォントにも日本語フォントを追加
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("noto_sans_jp".to_owned());
    
    // フォント設定を適用
    ctx.set_fonts(fonts);
}

// アプリケーションの状態を保持する構造体
struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "山田太郎".to_owned(),
            age: 42,
        }
    }
}

// アプリケーションの描画とロジックを実装
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("名前: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("年齢"));
            if ui.button("年齢+1").clicked() {
                self.age += 1;
            }
            ui.label(format!("こんにちは、{}さん({}歳)", self.name, self.age));
        });
    }
}
