use std::{net::SocketAddr, sync::Arc};

use is_alive_middleware::IsAliveMiddleware;
use my_http_server::MyHttpServer;
use rust_extensions::AppStates;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub enum Addr {
    Tcp(SocketAddr),
    Unix(String),
}

impl Into<Addr> for SocketAddr {
    fn into(self) -> Addr {
        Addr::Tcp(self)
    }
}

impl Into<Addr> for String {
    fn into(self) -> Addr {
        Addr::Unix(self)
    }
}

pub fn setup_server(app_states: Arc<AppStates>, addr: impl Into<Addr>) {
    let mut http_server = match addr.into() {
        Addr::Tcp(socket_addr) => MyHttpServer::new(socket_addr),
        Addr::Unix(unix_socket_addr) => MyHttpServer::new_as_unix_socket(unix_socket_addr),
    };

    http_server.add_middleware(Arc::new(IsAliveMiddleware::new(
        get_app_name(),
        get_app_version(),
    )));

    http_server.add_middleware(Arc::new(
        my_http_server::StaticFilesMiddleware::new(None, vec!["index.html".to_string()].into())
            .set_not_found_file("index.html".to_string())
            .enable_files_caching(),
    ));

    http_server.start(app_states, my_logger::LOGGER.clone());
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
