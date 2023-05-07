use serde::Deserialize;

use crate::logger::LogLevel;

#[derive(Debug, Default, Deserialize)]
pub struct TracingSettings {
    pub(crate) enabled: bool,
    pub(crate) service_name: String,
    pub(crate) otlp_endpoint: String,
}

#[derive(Debug, Default, Deserialize)]
pub enum DatabaseType {
    #[default]
    Postgres,
    InfuxDB,
}

#[derive(Debug, Default, Deserialize)]
pub struct Database {
    db_type: DatabaseType,
    // host: String,
    // port: u16,
    // username: String,
    // password: String,
    // database: String,
    // secrets_path: String,
}
impl Database {
    pub fn new() -> Self {
        let db_type = DatabaseType::Postgres;
        Database {
            db_type,
            // host: (),
            // port: (),
            // username: (),
            // password: (),
            // database: (),
            // sedrets_path: (),
        }
    }
}

// create a struct to hold the settings for the server in file ./server.rs
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Option<Database>,
    pub(crate) address: String,
    pub(crate) log_level: LogLevel,
    #[cfg(feature = "trace")]
    pub(crate) tracing: TracingSettings,
}

impl Settings {
    pub fn new(database: Option<Database>, address: String, log_level: LogLevel) -> Self {
        Settings {
            database,
            address,
            log_level,
            #[cfg(feature = "trace")]
            tracing: TracingSettings::default(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings::new(Some(Database::new()), "".to_string(), LogLevel::Info)
    }
}
