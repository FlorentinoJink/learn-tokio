use tokio::fs::File;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader: &[u8] = b"gakki";
    let mut file = File::create("foo2.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    Ok(())
}