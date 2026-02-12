mod frontend;
mod background;
mod app_config;

use rusqlite::Connection;

use crate::app_config::{AppConfig, AppConfigError};
use crate::frontend::{home, header, sidepanel, uistate};
use crate::background::{db_operator};

fn main() -> eframe::Result<()> {
    // setting config
    let db_path;
    match AppConfig::initilize_db_path(){
        Ok(cfg) => {
            println!("db path is got", );
            db_path = cfg.db_path;
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

    // db connection
    let conn = match db_operator::start_db_connection(&db_path){
        Ok(c) => {
            println!("Conected DB");
            c},
        Err(e) => 
            {println!("failed conection ");
            eprintln!("Failed to connect to DB: {:?}", e);
            panic!()},
    };

    // setup MemoApp
    let options = eframe::NativeOptions{
        viewport: eframe::egui::ViewportBuilder {
            maximized: Some(true),
            ..Default::default()
        },
        ..Default::default()
    };
    let mut app = MemoApp::default();
    app.conn = Some(conn);

    // execute eframe application
    eframe::run_native(
        "kantan-memo-kun",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}

pub struct MemoApp {
    memos: Vec<String>,
    input: String,
    state: uistate::UIState,
    conn: Option<Connection>,
}
impl Default for MemoApp {
    fn default() -> Self {
        Self {
            memos: Vec::new(),
            input: String::new(),
            state: uistate::UIState::default(),
            conn: None,
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