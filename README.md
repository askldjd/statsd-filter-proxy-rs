![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)
![Continuous integration](https://github.com/askldjd/statsd-filter-proxy-rs/workflows/CI/badge.svg)

# statsd-filter-proxy-rs

statsd-filter-proxy-rs is efficient and lightweight StatsD proxy to filter unwanted metrics to StatsD server.

## Why

"If you don't want metrics, why not stop sending the metrics?" you might ask. Sometimes disabling metrics isn't trivial because of scale, legacy code and time constraints. Sometimes the fastest way to disable a large number of metrics is to deploy a proxy to block unwanted metrics.

## Getting started

To build the proxy, you need
 - The rust toolset
    - Rust 1.51+
    - Cargo
 - You can also get it from [rustup](https://rustup.rs/)

## Configuration

statsd-filter-proxy-rs takes in a JSON file as the configuration file. 

```hjson
{
    // The host to bind to
    "listen_host": "0.0.0.0",
    
    // The UDP port to listen on for datagrams
    "listen_port": 8125,

    // The target StatsD server address to forward to
    "target_host": "0.0.0.0",
    
    // The target StatsD server port to forward to
    "target_port": 8125,

    // The list of metrics prefix to block
    "metric_blocklist": [
        "prefix1",
        "prefix2",
        "prefix3"
    ]
}
```