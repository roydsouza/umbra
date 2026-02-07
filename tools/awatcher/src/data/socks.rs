use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

pub async fn test_socks_proxy() -> bool {
    timeout(Duration::from_millis(500), TcpStream::connect("127.0.0.1:9050"))
        .await
        .map(|r| r.is_ok())
        .unwrap_or(false)
}
