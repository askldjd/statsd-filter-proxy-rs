mod config;
mod filter;
mod server;

use std::env;
use std::path::Path;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config_path_env = env::var("PROXY_CONFIG_FILE")
        .expect("PROXY_CONFIG_FILE must be set");

    let path = Path::new(&config_path_env);
    let _config = config::parse(&path);
    server::run_server(_config)
        .await
        .expect("Unable to run server");
}
