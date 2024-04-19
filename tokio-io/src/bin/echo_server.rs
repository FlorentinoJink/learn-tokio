use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match stream.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if stream.write_all(&buf[..n]).await.is_err() {
                            eprintln!("reply to cleint failed");
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });
    }
}
