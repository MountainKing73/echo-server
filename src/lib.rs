use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn run(addr: String) -> std::io::Result<()> {
    println!("Listening on address {}", addr);
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(&mut socket).await {
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }
}

// This is generic to handle testing
async fn handle_connection<T>(stream: &mut T) -> std::io::Result<()>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    println!("Starting connection");
    let mut buf = [0; 1028];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        stream.write_all(&buf[0..n]).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::duplex;

    #[tokio::test]
    async fn test_handle_connection() {
        let (mut client, mut server) = duplex(64);

        // spawn server
        let server_task = tokio::spawn(async move {
            handle_connection(&mut server).await.unwrap();
        });

        client.write_all(b"hello").await.unwrap();
        client.shutdown().await.unwrap();

        let mut response = vec![0u8; 5];
        client.read_exact(&mut response).await.unwrap();
        assert_eq!(&response, b"hello");

        server_task.await.unwrap();
    }
}
