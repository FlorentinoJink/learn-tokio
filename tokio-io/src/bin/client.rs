use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    let message = b"hello world!";
    stream.write_all(message).await?;

    let mut response = vec![0; message.len()];
    stream.read_exact(&mut response).await?;

    assert_eq!(&response, message);

    println!("Message from server: {:?}", String::from_utf8(response));

    Ok(())
}
