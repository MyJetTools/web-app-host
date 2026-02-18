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

pub async fn setup_server(addr: impl Into<Addr>) {
    let mut http_server = match addr.into() {
        Addr::Tcp(socket_addr) => MyHttpServer::new(socket_addr),
        Addr::Unix(unix_socket_addr) => MyHttpServer::new_as_unix_socket(unix_socket_addr),
    };

    http_server.add_middleware(Arc::new(IsAliveMiddleware::new(
        crate::app::APP_CTX.app_name.to_string(),
        crate::app::APP_CTX.app_version.to_string(),
    )));

    http_server.add_middleware(Arc::new(super::InjectVersionMiddleware));

    let mut static_files_middleware = my_http_server::StaticFilesMiddleware::new()
        .set_not_found_file("index.html".to_string())
        .add_index_file("index.html")
        .with_etag()
        .set_path_not_to_cache("/")
        .enable_files_caching();

    for no_cache in get_disable_cache_list().await {
        static_files_middleware = static_files_middleware.set_path_not_to_cache(no_cache);
    }

    http_server.add_middleware(Arc::new(static_files_middleware));

    http_server.start(
        crate::app::APP_CTX.app_states.clone(),
        my_logger::LOGGER.clone(),
    );
}

async fn get_disable_cache_list() -> Vec<String> {
    let disabled_cache = tokio::fs::read_to_string("./www-system").await;

    let result = match disabled_cache {
        Ok(value) => value,
        Err(_) => {
            println!("Can not find file ./www-system. No Disabled cache list is used");
            return vec![];
        }
    };

    let result: Vec<String> = result.split('\n').map(|itm| itm.to_string()).collect();

    println!("Loaded {:?} no-cache paths", result);

    result
}
