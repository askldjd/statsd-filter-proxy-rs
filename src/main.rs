mod config;
mod filter;
mod server;

use std::path::Path;

#[tokio::main]
async fn main() {
    let path =
        Path::new("/home/askldjd/work/tenable/test/statsd-filter-proxy-rs/config/reference.json");
    let _config = config::parse(&path);
    server::run_server(_config)
        .await
        .expect("Unable to run server");
}
