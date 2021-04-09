mod server;
mod config;
mod filter;

#[tokio::main]
async fn main() {
    let _config = config::parse_config();
    server::run_server(_config).await.expect("Unable to run server");
}
