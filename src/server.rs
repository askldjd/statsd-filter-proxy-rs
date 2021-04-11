use std::net::SocketAddr;
use std::{io, sync::Arc};
use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};

use crate::config::Config;
use crate::filter::should_be_blocked;

use log::{info, trace};

pub async fn run_server(config: Config) -> io::Result<()> {
    let sock = UdpSocket::bind(format!("{}:{}", config.listen_host, config.listen_port)).await?;
    info!("Listening on: {}", sock.local_addr()?);

    let sock = Arc::new(sock);
    let blocklist = Arc::new(config.metric_blocklist);

    let mut buf = [0; 8192];
    let multi_thread = match config.multi_thread {
        Some(p) => p,
        None => false,
    };

    if multi_thread {
        trace!("multi_thread is enabled");
    }

    let target_addr: SocketAddr = format!("{}:{}", config.target_host, config.target_port)
        .parse()
        .expect("Unable to parse socket address");

    let target_addr = Arc::new(target_addr);

    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        trace!("{:?} bytes received from {:?} onto {:p}", len, addr, &buf);

        if multi_thread {
            let sock_clone = sock.clone();
            let target_addr_clone = target_addr.clone();
            let blocklist_clone = blocklist.clone();
            tokio::spawn(async move {
                sleep(Duration::from_millis(2000)).await;

                if should_be_blocked(&blocklist_clone, &buf) == false {
                    trace!(
                        "{:?} at {:p}",
                        std::str::from_utf8(&buf[..len]).unwrap(),
                        &buf
                    );

                    let len = sock_clone
                        .send_to(&buf[..len], &*target_addr_clone)
                        .await
                        .unwrap();

                    trace!(
                        "Thread {}, Echoed {} bytes to {}",
                        thread_id::get(),
                        len,
                        target_addr_clone
                    );
                }
            });
        } else {
            if should_be_blocked(&blocklist, &buf) == false {
                trace!(
                    "{:?} at {:p}",
                    std::str::from_utf8(&buf[..len]).unwrap(),
                    &buf
                );

                let len = sock.send_to(&buf[..len], &*target_addr).await.unwrap();
                trace!(
                    "Thread {}, Echoed {} bytes to {}",
                    thread_id::get(),
                    len,
                    target_addr
                );
            }
        }
    }
}
