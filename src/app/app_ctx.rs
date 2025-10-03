use std::sync::Arc;

use rust_extensions::AppStates;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

lazy_static::lazy_static! {
    pub static ref APP_CTX: Arc<AppContext> = {
        Arc::new(AppContext::new())
    };
}

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub app_name: String,
    pub app_version: String,
    pub compile_time: String,

    pub file_to_inject: Option<String>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            app_states: Arc::new(AppStates::create_initialized()),
            app_name: get_app_name(),
            app_version: get_app_version(),
            compile_time: get_compile_time(),
            file_to_inject: get_file_to_version_injection(),
        }
    }
}

fn get_app_name() -> String {
    if let Ok(app_name) = std::env::var("BUILD_NAME") {
        app_name
    } else {
        APP_NAME.to_string()
    }
}

fn get_app_version() -> String {
    if let Ok(app_version) = std::env::var("BUILD_VERSION") {
        app_version
    } else {
        APP_VERSION.to_string()
    }
}

fn get_compile_time() -> String {
    if let Ok(app_version) = std::env::var("COMPILE_TIME") {
        app_version
    } else {
        "".to_string()
    }
}

fn get_file_to_version_injection() -> Option<String> {
    if let Ok(file) = std::env::var("FILE_TO_VERSION_INJECTION") {
        Some(file)
    } else {
        None
    }
}
