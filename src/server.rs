use std::{io, sync::Arc};
use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};

use crate::config::Config;
use crate::filter::should_be_blocked;

pub async fn run_server(config: Config) -> io::Result<()> {
    let sock = UdpSocket::bind(format!("{}:{}", config.listen_host, config.listen_port)).await?;
    println!("Listening on: {}", sock.local_addr()?);

    let r = Arc::new(sock);
    let blacklist = Arc::new(config.metric_blocklist);

    let mut buf = [0; 8192];
    let multi_thread = match config.multi_thread {
        Some(p) => p,
        None => false,
    };

    println!("multi_thread = {}", multi_thread);

    loop {
        let (len, addr) = r.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?} onto {:p}", len, addr, &buf);

        if multi_thread {
            let s = r.clone();
            let shared_blacklist = blacklist.clone();
            tokio::spawn(async move {
                sleep(Duration::from_millis(2000)).await;

                if should_be_blocked(&shared_blacklist, &buf) == false {
                    println!(
                        "{:?} at {:p}",
                        std::str::from_utf8(&buf[..len]).unwrap(),
                        &buf
                    );

                    let len = s.send_to(&buf[..len], &addr).await.unwrap();
                    println!(
                        "Thread {}, Echoed {} bytes to {}",
                        thread_id::get(),
                        len,
                        addr
                    );
                }
            });
        } else {
            if should_be_blocked(&blacklist, &buf) == false {
                println!(
                    "{:?} at {:p}",
                    std::str::from_utf8(&buf[..len]).unwrap(),
                    &buf
                );

                let len = r.send_to(&buf[..len], &addr).await.unwrap();
                println!(
                    "Thread {}, Echoed {} bytes to {}",
                    thread_id::get(),
                    len,
                    addr
                );
            }
        }
    }
}
