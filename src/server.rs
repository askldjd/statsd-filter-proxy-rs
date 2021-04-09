
use std::{env, io, sync::Arc};
use tokio::time::{sleep, Duration};
use tokio::{net::UdpSocket};

use crate::filter::should_be_blocked;
use crate::config::Config;

pub async fn run_server(config: Config) -> io::Result<()> {
    let sock = UdpSocket::bind(
        format!("{}:{}", config.listenHost, config.listenPort)
    ).await?;
    println!("Listening on: {}", sock.local_addr()?);

    let r = Arc::new(sock);
    let blacklist = Arc::new(config.metricBlacklist);

    let mut buf = [0; 4096];
    loop {
        let (len, addr) = r.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?} onto {:p}", len, addr, &buf);

        let s = r.clone();
        let shared_blacklist = blacklist.clone();

        tokio::spawn(async move {
            sleep(Duration::from_millis(2000)).await;
            should_be_blocked(&shared_blacklist, &buf);
            
            println!("{:?} at {:p}", std::str::from_utf8(&buf[..len]).unwrap(), &buf);

            let len = s.send_to(&buf[..len], &addr).await.unwrap();
            println!(
                "Thread {}, Echoed {} bytes to {}",
                thread_id::get(),
                len,
                addr
            );
        });
    }
}
