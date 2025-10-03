use std::{net::SocketAddr, sync::Arc};

use is_alive_middleware::IsAliveMiddleware;
use my_http_server::MyHttpServer;

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

pub fn setup_server(addr: impl Into<Addr>) {
    let mut http_server = match addr.into() {
        Addr::Tcp(socket_addr) => MyHttpServer::new(socket_addr),
        Addr::Unix(unix_socket_addr) => MyHttpServer::new_as_unix_socket(unix_socket_addr),
    };

    http_server.add_middleware(Arc::new(IsAliveMiddleware::new(
        crate::app::APP_CTX.app_name.to_string(),
        crate::app::APP_CTX.app_version.to_string(),
    )));

    http_server.add_middleware(Arc::new(super::InjectVersionMiddleware));

    http_server.add_middleware(Arc::new(
        my_http_server::StaticFilesMiddleware::new(None, vec!["index.html".to_string()].into())
            .set_not_found_file("index.html".to_string())
            .enable_files_caching(),
    ));

    http_server.start(
        crate::app::APP_CTX.app_states.clone(),
        my_logger::LOGGER.clone(),
    );
}
