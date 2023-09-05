use anyhow::{Context, Result};
use env_logger as logger;
use log::{error, info};
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener};

fn main() -> Result<()> {
    logger::init();

    let listener = TcpListener::bind("127.0.0.1:10334", false).context("failed to bind")?;
    loop {
        let (mut stream, client_addr) = listener
            .accept(false)
            .context("failed to establish connection")?;

        info!("accepted connection of client {} ", client_addr.ip());

        let mut buf = [0u8; 1024];
        loop {
            let Ok(n) = stream.read(&mut buf) else {
                error!("failed to read stream of {}", client_addr);
                break;
            };

            if n == 0 {
                break;
            }

            let message = &buf[..n];
            info!(
                r#"received "{}" from {}"#,
                String::from_utf8(Vec::from(message))
                    .unwrap_or("<<not utf8>>".into())
                    .trim(),
                client_addr,
            );
            stream.write(message).context("failed to send to client")?;
        }

        info!("disconnected client {}", client_addr.ip());
        stream
            .shutdown(Shutdown::Both)
            .context("failed to shutdown stream")?;
    }

    // unreachable
    // anyhow::Ok(())
}
