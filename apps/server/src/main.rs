use std::str::from_utf8;

use anyhow::Result;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup tracing subscriber.
    tracing_subscriber::fmt::init();

    // Display some basic information about the running process, useful for debugging with users.
    tracing::info!(
        "Starting {} v{} on {}:{} Arch: {}",
        option_env!("CARGO_PKG_NAME").unwrap_or("Unknown Package"),
        option_env!("CARGO_PKG_VERSION").unwrap_or("X.X.X"),
        std::env::consts::FAMILY,
        std::env::consts::OS,
        std::env::consts::ARCH,
    );

    start_server();
    Ok(())
}

fn start_server() {
    // Build Runtimes
    let acceptor_runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .name("acceptor_runtime")
        .thread_name("acceptor-pool")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();

    let request_runtime = Builder::new_multi_thread()
        .worker_threads(2)
        .name("request_runtime")
        .thread_name("request-pool")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .on_thread_start(|| tracing::debug!("Request thread started"))
        .on_thread_stop(|| tracing::debug!("Request thread stopped."))
        .build()
        .unwrap();

    acceptor_runtime.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:9990").await.unwrap();
        tracing::info!("Acceptor listening on 127.0.0.1:9990");

        loop {
            // Listen for new connections in the acceptor thread.
            let (socket, _) = listener.accept().await.unwrap();
            tracing::info!(
                "Accepted connection from {}, passing to request_runtime.",
                socket.peer_addr().unwrap()
            );

            // Switch the runtime to hand off our connection to the request thread.
            let _g = request_runtime.enter();
            request_runtime.spawn(stream_handler(socket));
        }
    })
}

async fn stream_handler(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    tracing::debug!("stream_handler started for {}", stream.peer_addr().unwrap());

    loop {
        stream.readable().await.unwrap();
        let n = match stream.read(&mut buf).await {
            // Scoket Closed
            Ok(0) => return,
            Ok(n) => n,
            Err(error) => {
                tracing::error!("failed to read from socket; err = {:?}", error);
                return;
            }
        };

        let answer = from_utf8(&buf[0..n]).expect("some utf8 issue");
        stream.writable().await.unwrap();
        let result = stream.try_write(format!("Back to you! {}", answer).as_bytes());
        match result {
            Ok(amount) => tracing::info!("Wrote {} bytes.", amount),
            Err(error) => tracing::error!("Got Write Error {:?}", error),
        }
    }
}
