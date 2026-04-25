use std::{net::SocketAddr, sync::Arc};

use is_alive_middleware::IsAliveMiddleware;
use my_http_server::MyHttpServer;

pub async fn setup_server(
    tcp_listen_addr: SocketAddr,
    #[cfg(unix)] unix_socket_path: Option<String>,
) {
    let mut http_server = MyHttpServer::new(tcp_listen_addr);

    #[cfg(unix)]
    let mut unix_server = unix_socket_path.map(MyHttpServer::new_as_unix_socket);

    let is_alive_middleware = Arc::new(IsAliveMiddleware::new(
        crate::app::APP_CTX.app_name.to_string(),
        crate::app::APP_CTX.app_version.to_string(),
    ));

    #[cfg(unix)]
    if let Some(unix_server) = unix_server.as_mut() {
        unix_server.add_middleware(is_alive_middleware.clone());
    }

    http_server.add_middleware(is_alive_middleware);

    let version_middleware = Arc::new(super::InjectVersionMiddleware);

    #[cfg(unix)]
    if let Some(unix_server) = unix_server.as_mut() {
        unix_server.add_middleware(version_middleware.clone());
    }

    http_server.add_middleware(version_middleware);

    let mut static_files_middleware = my_http_server::StaticFilesMiddleware::new()
        .set_not_found_file("index.html".to_string())
        .add_index_file("index.html")
        .with_etag()
        .set_path_not_to_cache("/")
        .enable_files_caching();

    for no_cache in get_disable_cache_list().await {
        static_files_middleware = static_files_middleware.set_path_not_to_cache(no_cache);
    }


    let static_middleware = Arc::new(static_files_middleware);

    #[cfg(unix)]
    if let Some(unix_server) = unix_server.as_mut() {
        unix_server.add_middleware(static_middleware.clone());
    }


    http_server.add_middleware(static_middleware);


    http_server.start_auto(
        crate::app::APP_CTX.app_states.clone(),
        my_logger::LOGGER.clone(),
    );


    #[cfg(unix)]
    if let Some(unix_server) = unix_server.as_mut() {
        unix_server.start_auto(
            crate::app::APP_CTX.app_states.clone(),
            my_logger::LOGGER.clone(),
        );
    }
}

async fn get_disable_cache_list() -> Vec<String> {
    const FILE_NAME: &'static str = "./www-system/.disable-cache";
    let disabled_cache = tokio::fs::read_to_string(FILE_NAME).await;

    let result = match disabled_cache {
        Ok(value) => value,
        Err(_) => {
            println!("Can not find file '{FILE_NAME}'. No Disabled cache list is used");
            return vec![];
        }
    };

    let result: Vec<String> = result
        .split('\n')
        .map(|itm| itm.trim())
        .filter(|itm| itm.len() > 0)
        .map(|itm| itm.to_string())
        .collect();

    println!("Loaded {:?} no-cache paths", result);

    result
}
