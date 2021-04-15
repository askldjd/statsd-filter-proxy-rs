![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)
![Continuous integration](https://github.com/askldjd/statsd-filter-proxy-rs/workflows/CI/badge.svg)
![Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/askldjd/statsd-filter-proxy-rs)

# statsd-filter-proxy-rs

statsd-filter-proxy-rs is efficient and lightweight StatsD proxy that filters out unwanted metrics to a StatsD server.

## Why

"If you don't want metrics, why not just stop sending them?" you might ask. Sometimes disabling metrics isn't trivial because of scale, legacy code and time constraints. Sometimes the fastest way to disable a large number of metrics is to deploy a proxy to block them.

## Getting started

To build the proxy, you need
 - The rust toolset
    - Rust 1.51+
    - Cargo
 - You can also get them from [rustup](https://rustup.rs/)

## To Run

```
export PROXY_CONFIG_FILE=/path/to/your/proxy-config-file.json
RUST_LOG=debug 
cargo run --release
```

`PROXY_CONFIG_FILE` is a _required_ variable that points to the configuration file path.

`RUST_LOG` is an _optional_ variable that defines the log level. They can be `error`, `warn`, `info`, `debug` or `trace`.

## Docker

Make a JSON configuration file locally. This sample configuration below would make the filter proxy listen on port 8125, and forward datagrams to port 8127.
```json
{
    "listen_host": "0.0.0.0",
    "listen_port": 8125,
    "target_host": "127.0.0.1",
    "target_port": 8127,
    "metric_blocklist": [
        "foo1",
        "foo2",
    ]
}
```

Now run the proxy with the configuration mounted through Docker volume.
```bash
docker run -it \
    --volume $(pwd)/config.json:/app/config.json:Z \
    -e PROXY_CONFIG_FILE=/app/config.json \
    -e RUST_LOG=trace \
    -p 8125:8125/udp \
    askldjd/statsd-filter-proxy-rs:latest
```


## Configuration

statsd-filter-proxy-rs takes in a JSON file as the configuration file. 

```yaml
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

    // Set to true to delegate the send path to the tokio threadpool.
    // If you turn this on, filtering and the sending of the datagram may
    // be performed in Tokio background threads.
    //
    // Pros:
    // - scalable to more than 1 CPU, especially useful if your filter list 
    //   large enough to become a bottleneck.
    // Cons:
    // - slightly more overhead performed per message
    //   - an extra deep copy of the send buffer
    //   - Arc increments for sharing objects among threads
    //
    // - message sent might not be the same order they are received, since
    //   send path is concurrent
    "multi_thread": true | false (optional, default=false)
}
```

## Tests

### Unit Test

```
cargo run test
```

### Integration Test

```
python test/integration_test.py
```

## Benchmark

statsd-filter-proxy was [originally written](./benchmark/statsd-filter-proxy.js) in Node.js. So benchmark will use the original version as a baseline.

| packet latency | JS  | Rust (single-threaded) | RS (multi-threaded) |
|----------------|-----|------------------------|---------------------|
| Median(us)     | 639 | 399                    | 499                 |
| P95(us)        | 853 | 434                    | 547                 |

The latency number should not be taken in absolute form because it doesn not account for benchmark overhead (in Python).

CPU = Intel i7-8700K (12) @ 4.700GHz

## Limitations / Known Issues
- StatsD datagram are capped at 8192 bytes. This can be only be adjusted in code at the moment.