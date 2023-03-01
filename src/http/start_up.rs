use std::{net::SocketAddr, sync::Arc};

use is_alive_middleware::IsAliveMiddleware;
use my_http_server::MyHttpServer;
use rust_extensions::AppStates;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub fn setup_server(app_states: Arc<AppStates>) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8000)));

    http_server.add_middleware(Arc::new(my_http_server::StaticFilesMiddleware::new(
        None,
        vec!["index.html".to_string()].into(),
    )));

    http_server.add_middleware(Arc::new(IsAliveMiddleware::new(
        APP_NAME.to_string(),
        APP_VERSION.to_string(),
    )));

    http_server.start(app_states, my_logger::LOGGER.clone());
}
