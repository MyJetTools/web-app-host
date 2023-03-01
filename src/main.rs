use std::sync::Arc;

use rust_extensions::AppStates;

mod http;

#[tokio::main]
async fn main() {
    let app_states = Arc::new(AppStates::create_initialized());
    crate::http::start_up::setup_server(app_states.clone());
    app_states.wait_until_shutdown().await;
}
