mod frontend;
mod background;
mod app_config;

use crate::app_config::{AppConfig, AppConfigError};
use crate::frontend::{home, header, sidepanel, uistate};
use crate::background::{db_operator};

fn main() -> eframe::Result<()> {
    // setting config
    match AppConfig::initilize_db_path(){
        Ok(cfg) => {
            println!("db path is got", );
            match db_operator::start_db_connection(&cfg.db_path){
                Ok(_) => println!("Conected DB"),
                Err(e) => 
                    {println!("failed conection ");
                    eprintln!("Failed to connect to DB: {:?}", e)},
            };
        }
        Err(AppConfigError::EnvVar(err)) => {
            eprintln!("Env var error: {}", err);
            panic!()
        }
        Err(AppConfigError::Dotenv(err)) => {
            eprintln!("Dotenv file error: {}", err);
            panic!()
        }
    };

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