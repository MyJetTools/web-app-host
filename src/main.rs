use std::{net::SocketAddr, sync::Arc};

use rust_extensions::AppStates;

mod http;

#[tokio::main]
async fn main() {
    let app_states = Arc::new(AppStates::create_initialized());
    crate::http::start_up::setup_server(
        app_states.clone(),
        SocketAddr::new([0, 0, 0, 0].into(), 8000),
    );

    #[cfg(unix)]
    if let Ok(unix_socket_addr) = std::env::var("UNIX_SOCKET") {
        crate::http::start_up::setup_server(app_states.clone(), unix_socket_addr);
    }

    app_states.wait_until_shutdown().await;
}
