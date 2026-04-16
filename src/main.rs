use std::net::SocketAddr;

mod app;
mod http;

#[tokio::main]
async fn main() {

  
   

    #[cfg(unix)]
    let unix_socket_addr = std::env::var("UNIX_SOCKET").ok();

     crate::http::start_up::setup_server(SocketAddr::new([0, 0, 0, 0].into(), 8000), #[cfg(unix)] unix_socket_addr).await;

    crate::app::APP_CTX.app_states.wait_until_shutdown().await;
}
