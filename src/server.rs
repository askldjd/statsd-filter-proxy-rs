use std::net::SocketAddr;
use std::{io, sync::Arc};
use tokio::net::UdpSocket;

use crate::config::Config;
use crate::filter::filter;

use log::{debug, info, log_enabled, trace, Level};

pub async fn run_server(config: Config) -> io::Result<()> {
    let sock = UdpSocket::bind(format!("{}:{}", config.listen_host, config.listen_port)).await?;
    info!("Listening on: {}", sock.local_addr()?);

    let sock = Arc::new(sock);
    let blocklist = Arc::new(config.metric_blocklist);

    let mut buf = [0; 8192];
    let multi_thread = config.multi_thread.unwrap_or(false);

    if multi_thread {
        trace!("multi_thread is enabled");
    }

    let target_addr: SocketAddr = format!("{}:{}", config.target_host, config.target_port)
        .parse()
        .expect("Unable to parse socket address");

    let target_addr = Arc::new(target_addr);

    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        debug!("{:?} bytes received from {:?} onto {:p}", len, addr, &buf);

        if log_enabled!(Level::Trace) {
            trace!(
                "{:?} at {:p}",
                std::str::from_utf8(&buf[..len]).unwrap(),
                &buf
            );
        }

        if multi_thread {
            let sock_clone = sock.clone();
            let target_addr_clone = target_addr.clone();
            let blocklist_clone = blocklist.clone();
            tokio::spawn(async move {
                let filtered_string = filter(&blocklist_clone, &buf[..len]);
                if filtered_string.len() > 0 {
                    let len = sock_clone
                        .send_to(filtered_string.as_bytes(), &*target_addr_clone)
                        .await
                        .unwrap();

                    debug!(
                        "Thread {}, Echoed {} bytes to {}",
                        thread_id::get(),
                        len,
                        target_addr_clone
                    );
                }
            });
        } else {
            let filtered_string = filter(&blocklist, &buf[..len]);
            if filtered_string.len() > 0 {
                let len = sock
                    .send_to(filtered_string.as_bytes(), &*target_addr)
                    .await
                    .unwrap();
                debug!(
                    "Thread {}, Echoed {} bytes to {}",
                    thread_id::get(),
                    len,
                    target_addr
                );
            }
        }
    }
}
