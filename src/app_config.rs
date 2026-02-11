use std::env::VarError;
use std::fmt;
use std::error::Error;
use std::env;


#[derive(Debug)]
pub struct AppConfig{
    pub db_path: String,
}

impl AppConfig {
    pub fn initilize_db_path() -> Result<Self, AppConfigError> {
        // get key from OS envPath
        let app_env = env::var("MEMO_DB_PATH")
            .unwrap_or_else(|_| "dev".to_string());

        // read file
        let filename = format!(".env.{}", app_env);
        dotenvy::from_filename(filename)?;

        // fetch env var
        let db_path = env::var("MEMO_DB_PATH")?;

        Ok(Self { db_path })
    }
}

#[derive(Debug)]
pub enum AppConfigError {
    EnvVar(VarError),   // env cant get 
    Dotenv(dotenvy::Error),    //dotenv cant read 

}
impl fmt::Display for AppConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppConfigError::EnvVar(e) => write!(f, "Environment variable error: {}", e),
            AppConfigError::Dotenv(e) => write!(f, "Failed to load dotenv file: {}", e),
        }
    }
}
impl Error for AppConfigError {}
impl From<VarError> for AppConfigError {
    fn from(err: VarError) -> AppConfigError {
        AppConfigError::EnvVar(err)
    }
}
impl From<dotenvy::Error> for AppConfigError {
    fn from(err: dotenvy::Error) -> AppConfigError {
        AppConfigError::Dotenv(err)
    }
}
